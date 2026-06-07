use crate::{
    core::{error::AppError, extractor::AuthContext, response::ApiResponse, state::SharedState},
    features::guild::services,
};
use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use shared::{
    data::GuildId,
    dtos::guild::{CreateGuildRequest, CreateInviteRequest},
};

pub async fn create(
    State(state): State<SharedState>,
    auth: AuthContext,
    Json(payload): Json<CreateGuildRequest>,
) -> Result<impl IntoResponse, AppError> {
    let guild = services::create::create(&state, auth.user_id, payload).await?;

    Ok(Json(ApiResponse::success(guild)))
}

pub async fn create_invite(
    State(state): State<SharedState>,
    Path(guild_id): Path<GuildId>,
    auth: AuthContext,
    Json(payload): Json<CreateInviteRequest>,
) -> Result<impl IntoResponse, AppError> {
    let data =
        services::create_invite::create_invite(&state, auth.user_id, guild_id, payload).await?;

    Ok(Json(ApiResponse::success(data)))
}

pub async fn delete(
    State(state): State<SharedState>,
    Path(guild_id): Path<GuildId>,
    auth: AuthContext,
) -> Result<impl IntoResponse, AppError> {
    services::delete::delete(&state, guild_id, auth.user_id).await?;

    Ok(Json(ApiResponse::success(())))
}
