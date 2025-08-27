use axum::extract::Query;
use axum::http::StatusCode;
use axum::{Extension, Json};

use deckmaster_domain::mtg::service::FindCardsFilter;

use crate::router::api::v0::{PaginatedResponse, PaginationParams};
use crate::services::SharedServices;

use super::Card;

pub async fn handler(
    Extension(services): Extension<SharedServices>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Card>>, StatusCode> {
    let page = pagination.page();
    let limit = pagination.limit();
    let cards: Vec<Card> = services
        .mtg
        .get_cards(FindCardsFilter {
            deck_id: None,
            page: page.into(),
        })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(Card::from)
        .collect();
    let total = cards.len() as u64;
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
