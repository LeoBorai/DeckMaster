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
    Query(page): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Deck>>, StatusCode> {
    let decks_qs = services
        .mtg
        .get_decks(FindDecksFilter {
            page: page.page().into(),
            ..Default::default()
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
