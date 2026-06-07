use crate::core::{
    error::AppError, extractor::AuthContext, response::ApiResponse, state::SharedState,
};
use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use shared::{
    data::{ChannelId, MessageId},
    dtos::message::{
        MessageEditRequest, MessagePinRequest, MessageQueryParams, MessageSendRequest,
    },
};

pub async fn send(
    State(state): State<SharedState>,
    Path(channel_id): Path<ChannelId>,
    auth: AuthContext,
    Json(body): Json<MessageSendRequest>,
) -> Result<impl IntoResponse, AppError> {
    super::services::send::send(&state, channel_id, auth.user_id, body).await?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn get(
    State(state): State<SharedState>,
    Path(channel_id): Path<ChannelId>,
    Query(query): Query<MessageQueryParams>,
) -> Result<impl IntoResponse, AppError> {
    let data = super::services::get::get_messages(&state, channel_id, query).await?;

    Ok(Json(ApiResponse::paginated(data)))
}

pub async fn edit(
    State(state): State<SharedState>,
    Path(channel_id): Path<ChannelId>,
    Path(id): Path<MessageId>,
    auth: AuthContext,
    Json(body): Json<MessageEditRequest>,
) -> Result<impl IntoResponse, AppError> {
    super::services::edit::edit(&state, channel_id, id, body.content, auth.user_id).await?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn delete(
    State(state): State<SharedState>,
    Path(channel_id): Path<ChannelId>,
    Path(id): Path<MessageId>,
    auth: AuthContext,
) -> Result<impl IntoResponse, AppError> {
    super::services::delete::delete(&state, channel_id, id, auth.user_id).await?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn pin(
    State(state): State<SharedState>,
    Path(channel_id): Path<ChannelId>,
    Path(id): Path<MessageId>,
    auth: AuthContext,
    Json(body): Json<MessagePinRequest>,
) -> Result<impl IntoResponse, AppError> {
    super::services::pin::pin(&state, channel_id, id, body.label, auth.user_id).await?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn unpin(
    State(state): State<SharedState>,
    Path(channel_id): Path<ChannelId>,
    Path(id): Path<MessageId>,
) -> Result<impl IntoResponse, AppError> {
    super::services::pin::unpin(&state, channel_id, id).await?;

    Ok(Json(ApiResponse::success(())))
}
