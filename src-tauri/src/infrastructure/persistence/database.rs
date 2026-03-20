use std::path::Path;

use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

use super::error::StorageError;

/// Initializes a SQLite connection pool at the given path.
///
/// - Creates the parent directory if it does not exist.
/// - Opens (or creates) the database file.
/// - Enables WAL journal mode for concurrent read/write support.
/// - Runs all pending migrations via [`sqlx::migrate!()`].
///
/// # Errors
///
/// Returns [`StorageError`] if:
/// - The parent directory cannot be created ([`StorageError::DatabasePath`]).
/// - The connection pool cannot be opened ([`StorageError::Sqlx`]).
/// - Migrations fail to apply ([`StorageError::Migration`]).
pub async fn init_pool(db_path: &Path) -> Result<SqlitePool, StorageError> {
    // Validate existing DB file header if it already exists
    if db_path.exists() {
        validate_sqlite_header(db_path)?;
    }

    // Ensure the parent directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

    let options: SqliteConnectOptions = db_url
        .parse()
        .map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;

    let options = options
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .busy_timeout(std::time::Duration::from_secs(5));

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    // Enable foreign key constraint enforcement (SQLite default: OFF).
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

    Ok(pool)
}

/// SQLite file header magic bytes.
const SQLITE_HEADER: &[u8] = b"SQLite format 3\0";

/// Validates that a database file has a valid SQLite header.
///
/// # Errors
///
/// Returns [`StorageError::DatabasePath`] if the file cannot be read or has
/// an invalid SQLite header.
fn validate_sqlite_header(path: &Path) -> Result<(), StorageError> {
    use std::io::{self, Read};

    let mut file = std::fs::File::open(path)?;
    let mut header = [0u8; 16];
    let bytes_read = file.read(&mut header)?;

    if bytes_read < SQLITE_HEADER.len() || header != *SQLITE_HEADER {
        return Err(StorageError::DatabasePath(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "database file is not a valid SQLite database: {}",
                path.display()
            ),
        )));
    }

    Ok(())
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn init_pool_succeeds_for_new_db() {
        let dir = std::env::temp_dir().join(format!("rdbn_test_init_{}", uuid::Uuid::now_v7()));
        let db_path = dir.join("test.db");

        let pool = init_pool(&db_path).await.expect("should succeed");
        pool.close().await;

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[tokio::test]
    async fn init_pool_rejects_corrupt_db() {
        let dir = std::env::temp_dir().join(format!("rdbn_test_corrupt_{}", uuid::Uuid::now_v7()));
        std::fs::create_dir_all(&dir).expect("create dir");
        let db_path = dir.join("corrupt.db");

        // Write invalid header
        std::fs::write(&db_path, b"NOT A SQLITE FILE!!").expect("write");

        let result = init_pool(&db_path).await;
        assert!(result.is_err());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn validate_sqlite_header_rejects_invalid() {
        let dir = std::env::temp_dir().join(format!("rdbn_test_header_{}", uuid::Uuid::now_v7()));
        std::fs::create_dir_all(&dir).expect("create dir");
        let path = dir.join("bad.db");

        std::fs::write(&path, b"garbage data here").expect("write");
        let result = validate_sqlite_header(&path);
        assert!(result.is_err());

        let _ = std::fs::remove_dir_all(&dir);
    }
}
