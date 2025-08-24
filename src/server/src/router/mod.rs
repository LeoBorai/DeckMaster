mod api;

use std::sync::Arc;

use anyhow::Result;
use axum::{Extension, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::router::api::v0::ApiV0Doc;
use crate::services::Services;

pub async fn make_router() -> Result<Router> {
    let services = Services::new().await?;
    let services = Arc::new(services);
    let router = axum::Router::new()
        .nest("/api", api::make_api_router())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiV0Doc::openapi()))
        .layer(Extension(services));

    Ok(router)
}
