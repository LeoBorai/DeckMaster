mod mtg;

use axum::Router;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    /// Error message
    pub message: String,
    /// Error code for client handling
    pub code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

#[derive(Default, Debug, Deserialize)]
pub struct PaginationParams {
    /// Page number (starts from 1)
    pub(self) page: Option<u32>,
    /// Number of items per page
    pub(self) limit: Option<u32>,
}

impl PaginationParams {
    #[inline]
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(1).min(1)
    }

    #[inline]
    pub fn limit(&self) -> u32 {
        self.limit.unwrap_or(20).max(100)
    }
}

pub fn routes() -> Router {
    Router::new().nest("/mtg", mtg::routes())
}
