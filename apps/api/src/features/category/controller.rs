use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use shared::{
    data::{CategoryId, GuildId},
    dtos::category::{CreateCategoryRequest, UpdateCategoryRequest},
};

use crate::core::{
    error::AppError, extractor::AuthContext, response::ApiResponse, state::SharedState,
};

pub async fn create(
    State(state): State<SharedState>,
    Path(guild_id): Path<GuildId>,
    auth: AuthContext,
    Json(req): Json<CreateCategoryRequest>,
) -> Result<impl IntoResponse, AppError> {
    let res = super::services::create::create(&state, auth.user_id, guild_id, req).await?;

    Ok(Json(ApiResponse::success(res)))
}

pub async fn update(
    State(state): State<SharedState>,
    Path((guild_id, category_id)): Path<(GuildId, CategoryId)>,
    auth: AuthContext,
    Json(req): Json<UpdateCategoryRequest>,
) -> Result<impl IntoResponse, AppError> {
    let res =
        super::services::update::update(&state, auth.user_id, guild_id, category_id, req).await?;

    Ok(Json(ApiResponse::success(res)))
}

pub async fn delete(
    State(state): State<SharedState>,
    Path((guild_id, category_id)): Path<(GuildId, CategoryId)>,
    auth: AuthContext,
) -> Result<impl IntoResponse, AppError> {
    super::services::delete::delete(&state, auth.user_id, guild_id, category_id).await?;
    Ok(Json(ApiResponse::success(())))
}
