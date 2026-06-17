use crate::core::state::SharedState;
use axum::{Router, routing::post};

pub mod controller;
pub mod services;

pub fn router() -> Router<SharedState> {
    Router::new().route(
        "/",
        post(controller::create_or_get_dm).get(controller::get_open_dms),
    )
}
