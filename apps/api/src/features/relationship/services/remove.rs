use crate::core::{cache::friends::cache_remove_friend, error::AppError, state::SharedState};
use entity::relationship;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use shared::data::UserId;

pub async fn remove_relationship(
    state: &SharedState,
    user_id: UserId,
    target_id: UserId,
) -> Result<(), AppError> {
    let rel = relationship::Entity::find()
        .filter(
            sea_orm::Condition::any()
                .add(
                    sea_orm::Condition::all()
                        .add(relationship::Column::UserId.eq(user_id.0))
                        .add(relationship::Column::TargetId.eq(target_id.0)),
                )
                .add(
                    sea_orm::Condition::all()
                        .add(relationship::Column::UserId.eq(target_id.0))
                        .add(relationship::Column::TargetId.eq(user_id.0)),
                ),
        )
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound)?;

    rel.delete(&state.db).await?;

    cache_remove_friend(state, user_id, target_id).await;

    Ok(())
}
