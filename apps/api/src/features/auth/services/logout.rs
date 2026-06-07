use crate::core::{error::AppError, sessions, state::SharedState};

pub async fn logout(state: SharedState, session_token: &str) -> Result<(), AppError> {
    sessions::revoke(&state.redis.sessions, session_token).await?;

    Ok(())
}
