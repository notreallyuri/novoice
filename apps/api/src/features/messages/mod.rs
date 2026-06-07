use crate::core::state::SharedState;
use axum::{
    Router,
    routing::{get, patch, post},
};

mod controller;
mod services;

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/", get(controller::get).post(controller::send))
        .route("/{id}", patch(controller::edit).delete(controller::delete))
        .route("/pin/{id}", post(controller::pin).delete(controller::unpin))
}
