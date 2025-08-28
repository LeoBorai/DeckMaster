pub mod retrieve_cards;
pub mod retrieve_decks;

use axum::{Router, routing::get};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: Uuid,
    pub title: String,
    pub number: i64,
    pub description: Option<String>,
    pub mana: Option<Vec<String>>,
    pub kind: String,
    pub rarity: String,
    pub artist: Option<String>,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub deck_id: Uuid,
}

impl From<deckmaster_domain::mtg::model::Card> for Card {
    fn from(card: deckmaster_domain::mtg::model::Card) -> Self {
        Self {
            id: card.id,
            title: card.title,
            number: card.number,
            description: card.description,
            mana: card.mana,
            kind: card.kind,
            rarity: card.rarity,
            artist: card.artist,
            power: card.power,
            toughness: card.toughness,
            deck_id: card.deck_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub release: i64,
}

impl From<deckmaster_domain::mtg::model::Deck> for Deck {
    fn from(deck: deckmaster_domain::mtg::model::Deck) -> Self {
        Self {
            id: deck.id,
            name: deck.name,
            code: deck.code,
            release: deck.release,
        }
    }
}

pub fn routes() -> Router {
    Router::new()
        .route("/cards", get(retrieve_cards::handler))
        .route("/decks", get(retrieve_decks::handler))
}
