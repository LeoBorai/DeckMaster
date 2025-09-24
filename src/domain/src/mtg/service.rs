use std::sync::Arc;

use anyhow::Result;
use bytes::Bytes;
use uuid::Uuid;

use crate::common::query_set::QuerySet;
use crate::mtg::model::{Card, Deck};

#[derive(Clone, Debug, Default)]
pub struct FindCardsFilter {
    pub id: Option<Uuid>,
    pub deck_id: Option<Uuid>,
    pub unique: Option<bool>,
    pub title: Option<String>,
    pub page: Option<u32>,
}

#[derive(Clone, Debug, Default)]
pub struct FindDecksFilter {
    pub id: Option<Uuid>,
    pub page: Option<u32>,
}

#[derive(Clone, Debug, Default)]
pub struct FindImageFilter {
    pub card_id: Uuid,
    pub deck_id: Uuid,
}

#[allow(async_fn_in_trait)]
pub trait MtgDataAccessLayer: Clone + Send + Sync {
    async fn find_cards(&self, filter: FindCardsFilter) -> Result<QuerySet<Card>>;
    async fn find_decks(&self, filter: FindDecksFilter) -> Result<QuerySet<Deck>>;
    async fn find_image(&self, filter: FindImageFilter) -> Result<Bytes>;
}

pub struct MtgService<T: MtgDataAccessLayer> {
    repo: Arc<T>,
}

impl<T: MtgDataAccessLayer> MtgService<T> {
    pub fn new(repo: Arc<T>) -> Self {
        MtgService { repo }
    }

    pub async fn get_cards(&self, filter: FindCardsFilter) -> Result<QuerySet<Card>> {
        let cards = self.repo.find_cards(filter).await?;
        Ok(cards)
    }

    pub async fn get_decks(&self, filter: FindDecksFilter) -> Result<QuerySet<Deck>> {
        let decks = self.repo.find_decks(filter).await?;
        Ok(decks)
    }

    pub async fn get_image(&self, filter: FindImageFilter) -> Result<Bytes> {
        let image = self.repo.find_image(filter).await?;
        Ok(image)
    }
}
