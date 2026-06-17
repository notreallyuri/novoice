use crate::core::{
    error::AppError, extractor::AuthContext, response::ApiResponse, state::SharedState,
};
use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use shared::{data::GuildId, dtos::user::JoinGuildRequest};

pub async fn get_me(
    State(state): State<SharedState>,
    auth: AuthContext,
) -> Result<impl IntoResponse, AppError> {
    let data = super::services::get::get_me(&state, auth.user_id).await?;

    Ok(Json(ApiResponse::success(data)))
}

pub async fn get_guild(
    State(state): State<SharedState>,
    Path(guild_id): Path<GuildId>,
    auth: AuthContext,
) -> Result<impl IntoResponse, AppError> {
    let data = super::services::get::get_guild(&state, guild_id, auth.user_id).await?;

    Ok(Json(ApiResponse::success(data)))
}

pub async fn join_guild(
    State(state): State<SharedState>,
    auth: AuthContext,
    Json(payload): Json<JoinGuildRequest>,
) -> Result<impl IntoResponse, AppError> {
    super::services::join_guild::join_guild(state, auth.user_id, payload).await?;

    Ok(Json(ApiResponse::success(())))
}
