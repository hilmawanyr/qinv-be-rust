use std::sync::Arc;

use axum::{Router, routing::post};

use super::auth;
use crate::modules::auth::port::inbound::AuthService;

pub fn auth_router(service: Arc<dyn AuthService>) -> Router {
    Router::new()
        .route("/sign-in", post(auth::sign_in))
        .with_state(service)
}
