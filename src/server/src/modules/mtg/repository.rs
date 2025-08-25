use std::sync::Arc;

use anyhow::Result;

use deckmaster_domain::mtg::model::{Card, Deck};
use deckmaster_domain::mtg::service::MtgDataAccessLayer;
use deckmaster_domain::mtg::service::{FindCardsFilter, FindDecksFilter};

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
        let qs = self
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

        Ok(qs)
    }

    async fn find_decks(&self, _: FindDecksFilter) -> Result<Vec<Deck>> {
        Ok(self.decks.to_vec())
    }
}
