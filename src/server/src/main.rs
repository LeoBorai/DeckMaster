mod helpers;
mod modules;

use std::sync::Arc;

use anyhow::Result;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use deckmaster_domain::mtg::service::{FindDecksFilter, MtgService};

use self::modules::mtg::repository::MtgRepository;

#[tokio::main]
async fn main() -> Result<()> {
    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter_layer)
        .init();

    let mtg_repository = MtgRepository::new().await?;
    let mtg_repository = Arc::new(mtg_repository);
    let mtg_service = MtgService::new(mtg_repository).await?;

    let decks = mtg_service.get_decks(FindDecksFilter::default()).await?;

    println!("{decks:#?}");
    Ok(())
}
