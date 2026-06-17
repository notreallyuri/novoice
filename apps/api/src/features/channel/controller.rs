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
    data::{ChannelId, GuildId},
    dtos::channel::{CreateChannelRequest, UpdateChannelRequest},
};

pub async fn create(
    State(state): State<SharedState>,
    Path(guild_id): Path<GuildId>,
    auth: AuthContext,
    Json(req): Json<CreateChannelRequest>,
) -> Result<impl IntoResponse, AppError> {
    let res = create::create(&state, auth.user_id, guild_id, req).await?;

    Ok(Json(ApiResponse::success(res)))
}

pub async fn update(
    State(state): State<SharedState>,
    Path((guild_id, channel_id)): Path<(GuildId, ChannelId)>,
    auth: AuthContext,
    Json(req): Json<UpdateChannelRequest>,
) -> Result<impl IntoResponse, AppError> {
    let res = update::update(state, auth.user_id, guild_id, channel_id, req).await?;

    Ok(Json(ApiResponse::success(res)))
}

pub async fn delete(
    State(state): State<SharedState>,
    Path((guild_id, channel_id)): Path<(GuildId, ChannelId)>,
    auth: AuthContext,
) -> Result<impl IntoResponse, AppError> {
    delete::delete(&state, auth.user_id, guild_id, channel_id).await?;

    Ok(Json(ApiResponse::success(())))
}
