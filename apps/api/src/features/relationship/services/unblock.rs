use entity::relationship::{self, DbRelationshipStatus};
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use shared::data::UserId;

use crate::core::{error::AppError, state::SharedState};

pub async fn unblock(
    state: &SharedState,
    user_id: UserId,
    target_id: UserId,
) -> Result<(), AppError> {
    if user_id == target_id {
        return Err(AppError::BadRequest(
            "You cannot even block yourself...".into(),
        ));
    }

    let user_rel = relationship::Entity::find()
        .filter(relationship::Column::UserId.eq(user_id.0))
        .filter(relationship::Column::TargetId.eq(target_id.0))
        .one(&state.db)
        .await?;

    match user_rel {
        Some(rel) => {
            if rel.status == DbRelationshipStatus::Blocked {
                rel.delete(&state.db).await?;
            }
        }
        None => {
            return Err(AppError::BadRequest("User not blocked.".into()));
        }
    }

    Ok(())
}
