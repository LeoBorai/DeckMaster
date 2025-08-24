use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Deck {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub release: DateTime<Utc>,
}
