pub const PAGE_SIZE: u64 = 20;

#[derive(Debug, Clone)]
pub struct Pagination {
    page: u64,
}

impl Pagination {
    pub fn new(page: u64) -> Self {
        Self { page }
    }

    pub fn page(&self) -> u64 {
        self.page
    }

    pub fn offset(&self) -> u64 {
        (self.page.saturating_sub(1)) * PAGE_SIZE
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self { page: 1 }
    }
}
