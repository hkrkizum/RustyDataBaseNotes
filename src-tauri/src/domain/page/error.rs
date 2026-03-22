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

    /// Moving a page would create a circular reference in the hierarchy.
    #[error("circular reference detected: page {page_id} cannot be moved under {target_parent_id}")]
    CircularReference {
        /// The page being moved.
        page_id: String,
        /// The target parent that would cause a cycle.
        target_parent_id: String,
    },

    /// The resulting nesting depth would exceed the maximum allowed.
    #[error("maximum nesting depth ({max_depth}) exceeded for page {page_id}")]
    MaxDepthExceeded {
        /// The page being moved or created.
        page_id: String,
        /// The depth that would result from the operation.
        current_depth: usize,
        /// The maximum allowed depth.
        max_depth: usize,
    },

    /// A database-owned page cannot participate in the page hierarchy.
    #[error("database page {page_id} cannot participate in page hierarchy")]
    DatabasePageCannotNest {
        /// The database-owned page ID.
        page_id: String,
    },
}
