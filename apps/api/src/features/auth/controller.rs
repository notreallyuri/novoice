use axum::{Json, extract::State, response::IntoResponse};
use shared::dtos::auth::{LoginRequest, RegisterRequest};

use crate::{
    core::{error::AppError, extractor::AuthContext, response::ApiResponse, state::SharedState},
    features::auth::services,
};

pub async fn register(
    State(state): State<SharedState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    let token = services::register::register(state, payload).await?;

    Ok(Json(ApiResponse::success(token)))
}

pub async fn login(
    State(state): State<SharedState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let token = services::login::login(state, payload).await?;

    Ok(Json(ApiResponse::success(token)))
}

pub async fn logout(
    State(state): State<SharedState>,
    auth: AuthContext,
) -> Result<impl IntoResponse, AppError> {
    let token = auth.session_id.0.to_string();

    services::logout::logout(state, &token).await?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn ticket(
    State(state): State<SharedState>,
    auth: AuthContext,
) -> Result<impl IntoResponse, AppError> {
    let ticket_token = services::ticket::generate_ticket(&state, auth.user_id).await?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "ticket": ticket_token
    }))))
}
