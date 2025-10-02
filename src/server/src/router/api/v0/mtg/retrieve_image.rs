use axum::Extension;
use axum::body::Body;
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
        (status = 404, description = "Image not found", body = ApiError)
    ),
    tag = "cards"
)]
pub async fn handler(
    Extension(services): Extension<SharedServices>,
    Path(path): Path<RetrieveImageQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let maybe_upstream_stream = services
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

    if let Some(upstream_stream) = maybe_upstream_stream {
        let body = Body::from_stream(upstream_stream);

        return Ok((
            [
                (CONTENT_TYPE, "image/jpeg"),
                (CACHE_CONTROL, "max-age=31536000, immutable"),
            ],
            body,
        ));
    }

    Err(StatusCode::NOT_FOUND)
}
