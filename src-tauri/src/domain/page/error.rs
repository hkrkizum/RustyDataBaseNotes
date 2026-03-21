use super::entity::PageId;
use crate::domain::database::entity::DatabaseId;

/// Errors originating from the Page domain model.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum PageError {
    /// The page title was empty after trimming whitespace.
    #[error("page title must not be empty")]
    TitleEmpty,

    /// The page title exceeded the maximum allowed length.
    #[error("page title too long: {len} characters (max {max})")]
    TitleTooLong {
        /// Actual character count.
        len: usize,
        /// Maximum allowed character count.
        max: usize,
    },

    /// No page was found with the given ID.
    #[error("page not found: {id}")]
    NotFound {
        /// The ID that was looked up.
        id: PageId,
    },

    /// The page is already in a database.
    #[error("page {page_id} already in database {database_id}")]
    AlreadyInDatabase {
        /// The page ID.
        page_id: PageId,
        /// The database ID.
        database_id: DatabaseId,
    },
}
