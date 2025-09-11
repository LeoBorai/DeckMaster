use http::{Method, header};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

pub mod v0;

pub fn make_api_router() -> axum::Router {
    axum::Router::new()
        .nest("/v0", v0::routes())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(
            CorsLayer::new()
                .allow_headers([header::CONTENT_TYPE])
                .allow_methods([Method::GET, Method::OPTIONS])
                .allow_origin(Any),
        )
}
