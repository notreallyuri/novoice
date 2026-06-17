use axum::{
    Router,
    routing::{patch, post},
};

use crate::core::state::SharedState;

pub mod controller;
pub mod services;

pub fn router() -> Router<SharedState> {
    Router::new().route("/", post(controller::create)).route(
        "/{category_id}",
        patch(controller::update).delete(controller::delete),
    )
}
