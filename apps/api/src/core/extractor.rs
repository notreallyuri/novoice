use crate::core::{
    error::AppError,
    sessions,
    state::{SessionId, SharedState},
};
use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use shared::data::UserId;
use std::str::FromStr;
use uuid::Uuid;

pub struct AuthContext {
    pub user_id: UserId,
    pub session_id: SessionId,
}

impl FromRequestParts<SharedState> for AuthContext {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &SharedState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|val| val.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Unauthorized);
        }

        let token_str = &auth_header["Bearer ".len()..];

        let session_uuid = Uuid::from_str(token_str).map_err(|_| AppError::Unauthorized)?;
        let session_id = SessionId(session_uuid);

        let user_id_str = sessions::verify_and_extend(&state.redis.sessions, token_str).await?;

        let user_uuid = Uuid::from_str(&user_id_str).map_err(|_| AppError::Unauthorized)?;
        let user_id = UserId(user_uuid);

        Ok({
            AuthContext {
                user_id,
                session_id,
            }
        })
    }
}
