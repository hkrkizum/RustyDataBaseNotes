use super::entity::{Page, PageId, PageTitle};
use super::error::PageError;

/// Trait defining the persistence operations for [`Page`] entities.
///
/// Implementations handle the actual storage mechanism (e.g. SQLite).
#[allow(async_fn_in_trait)]
pub trait PageRepository {
    /// The error type returned by storage operations.
    type Error: From<PageError>;

    /// Persists a new page.
    async fn create(&self, page: &Page) -> Result<(), Self::Error>;

    /// Retrieves a page by its ID, or returns [`PageError::NotFound`].
    async fn find_by_id(&self, id: &PageId) -> Result<Page, Self::Error>;

    /// Retrieves all pages, ordered by `created_at` descending.
    async fn find_all(&self) -> Result<Vec<Page>, Self::Error>;

    /// Updates the title of an existing page identified by `id`.
    async fn update_title(&self, id: &PageId, title: &PageTitle) -> Result<Page, Self::Error>;

    /// Deletes a page by its ID. Returns [`PageError::NotFound`] if absent.
    async fn delete(&self, id: &PageId) -> Result<(), Self::Error>;
}
