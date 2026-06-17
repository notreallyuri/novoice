use entity::relationship::{self, DbRelationshipStatus};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, Set};
use shared::data::UserId;

use crate::core::{cache::friends::cache_remove_friend, error::AppError, state::SharedState};

pub async fn block(
    state: &SharedState,
    user_id: UserId,
    target_id: UserId,
) -> Result<(), AppError> {
    if user_id == target_id {
        return Err(AppError::BadRequest("You cannot block yourself".into()));
    }

    let now = chrono::Utc::now().naive_utc();

    let target_rel = relationship::Entity::find()
        .filter(relationship::Column::UserId.eq(target_id.0))
        .filter(relationship::Column::TargetId.eq(user_id.0))
        .one(&state.db)
        .await?;

    if let Some(rel) = target_rel
        && rel.status != DbRelationshipStatus::Blocked
    {
        rel.delete(&state.db).await?;
    }

    let user_rel = relationship::Entity::find()
        .filter(relationship::Column::UserId.eq(user_id.0))
        .filter(relationship::Column::TargetId.eq(target_id.0))
        .one(&state.db)
        .await?;

    match user_rel {
        Some(rel) => {
            if rel.status != DbRelationshipStatus::Blocked {
                let mut active_rel: relationship::ActiveModel = rel.into();
                active_rel.status = Set(DbRelationshipStatus::Blocked);
                active_rel.since = Set(now);
                active_rel.update(&state.db).await?;
            }
        }
        None => {
            let new_rel = relationship::ActiveModel {
                user_id: Set(user_id.0),
                target_id: Set(target_id.0),
                status: Set(DbRelationshipStatus::Blocked),
                since: Set(now),
            };
            new_rel.insert(&state.db).await?;
        }
    }

    cache_remove_friend(state, user_id, target_id).await;

    Ok(())
}
