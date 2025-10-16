use axum::extract::Query;
use axum::http::StatusCode;
use axum::{Extension, Json};

use deckmaster_domain::mtg::service::FindDecksFilter;
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::router::api::v0::{ApiError, PaginatedResponse, PaginationParams};
use crate::services::SharedServices;

use super::Deck;

/// Get all Decks with filtering
#[utoipa::path(
    get,
    operation_id = "retrieve_decks",
    path = "/api/v0/mtg/decks",
    params(PaginationParams),
    responses(
        (status = 200, description = "List of MTG Decks", body = PaginatedResponse<Deck>),
        (status = 400, description = "Invalid query parameters", body = ApiError)
    ),
    tag = "decks"
)]
pub async fn handler(
    Extension(services): Extension<SharedServices>,
    Query(filter): Query<FindDecksParams>,
) -> Result<Json<PaginatedResponse<Deck>>, StatusCode> {
    let pagination = PaginationParams { page: filter.page };
    let decks_qs = services
        .mtg
        .get_decks(FindDecksFilter {
            id: filter.id,
            pagination: pagination.into(),
        })
        .await
        .map_err(|err| {
            tracing::error!("Failed to retrieve decks: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let decks_qs = decks_qs.inner_map(Deck::from);
    let paginated_response = PaginatedResponse::from(decks_qs);

    Ok(Json(paginated_response))
}

#[derive(Default, Debug, Deserialize, IntoParams)]
pub struct FindDecksParams {
    /// ID for the deck
    #[param(example = "2504cb4b-292f-5dd8-8c9e-6e805500454d")]
    pub(self) id: Option<Uuid>,
    /// Page number (starts from 1)
    #[param(example = 1, minimum = 1)]
    pub(self) page: Option<u32>,
}
