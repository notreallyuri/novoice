use crate::core::{crypto, error::AppError, sessions, state::SharedState};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use shared::dtos::auth::LoginRequest;

pub async fn login(state: SharedState, payload: LoginRequest) -> Result<String, AppError> {
    let user = entity::user_account::Entity::find()
        .filter(entity::user_account::Column::Email.eq(payload.email))
        .one(&state.db)
        .await?
        .ok_or(AppError::Unauthorized)?;

    let is_valid = crypto::verify_password(&payload.password, &user.password_hash)?;

    if !is_valid {
        return Err(AppError::Unauthorized);
    }

    let token = sessions::create(&state.redis.sessions, &user.user_id.to_string()).await?;

    Ok(token)
}
