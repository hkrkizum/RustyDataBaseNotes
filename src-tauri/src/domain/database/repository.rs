use super::entity::{Database, DatabaseId, DatabaseTitle};
use super::error::DatabaseError;

/// Trait defining the persistence operations for [`Database`] entities.
///
/// Implementations handle the actual storage mechanism (e.g. SQLite).
#[allow(async_fn_in_trait)]
pub trait DatabaseRepository {
    /// The error type returned by storage operations.
    type Error: From<DatabaseError>;

    /// Persists a new database.
    async fn create(&self, database: &Database) -> Result<(), Self::Error>;

    /// Retrieves a database by its ID, or returns [`DatabaseError::NotFound`].
    async fn find_by_id(&self, id: &DatabaseId) -> Result<Database, Self::Error>;

    /// Retrieves all databases, ordered by `created_at` descending.
    async fn find_all(&self) -> Result<Vec<Database>, Self::Error>;

    /// Updates the title of an existing database identified by `id`.
    async fn update_title(
        &self,
        id: &DatabaseId,
        title: &DatabaseTitle,
    ) -> Result<Database, Self::Error>;

    /// Deletes a database by its ID. Returns [`DatabaseError::NotFound`] if absent.
    async fn delete(&self, id: &DatabaseId) -> Result<(), Self::Error>;
}
