use crate::core::{self, error::AppError, mappers::FromDomain, state::SharedState};
use entity::{dm_channel, dm_channel_member, user};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set, TransactionTrait,
};
use shared::{
    data::{
        ChannelId, UserId,
        channel::prelude::DMChannel,
        user::{Status, UserPresence, UserProfile, UserPublic},
    },
    dtos::dm::CreateDmRequest,
    ws::{ServerMessage, user::UserServerEvents},
};
use uuid::Uuid;

pub async fn create_or_get_dm(
    state: &SharedState,
    user_id: UserId,
    req: CreateDmRequest,
) -> Result<DMChannel, AppError> {
    let target_id = req.target_user_id;

    if user_id == target_id {
        return Err(AppError::BadRequest(
            "You cannot create a Direct Message Channel with yourself".into(),
        ));
    }

    let target_user_model = user::Entity::find_by_id(target_id.0)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    let my_memberships = dm_channel_member::Entity::find()
        .filter(dm_channel_member::Column::UserId.eq(user_id.0))
        .all(&state.db)
        .await?;

    let my_channel_ids: Vec<Uuid> = my_memberships.iter().map(|m| m.channel_id).collect();

    let existing_dm = dm_channel_member::Entity::find()
        .filter(dm_channel_member::Column::UserId.eq(target_id.0))
        .filter(dm_channel_member::Column::ChannelId.is_in(my_channel_ids))
        .one(&state.db)
        .await?;

    let channel_id = if let Some(target_membership) = existing_dm {
        let cid = target_membership.channel_id;

        let my_membership = my_memberships
            .into_iter()
            .find(|m| m.channel_id == cid)
            .unwrap();

        if !my_membership.is_open {
            let mut active_membership = my_membership.into_active_model();
            active_membership.is_open = Set(true);
            active_membership.update(&state.db).await?;
        }

        cid
    } else {
        let new_channel_id = Uuid::new_v4();
        let now = chrono::Utc::now().into();

        let new_channel = dm_channel::ActiveModel {
            id: Set(new_channel_id),
            created_at: Set(now),
        };

        let my_member = dm_channel_member::ActiveModel {
            channel_id: Set(new_channel_id),
            user_id: Set(user_id.0),
            is_open: Set(true),
            joined_at: Set(now),
        };

        let target_member = dm_channel_member::ActiveModel {
            channel_id: Set(new_channel_id),
            user_id: Set(target_id.0),
            is_open: Set(false),
            joined_at: Set(now),
        };

        let txn = state.db.begin().await?;
        new_channel.insert(&txn).await?;
        my_member.insert(&txn).await?;
        target_member.insert(&txn).await?;
        txn.commit().await?;

        new_channel_id
    };

    let target_presence = core::presence::get_presence(&state.redis.presence, &target_id.0)
        .await
        .unwrap_or(UserPresence {
            status: Status::Offline,
            preset: None,
        });

    let dm_dto = DMChannel {
        id: ChannelId(channel_id),
        recipients: vec![UserPublic {
            id: target_id,
            profile: UserProfile::from_domain(target_user_model),
            presence: target_presence,
        }],
        is_open: true,
    };

    let event = ServerMessage::User(UserServerEvents::DirectMessageCreated {
        channel: Box::new(dm_dto.clone()),
    });

    let _ = core::broadcast::to_user(&state.redis.messages, &user_id, &event).await;
    let _ = core::broadcast::to_user(&state.redis.messages, &target_id, &event).await;

    Ok(dm_dto)
}
