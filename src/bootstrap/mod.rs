use std::sync::Arc;

use axum::Router;
use sqlx::PgPool;

use crate::{
    infrastructure::{
        config::config::Config, http::router::create_router,
        security::internal_encrypt::InternalEncryptAdapter,
    },
    modules::{
        auth::{adapter::handler::router::auth_router, bootstrap::create_auth_service},
        shared::encrypt::EncryptPort,
    },
};

pub async fn create_app(pool: PgPool) -> Result<Router, Box<dyn std::error::Error>> {
    let cfg = Config::new()?;
    let enc: Arc<dyn EncryptPort> = Arc::new(InternalEncryptAdapter::new(
        cfg.internal_enc_duration,
        cfg.internal_enc_id,
        cfg.internal_enc_key,
    )?);

    let auth_route = auth_router(create_auth_service(pool, enc));

    let router = create_router(auth_route);

    Ok(router)
}
