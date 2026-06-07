use crate::core::state::SharedState;
use axum::{
    Router,
    routing::{delete, post},
};

mod controller;
mod services;

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/", post(controller::create))
        .route("/{id}", delete(controller::delete))
        .route("/{id}/invite", post(controller::create_invite))
}
