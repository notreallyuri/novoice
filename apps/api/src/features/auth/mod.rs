use axum::{
    Router,
    routing::{delete, post},
};

mod controller;
mod services;

pub fn router() -> Router<crate::core::state::SharedState> {
    Router::new()
        .route("/register", post(controller::register))
        .route("/login", post(controller::login))
        .route("/logout", delete(controller::logout))
        .route("/ticket", post(controller::ticket))
}
