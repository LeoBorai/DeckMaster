use std::{env::var, sync::Arc};

use anyhow::{Context, Result};
use reqwest::Url;
use sqlx::SqlitePool;

use deckmaster_domain::mtg::service::MtgService;

use crate::modules::mtg::repository::MtgRepository;

pub type SharedServices = Arc<Services>;

pub struct Services {
    pub mtg: MtgService<MtgRepository>,
}

impl Services {
    pub async fn new() -> Result<Self> {
        let storage_url = var("STORAGE_URL").context("Missing STORAGE_URL")?;
        let db_pool = SqlitePool::connect("data/mtg.sqlite").await?;
        let db_pool = Arc::new(db_pool);
        let storage_url = Url::parse(&storage_url)?;
        let mtg_repository = MtgRepository::new(Arc::clone(&db_pool), storage_url).await?;
        let mtg_repository = Arc::new(mtg_repository);
        let mtg = MtgService::new(mtg_repository);

        Ok(Self { mtg })
    }
}
