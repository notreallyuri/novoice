use crate::core::state::SharedState;
use axum::{Router, routing::post};

pub mod controller;
pub mod services;

pub fn router() -> Router<SharedState> {
    Router::new()
        .route(
            "/{target_id}/friend",
            post(controller::send_friend_request)
                .put(controller::accept_friend_request)
                .delete(controller::remove_friend),
        )
        .route(
            "/{target_id}/block",
            post(controller::block).delete(controller::unblock),
        )
}
