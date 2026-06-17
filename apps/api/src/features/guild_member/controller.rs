use super::services::*;
use crate::core::{
    error::AppError, extractor::AuthContext, response::ApiResponse, state::SharedState,
};
use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use shared::{
    data::{GuildId, UserId},
    dtos::guild_member::BanMemberRequest,
};

pub async fn leave(
    State(state): State<SharedState>,
    Path(guild_id): Path<GuildId>,
    auth: AuthContext,
) -> Result<impl IntoResponse, AppError> {
    leave::leave(&state, auth.user_id, guild_id).await?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn kick(
    State(state): State<SharedState>,
    Path((guild_id, target_id)): Path<(GuildId, UserId)>,
    auth: AuthContext,
) -> Result<impl IntoResponse, AppError> {
    kick::kick(&state, auth.user_id, guild_id, target_id).await?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn ban(
    State(state): State<SharedState>,
    Path((guild_id, target_id)): Path<(GuildId, UserId)>,
    auth: AuthContext,
    Json(req): Json<BanMemberRequest>,
) -> Result<impl IntoResponse, AppError> {
    ban::ban(&state, auth.user_id, guild_id, target_id, req).await?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn unban(
    State(state): State<SharedState>,
    Path((guild_id, target_id)): Path<(GuildId, UserId)>,
    auth: AuthContext,
) -> Result<impl IntoResponse, AppError> {
    unban::unban(&state, auth.user_id, guild_id, target_id).await?;

    Ok(Json(ApiResponse::success(())))
}
