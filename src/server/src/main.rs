mod modules;
mod router;
mod services;

use anyhow::Result;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::router::make_router;

#[tokio::main]
async fn main() -> Result<()> {
    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter_layer)
        .init();

    let router = make_router().await?;
    let listener = TcpListener::bind("0.0.0.0:7878").await?;

    tracing::info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, router).await?;

    Ok(())
}
