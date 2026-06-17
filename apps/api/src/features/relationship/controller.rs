use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use shared::data::UserId;

use crate::core::{
    error::AppError, extractor::AuthContext, response::ApiResponse, state::SharedState,
};

pub async fn send_friend_request(
    State(state): State<SharedState>,
    auth: AuthContext,
    Path(target_id): Path<UserId>,
) -> Result<impl IntoResponse, AppError> {
    super::services::send::send_request(&state, auth.user_id, target_id).await?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn accept_friend_request(
    State(state): State<SharedState>,
    auth: AuthContext,
    Path(target_id): Path<UserId>,
) -> Result<impl IntoResponse, AppError> {
    super::services::accept::accept_request(&state, auth.user_id, target_id).await?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn remove_friend(
    State(state): State<SharedState>,
    auth: AuthContext,
    Path(target_id): Path<UserId>,
) -> Result<impl IntoResponse, AppError> {
    super::services::remove::remove_relationship(&state, auth.user_id, target_id).await?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn block(
    State(state): State<SharedState>,
    auth: AuthContext,
    Path(target_id): Path<UserId>,
) -> Result<impl IntoResponse, AppError> {
    super::services::block::block(&state, auth.user_id, target_id).await?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn unblock(
    State(state): State<SharedState>,
    auth: AuthContext,
    Path(target_id): Path<UserId>,
) -> Result<impl IntoResponse, AppError> {
    super::services::unblock::unblock(&state, auth.user_id, target_id).await?;

    Ok(Json(ApiResponse::success(())))
}
