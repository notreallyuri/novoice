use crate::core::state::SharedState;
use axum::{
    Router,
    routing::{patch, post},
};

pub mod controller;
pub mod services;

pub fn router() -> Router<SharedState> {
    Router::new().route("/", post(controller::create)).route(
        "/{channel_id}",
        patch(controller::update).delete(controller::delete),
    )
}
