/// Errors originating from the persistence / storage layer.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    /// An error from the SQLite driver (sqlx).
    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    /// A migration failed to apply.
    #[error("migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    /// The database file path could not be resolved or created.
    #[error("database path error: {0}")]
    DatabasePath(#[from] std::io::Error),
}
