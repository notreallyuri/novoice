use entity::{
    relationship::{self, DbRelationshipStatus},
    user,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
use shared::{
    data::{
        UserId,
        relationship::{RelationshipStatus, UserRelationship},
        user::UserPublic,
    },
    ws::{ServerMessage, user::UserServerEvents},
};

use crate::core::{broadcast, error::AppError, mappers::FromDomain, presence, state::SharedState};

pub async fn send_request(
    state: &SharedState,
    user_id: UserId,
    target_id: UserId,
) -> Result<(), AppError> {
    if user_id == target_id {
        return Err(AppError::BadRequest("You cannot add yourself".into()));
    }

    let existing = relationship::Entity::find()
        .filter(
            Condition::any()
                .add(
                    Condition::all()
                        .add(relationship::Column::UserId.eq(user_id.0))
                        .add(relationship::Column::TargetId.eq(target_id.0)),
                )
                .add(
                    Condition::all()
                        .add(relationship::Column::UserId.eq(target_id.0))
                        .add(relationship::Column::TargetId.eq(user_id.0)),
                ),
        )
        .one(&state.db)
        .await?;

    if existing.is_some() {
        return Err(AppError::Conflict("Relationship already exists".into()));
    }

    let now = chrono::Utc::now();
    let naive_now = now.naive_utc();

    let new_rel = relationship::ActiveModel {
        user_id: Set(user_id.0),
        target_id: Set(target_id.0),
        status: Set(DbRelationshipStatus::PendingOutgoing),
        since: Set(naive_now),
    };

    new_rel.insert(&state.db).await?;

    let sender_model = user::Entity::find_by_id(user_id.0)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    let sender_presence = presence::get_presence(&state.redis.presence, &user_id.0).await?;

    let update = UserRelationship {
        id: user_id,
        user: UserPublic {
            id: user_id,
            profile: FromDomain::from_domain(sender_model),
            presence: sender_presence,
        },
        status: RelationshipStatus::PendingIncoming,
        since: now.to_rfc3339(),
    };

    let event = ServerMessage::User(UserServerEvents::RelationshipUpdate {
        relationship: Box::new(update),
    });

    broadcast::to_user(&state.redis.messages, &target_id, &event).await?;

    Ok(())
}
