mod api;
mod assets;

use std::sync::Arc;

use anyhow::Result;
use axum::routing::get;
use axum::{Extension, Router};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::router::api::v0::ApiV0Doc;
use crate::router::assets::serve_asset;
use crate::services::Services;

pub async fn make_router() -> Result<Router> {
    let services = Services::new().await?;
    let services = Arc::new(services);
    let router = axum::Router::new()
        .nest("/api", api::make_api_router())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiV0Doc::openapi()))
        .fallback_service(get(serve_asset))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(Extension(services));

    Ok(router)
}
