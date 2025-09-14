use error::{Error, Result};

pub const MIN_USER_PAGE_SIZE: i64 = 12;
pub const MAX_USER_PAGE_SIZE: i64 = 20;

#[derive(Debug, Clone, Copy)]
pub struct PagingConfig {
    pub page: i64,
    pub page_size: i64,
    pub offset: i64,
}

impl PagingConfig {
    pub fn new(page: i64, page_size: i64) -> Result<Self> {
        if page < 1 {
            return Err(Error::invalid_argument(
                "Invalid: page must be greater than 0",
            ));
        }

        if page_size < 1 {
            return Err(Error::invalid_argument(
                "Invalid: page_size must be greater than 0",
            ));
        }

        let offset = (page - 1).checked_mul(page_size).ok_or_else(|| {
            Error::invalid_argument("Invalid: either page or page_size too large")
        })?;

        Ok(Self {
            page,
            page_size,
            offset,
        })
    }
}

impl Default for PagingConfig {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: MIN_USER_PAGE_SIZE,
            offset: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PagingInfo {
    pub page: i64,
    pub page_size: i64,
    pub total_count: i64,
}

#[derive(Debug)]
pub struct PagingSearchPayload<T> {
    pub paging_info: PagingInfo,
    pub items: Vec<T>,
}
