use axum::extract::Query;
use axum::http::StatusCode;
use axum::{Extension, Json};
use deckmaster_domain::mtg::service::FindDecksFilter;

use crate::router::api::v0::{ApiError, PaginatedResponse, PaginationParams};
use crate::services::SharedServices;

use super::Deck;

/// Get all Decks with filtering
#[utoipa::path(
    get,
    path = "/api/v0/mtg/decks",
    params(PaginationParams),
    responses(
        (status = 200, description = "List of MTG Decks", body = Vec<Deck>),
        (status = 400, description = "Invalid query parameters", body = ApiError)
    ),
    tag = "decks"
)]
pub async fn handler(
    Extension(services): Extension<SharedServices>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Deck>>, StatusCode> {
    let page = pagination.page();
    let limit = pagination.limit();
    let decks: Vec<Deck> = services
        .mtg
        .get_decks(FindDecksFilter::default())
        .await
        .map_err(|err| {
            tracing::error!("Failed to retrieve decks: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_iter()
        .map(Deck::from)
        .collect();
    let total = decks.len() as u64;
    let total_pages = (total as f64 / limit as f64).ceil() as u32;
    let paginated_response = PaginatedResponse {
        data: decks,
        total,
        page,
        limit,
        total_pages,
    };

    Ok(Json(paginated_response))
}
