use crate::core::{
    error::AppError, extractor::AuthContext, response::ApiResponse, state::SharedState,
};
use axum::{Json, extract::State, response::IntoResponse};
use shared::dtos::dm::CreateDmRequest;

pub async fn create_or_get_dm(
    State(state): State<SharedState>,
    auth: AuthContext,
    Json(payload): Json<CreateDmRequest>,
) -> Result<impl IntoResponse, AppError> {
    let channel = super::services::create::create_or_get_dm(&state, auth.user_id, payload).await?;

    Ok(Json(ApiResponse::success(channel)))
}

pub async fn get_open_dms(
    State(state): State<SharedState>,
    auth: AuthContext,
) -> Result<impl IntoResponse, AppError> {
    let channels = super::services::get::get_open_dms(&state, auth.user_id).await?;

    Ok(Json(ApiResponse::success(channels)))
}
