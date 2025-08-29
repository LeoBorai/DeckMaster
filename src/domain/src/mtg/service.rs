use std::sync::Arc;

use anyhow::Result;
use bytes::Bytes;
use uuid::Uuid;

use crate::mtg::model::{Card, Deck};

#[derive(Clone, Debug, Default)]
pub struct FindCardsFilter {
    pub deck_id: Option<Uuid>,
    pub page: Option<u32>,
}

#[derive(Clone, Debug, Default)]
pub struct FindDecksFilter {
    pub id: Option<Uuid>,
    pub page: Option<u32>,
}

#[derive(Clone, Debug, Default)]
pub struct FindImageFilter {
    pub card: Option<(Uuid, Uuid)>,
}

#[allow(async_fn_in_trait)]
pub trait MtgDataAccessLayer: Clone + Send + Sync {
    async fn find_cards(&self, filter: FindCardsFilter) -> Result<Vec<Card>>;
    async fn find_decks(&self, filter: FindDecksFilter) -> Result<Vec<Deck>>;
    async fn find_image(&self, filter: FindImageFilter) -> Result<Bytes>;
}

pub struct MtgService<T: MtgDataAccessLayer> {
    repo: Arc<T>,
}

impl<T: MtgDataAccessLayer> MtgService<T> {
    pub fn new(repo: Arc<T>) -> Self {
        MtgService { repo }
    }

    pub async fn get_cards(&self, filter: FindCardsFilter) -> Result<Vec<Card>> {
        let cards = self.repo.find_cards(filter).await?;
        Ok(cards)
    }

    pub async fn get_decks(&self, filter: FindDecksFilter) -> Result<Vec<Deck>> {
        let decks = self.repo.find_decks(filter).await?;
        Ok(decks)
    }

    pub async fn get_image(&self, filter: FindImageFilter) -> Result<Bytes> {
        let image = self.repo.find_image(filter).await?;
        Ok(image)
    }
}
