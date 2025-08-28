use std::sync::Arc;

use anyhow::Result;
use sqlx::SqlitePool;

use deckmaster_domain::mtg::service::MtgService;

use crate::modules::mtg::repository::MtgRepository;

pub type SharedServices = Arc<Services>;

pub struct Services {
    pub mtg: MtgService<MtgRepository>,
}

impl Services {
    pub async fn new() -> Result<Self> {
        let db_pool = SqlitePool::connect("data/mtg.sqlite").await?;
        let db_pool = Arc::new(db_pool);
        let mtg_repository = MtgRepository::new(Arc::clone(&db_pool)).await?;
        let mtg_repository = Arc::new(mtg_repository);
        let mtg = MtgService::new(mtg_repository);

        Ok(Self { mtg })
    }
}
