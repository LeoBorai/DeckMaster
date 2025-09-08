mod mtg;

use axum::Router;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, OpenApi, ToSchema, schema};

use deckmaster_domain::common::query_set::QuerySet;

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
    pub page: u32,
    pub total: u32,
    pub total_pages: u32,
    pub data: Vec<T>,
}

impl<T: Clone> From<QuerySet<T>> for PaginatedResponse<T> {
    fn from(qs: QuerySet<T>) -> Self {
        let data = qs.records().into_iter().collect();

        PaginatedResponse {
            total: qs.count(),
            page: qs.page(),
            total_pages: qs.total_pages(),
            data,
        }
    }
}

#[derive(Default, Debug, Deserialize, IntoParams)]
pub struct PaginationParams {
    /// Page number (starts from 1)
    #[param(example = 1, minimum = 1)]
    pub(self) page: Option<u32>,
}

impl PaginationParams {
    #[inline]
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(1)
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
        (url = "https://api.deckmaster.leoborai.com", description = "Production server")
    )
)]
pub struct ApiV0Doc;

pub fn routes() -> Router {
    Router::new().nest("/mtg", mtg::routes())
}
