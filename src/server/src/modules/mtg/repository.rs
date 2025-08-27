use std::sync::Arc;

use anyhow::Result;

use deckmaster_domain::mtg::model::{Card, Deck};
use deckmaster_domain::mtg::service::MtgDataAccessLayer;
use deckmaster_domain::mtg::service::{FindCardsFilter, FindDecksFilter};

const PAGE_SIZE: usize = 20;

#[derive(Clone)]
pub struct MtgRepository {
    cards: Arc<Vec<Card>>,
    decks: Arc<Vec<Deck>>,
}

impl MtgRepository {
    pub async fn new(cards: Vec<Card>, decks: Vec<Deck>) -> Result<Self> {
        Ok(MtgRepository {
            cards: Arc::new(cards),
            decks: Arc::new(decks),
        })
    }
}

impl MtgDataAccessLayer for MtgRepository {
    async fn find_cards(&self, filter: FindCardsFilter) -> Result<Vec<Card>> {
        let mut qs = self
            .cards
            .to_vec()
            .iter()
            .filter(|card| {
                if let Some(deck_id) = filter.deck_id {
                    card.deck_id == deck_id
                } else {
                    true
                }
            })
            .cloned()
            .collect::<Vec<Card>>();

        if let Some(page) = filter.page {
            let start = (page as usize - 1) * PAGE_SIZE;
            let end = (start + PAGE_SIZE).min(qs.len());
            qs = qs[start..end].to_vec();
        } else {
            let end = PAGE_SIZE.min(qs.len());
            qs = qs[..end].to_vec();
        }

        Ok(qs)
    }

    async fn find_decks(&self, filter: FindDecksFilter) -> Result<Vec<Deck>> {
        let mut qs = self.decks.to_vec();

        if let Some(page) = filter.page {
            let start = (page as usize - 1) * PAGE_SIZE;
            let end = (start + PAGE_SIZE).min(qs.len());
            qs = qs[start..end].to_vec();
        } else {
            let end = PAGE_SIZE.min(qs.len());
            qs = qs[..end].to_vec();
        }

        Ok(qs)
    }
}
