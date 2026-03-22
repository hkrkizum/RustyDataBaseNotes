use super::entity::{Page, PageId, PageTitle};
use super::error::PageError;
use crate::domain::database::entity::DatabaseId;

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

    /// Sets the `database_id` for a page.
    async fn set_database_id(
        &self,
        page_id: &PageId,
        database_id: Option<&DatabaseId>,
    ) -> Result<(), Self::Error>;

    /// Returns all pages not belonging to any database (`database_id IS NULL`).
    async fn find_standalone_pages(&self) -> Result<Vec<Page>, Self::Error>;

    /// Returns all pages belonging to the given database, ordered by
    /// `created_at` descending.
    async fn find_by_database_id(&self, database_id: &DatabaseId)
    -> Result<Vec<Page>, Self::Error>;

    /// Updates the `parent_id` of a page (move / reparent operation).
    async fn update_parent_id(
        &self,
        page_id: &PageId,
        parent_id: Option<&PageId>,
    ) -> Result<Page, Self::Error>;

    /// Returns all direct children of the given parent page.
    async fn find_children(&self, parent_id: &PageId) -> Result<Vec<Page>, Self::Error>;

    /// Returns root-level standalone pages (`parent_id IS NULL` and
    /// `database_id IS NULL`), ordered by `created_at` descending.
    async fn find_root_pages(&self) -> Result<Vec<Page>, Self::Error>;

    /// Returns the ancestor chain from the given page to the root
    /// using a recursive CTE, with a safety limit of 10 levels.
    async fn find_ancestors(&self, page_id: &PageId) -> Result<Vec<Page>, Self::Error>;

    /// Bulk-updates the `parent_id` for multiple pages in a single
    /// statement.
    ///
    /// Intended for use inside a transaction during parent deletion
    /// (child promotion).
    async fn bulk_update_parent_id(
        &self,
        page_ids: &[PageId],
        new_parent_id: Option<&PageId>,
    ) -> Result<(), Self::Error>;
}
