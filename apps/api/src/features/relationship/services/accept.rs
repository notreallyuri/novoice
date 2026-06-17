use crate::core::{cache::friends::cache_add_friend, error::AppError, state::SharedState};
use entity::relationship::{self, DbRelationshipStatus};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use shared::data::UserId;

pub async fn accept_request(
    state: &SharedState,
    user_id: UserId,
    target_id: UserId,
) -> Result<(), AppError> {
    let rel = relationship::Entity::find()
        .filter(relationship::Column::UserId.eq(target_id.0))
        .filter(relationship::Column::TargetId.eq(user_id.0))
        .filter(relationship::Column::Status.eq(DbRelationshipStatus::PendingOutgoing))
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    let mut active_rel: relationship::ActiveModel = rel.into();
    active_rel.status = Set(DbRelationshipStatus::Friend);
    active_rel.since = Set(chrono::Utc::now().naive_utc());
    active_rel.update(&state.db).await?;

    cache_add_friend(state, user_id, target_id).await;

    Ok(())
}
