use axum::Extension;
use axum::extract::Query;
use axum::http::StatusCode;
use bytes::Bytes;
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

use deckmaster_domain::mtg::service::FindImageFilter;

use crate::router::api::v0::ApiError;
use crate::services::SharedServices;

use super::Card;

#[derive(Default, Debug, Deserialize, IntoParams)]
pub struct RetrieveImageQuery {
    card_id: Option<Uuid>,
    deck_id: Option<Uuid>,
}

#[utoipa::path(
    get,
    path = "/api/v0/mtg/image",
    params(RetrieveImageQuery),
    responses(
        (status = 200, description = "Retrieves an image from storage", body = Vec<Card>),
        (status = 400, description = "Invalid query parameters", body = ApiError)
    ),
    tag = "cards"
)]
pub async fn handler(
    Extension(services): Extension<SharedServices>,
    Query(query): Query<RetrieveImageQuery>,
) -> Result<Bytes, StatusCode> {
    let (Some(deck_id), Some(card_id)) = (query.deck_id, query.card_id) else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let bytes = services
        .mtg
        .get_image(FindImageFilter {
            card: Some((deck_id, card_id)),
        })
        .await
        .map_err(|err| {
            tracing::error!(
                "Failed to retrieve card image for {:?}. {:?}",
                (deck_id, card_id),
                err
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(bytes)
}
