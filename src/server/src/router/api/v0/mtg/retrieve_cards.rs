use axum::extract::Query;
use axum::http::StatusCode;
use axum::{Extension, Json};
use deckmaster_domain::mtg::service::FindCardsFilter;

use crate::router::api::v0::{ApiError, PaginatedResponse, PaginationParams};
use crate::services::SharedServices;

use super::Card;

#[utoipa::path(
    get,
    path = "/api/v0/mtg/cards",
    params(PaginationParams),
    responses(
        (status = 200, description = "List of MTG Cards", body = Vec<Card>),
        (status = 400, description = "Invalid query parameters", body = ApiError)
    ),
    tag = "cards"
)]
pub async fn handler(
    Extension(services): Extension<SharedServices>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Card>>, StatusCode> {
    let cards: Vec<Card> = services
        .mtg
        .get_cards(FindCardsFilter::default())
        .await
        .map_err(|err| {
            tracing::error!("Failed to retrieve cards: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_iter()
        .map(Card::from)
        .collect();
    let total = cards.len() as u64;
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(20);
    let total_pages = (total as f64 / limit as f64).ceil() as u32;
    let paginated_response = PaginatedResponse {
        data: cards,
        total,
        page,
        limit,
        total_pages,
    };

    Ok(Json(paginated_response))
}
