use super::entity::DatabaseId;

/// Errors originating from the Database domain model.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum DatabaseError {
    /// The database title was empty after trimming whitespace.
    #[error("database title must not be empty")]
    TitleEmpty,

    /// The database title exceeded the maximum allowed length.
    #[error("database title too long: {len} characters (max {max})")]
    TitleTooLong {
        /// Actual character count.
        len: usize,
        /// Maximum allowed character count.
        max: usize,
    },

    /// No database was found with the given ID.
    #[error("database not found: {id}")]
    NotFound {
        /// The ID that was looked up.
        id: DatabaseId,
    },
}
