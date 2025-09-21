use axum::extract::Query;
use axum::http::StatusCode;
use axum::{Extension, Json};

use deckmaster_domain::mtg::service::FindCardsFilter;
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

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
    Query(filter): Query<FindCardsParams>,
) -> Result<Json<PaginatedResponse<Card>>, StatusCode> {
    let page = pagination.page();
    let cards_qs = services
        .mtg
        .get_cards(FindCardsFilter {
            deck_id: None,
            id: filter.id,
            title: filter.title,
            page: page.into(),
        })
        .await
        .map_err(|err| {
            tracing::error!("Failed to retrieve cards: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let cards_qs = cards_qs.inner_map(Card::from);
    let paginated_response = PaginatedResponse::from(cards_qs);

    Ok(Json(paginated_response))
}

#[derive(Default, Debug, Deserialize, IntoParams)]
pub struct FindCardsParams {
    /// ID for the card
    #[param(example = "2504cb4b-292f-5dd8-8c9e-6e805500454d")]
    pub(self) id: Option<Uuid>,
    /// Title for the card
    #[param(example = "The Wise Mothman")]
    pub(self) title: Option<String>,
}
