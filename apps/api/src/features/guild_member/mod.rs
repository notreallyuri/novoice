use crate::core::state::SharedState;
use axum::{
    Router,
    routing::{delete, post},
};

pub mod controller;
pub mod services;

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/@me", delete(controller::leave))
        .route("/{target_id}", delete(controller::kick))
        .route(
            "/{target_id}/ban",
            post(controller::ban).delete(controller::unban),
        )
}
