mod mtg;

use axum::Router;

use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, OpenApi, ToSchema, schema};

use crate::router::api::v0::mtg::{Card, Deck};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiError {
    /// Error message
    #[schema(example = "Error message")]
    pub message: String,
    /// Error code for client handling
    #[schema(example = "NOT_FOUND")]
    pub code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

#[derive(Default, Debug, Deserialize, IntoParams)]
pub struct PaginationParams {
    /// Page number (starts from 1)
    #[param(example = 1, minimum = 1)]
    pub(self) page: Option<u32>,
    /// Number of items per page
    #[param(example = 20, minimum = 1, maximum = 100)]
    pub(self) limit: Option<u32>,
}

impl PaginationParams {
    #[inline]
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(1).min(1)
    }

    #[inline]
    pub fn limit(&self) -> u32 {
        self.limit.unwrap_or(20).max(100)
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        mtg::retrieve_cards::handler,
        mtg::retrieve_decks::handler,
    ),
    components(
        schemas(
            Card,
            Deck,
            ApiError,
            PaginatedResponse<Card>,
            PaginatedResponse<Deck>,
        )
    ),
    tags(
        (name = "cards", description = "Cards details retrieval endpoints"),
        (name = "decks", description = "Decks details retrieval endpoints"),
    ),
    info(
        title = "DeckMaster API",
        version = "0",
        description = "API for MTG cards and decks",
        contact(
            name = "API Support",
            email = "hi@leoborai.com"
        )
    ),
    servers(
        (url = "http://localhost:7878", description = "Local development server"),
        (url = "https://deckmaster.leoborai.com", description = "Production server")
    )
)]
pub struct ApiV0Doc;

pub fn routes() -> Router {
    Router::new().nest("/mtg", mtg::routes())
}
