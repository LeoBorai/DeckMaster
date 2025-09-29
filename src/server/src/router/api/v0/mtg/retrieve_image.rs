use axum::Extension;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use http::header::{CACHE_CONTROL, CONTENT_TYPE};
use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

use deckmaster_domain::mtg::service::FindImageFilter;

use crate::router::api::v0::ApiError;
use crate::services::SharedServices;

#[derive(Default, Debug, Deserialize, IntoParams)]
pub struct RetrieveImageQuery {
    card_id: Uuid,
    deck_id: Uuid,
}

#[utoipa::path(
    get,
    operation_id = "retrieve_image",
    path = "/api/v0/mtg/image/{deck_id}/{card_id}",
    params(RetrieveImageQuery),
    responses(
        (status = 200, description = "Retrieves an image from storage", body = Vec<u8>),
        (status = 400, description = "Invalid query parameters", body = ApiError)
    ),
    tag = "cards"
)]
pub async fn handler(
    Extension(services): Extension<SharedServices>,
    Path(path): Path<RetrieveImageQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let bytes = services
        .mtg
        .get_image(FindImageFilter {
            card_id: path.card_id,
            deck_id: path.deck_id,
        })
        .await
        .map_err(|err| {
            tracing::error!(
                "Failed to retrieve card image for {:?}. {:?}",
                (path.deck_id, path.card_id),
                err
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok((
        [
            (CONTENT_TYPE, "image/jpeg"),
            (CACHE_CONTROL, "max-age=31536000, immutable"),
        ],
        bytes.to_vec(),
    ))
}
