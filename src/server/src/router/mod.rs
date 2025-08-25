mod api;

use std::sync::Arc;

use anyhow::Result;
use axum::http::Method;
use axum::{Extension, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::services::Services;

pub async fn make_router(services: Arc<Services>) -> Result<Router> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    let router = axum::Router::new()
        .nest("/api", api::make_api_router())
        .layer(Extension(services))
        .layer(cors);

    Ok(router)
}
