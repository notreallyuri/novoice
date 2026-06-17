use shared::data::UserId;
use uuid::Uuid;

use crate::core::{error::AppError, state::SharedState};

pub async fn generate_ticket(state: &SharedState, user_id: UserId) -> Result<String, AppError> {
    let ticket = Uuid::new_v4().to_string();

    let mut conn = state.redis.sessions.get().await?;

    let _: () = deadpool_redis::redis::cmd("SETEX")
        .arg(format!("session:{}", ticket))
        .arg(30)
        .arg(user_id.0.to_string())
        .query_async(&mut conn)
        .await?;

    Ok(ticket)
}
