pub mod retrieve_cards;
pub mod retrieve_decks;

use axum::{Router, routing::get};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Card {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174003")]
    pub id: Uuid,
    #[schema(example = "The Wise Mothman")]
    pub title: String,
    #[schema(example = "0004")]
    pub number: i64,
    #[schema(
        example = "Whenever The Wise Mothman enters the battlefield or attacks, each player gets a rad counter."
    )]
    pub description: Option<String>,
    #[schema(example = "1, B, G, U")]
    pub mana: Option<Vec<String>>,
    #[schema(example = "Legendary Creature — Insect Mutant")]
    pub kind: String,
    #[schema(example = "M")]
    pub rarity: String,
    #[schema(example = "Sergei Leoluch Panin")]
    pub artist: Option<String>,
    #[schema(example = "3")]
    pub power: Option<String>,
    #[schema(example = "3")]
    pub toughness: Option<String>,
    #[schema(example = "123e4567-e89b-12d3-a456-426614174003")]
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Deck {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174003")]
    pub id: Uuid,
    #[schema(example = "Fallout")]
    pub name: String,
    #[schema(example = "PIP")]
    pub code: String,
    #[schema(example = "2024-01-01T00:00:00Z")]
    pub release: DateTime<Utc>,
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
