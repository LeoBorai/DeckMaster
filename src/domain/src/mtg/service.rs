use std::sync::Arc;

use anyhow::Result;
use uuid::Uuid;

use crate::mtg::model::{Card, Deck};

#[derive(Clone, Default)]
pub struct FindCardsFilter {
    pub deck_id: Option<Uuid>,
}

#[derive(Clone, Default)]
pub struct FindDecksFilter {
    pub id: Option<Uuid>,
}

#[allow(async_fn_in_trait)]
pub trait MtgDataAccessLayer: Clone + Send + Sync {
    async fn find_cards(&self, filter: FindCardsFilter) -> Result<Vec<Card>>;
    async fn find_decks(&self, filter: FindDecksFilter) -> Result<Vec<Deck>>;
}

pub struct MtgService<T: MtgDataAccessLayer> {
    repo: Arc<T>,
}

impl<T: MtgDataAccessLayer> MtgService<T> {
    pub async fn new(repo: Arc<T>) -> Result<Self> {
        Ok(MtgService { repo })
    }

    pub async fn get_cards(&self, filter: FindCardsFilter) -> Result<Vec<Card>> {
        let cards = self.repo.find_cards(filter).await?;
        Ok(cards)
    }

    pub async fn get_decks(&self, filter: FindDecksFilter) -> Result<Vec<Deck>> {
        let decks = self.repo.find_decks(filter).await?;
        Ok(decks)
    }
}
