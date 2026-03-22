use std::fmt;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::error::PageError;
use crate::domain::database::entity::DatabaseId;

/// Maximum number of characters allowed in a page title.
const MAX_TITLE_LENGTH: usize = 255;

/// A UUIDv7-based identifier for a [`Page`].
///
/// Wraps [`uuid::Uuid`] and is serialized transparently.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PageId(Uuid);

impl PageId {
    /// Generates a new time-ordered UUIDv7 identifier.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Returns the inner [`Uuid`] value.
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for PageId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PageId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s).map(Self)
    }
}

/// A validated page title (1–255 Unicode characters after trimming).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PageTitle(String);

impl PageTitle {
    /// Returns the inner string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for PageTitle {
    type Error = PageError;

    /// Creates a new [`PageTitle`] after trimming and validating the input.
    ///
    /// # Errors
    ///
    /// - [`PageError::TitleEmpty`] if the trimmed string is empty.
    /// - [`PageError::TitleTooLong`] if the trimmed string exceeds 255 characters.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim().to_owned();
        if trimmed.is_empty() {
            return Err(PageError::TitleEmpty);
        }
        let len = trimmed.chars().count();
        if len > MAX_TITLE_LENGTH {
            return Err(PageError::TitleTooLong {
                len,
                max: MAX_TITLE_LENGTH,
            });
        }
        Ok(Self(trimmed))
    }
}

impl fmt::Display for PageTitle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A page entity — the aggregate root for user-created pages.
///
/// A page is either "standalone" (`database_id` is `None`) or
/// "database-owned" (`database_id` is `Some`). Only standalone pages
/// can participate in the page hierarchy via `parent_id`.
#[derive(Debug, Clone)]
pub struct Page {
    id: PageId,
    title: PageTitle,
    database_id: Option<DatabaseId>,
    parent_id: Option<PageId>,
    sort_order: i64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Page {
    /// Creates a new standalone root-level [`Page`] with a generated UUIDv7 ID.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustydatabasenotes_lib::domain::page::entity::{Page, PageTitle};
    /// let title = PageTitle::try_from("My Page".to_owned())?;
    /// let page = Page::new(title);
    /// assert!(page.parent_id().is_none());
    /// assert!(page.is_standalone());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(title: PageTitle) -> Self {
        let now = Utc::now();
        Self {
            id: PageId::new(),
            title,
            database_id: None,
            parent_id: None,
            sort_order: 0,
            created_at: now,
            updated_at: now,
        }
    }

    /// Creates a new standalone [`Page`] as a child of the given parent.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustydatabasenotes_lib::domain::page::entity::{Page, PageId, PageTitle};
    /// let parent_id = PageId::new();
    /// let title = PageTitle::try_from("Child Page".to_owned())?;
    /// let child = Page::new_child(title, parent_id.clone());
    /// assert_eq!(child.parent_id(), Some(&parent_id));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new_child(title: PageTitle, parent_id: PageId) -> Self {
        let now = Utc::now();
        Self {
            id: PageId::new(),
            title,
            database_id: None,
            parent_id: Some(parent_id),
            sort_order: 0,
            created_at: now,
            updated_at: now,
        }
    }

    /// Reconstructs a [`Page`] from stored fields (e.g. database row).
    pub fn from_stored(
        id: PageId,
        title: PageTitle,
        database_id: Option<DatabaseId>,
        parent_id: Option<PageId>,
        sort_order: i64,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            title,
            database_id,
            parent_id,
            sort_order,
            created_at,
            updated_at,
        }
    }

    /// Returns a reference to the page's ID.
    pub fn id(&self) -> &PageId {
        &self.id
    }

    /// Returns a reference to the page's title.
    pub fn title(&self) -> &PageTitle {
        &self.title
    }

    /// Returns a reference to the optional database ID this page belongs to.
    pub fn database_id(&self) -> Option<&DatabaseId> {
        self.database_id.as_ref()
    }

    /// Returns a reference to the optional parent page ID.
    pub fn parent_id(&self) -> Option<&PageId> {
        self.parent_id.as_ref()
    }

    /// Returns the sort order value for ordering within the same parent.
    pub fn sort_order(&self) -> i64 {
        self.sort_order
    }

    /// Returns `true` if this page is standalone (not owned by a database).
    pub fn is_standalone(&self) -> bool {
        self.database_id.is_none()
    }

    /// Returns `true` if this page is owned by a database.
    pub fn is_database_page(&self) -> bool {
        self.database_id.is_some()
    }

    /// Returns the page's creation timestamp.
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Returns the page's last-updated timestamp.
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn page_title_valid() {
        let title = PageTitle::try_from("My Page".to_owned());
        assert!(title.is_ok());
        assert_eq!(title.as_ref().map(|t| t.as_str()), Ok("My Page"));
    }

    #[test]
    fn page_title_empty_string_is_rejected() {
        let result = PageTitle::try_from(String::new());
        assert!(matches!(result, Err(PageError::TitleEmpty)));
    }

    #[test]
    fn page_title_whitespace_only_is_rejected() {
        let result = PageTitle::try_from("   \t\n  ".to_owned());
        assert!(matches!(result, Err(PageError::TitleEmpty)));
    }

    #[test]
    fn page_title_255_chars_is_accepted() {
        let s: String = "a".repeat(255);
        let result = PageTitle::try_from(s);
        assert!(result.is_ok());
    }

    #[test]
    fn page_title_256_chars_is_rejected() {
        let s: String = "a".repeat(256);
        let result = PageTitle::try_from(s);
        assert!(matches!(
            result,
            Err(PageError::TitleTooLong { len: 256, max: 255 })
        ));
    }

    #[test]
    fn page_title_trims_whitespace() {
        let title = PageTitle::try_from("  hello  ".to_owned());
        assert!(title.is_ok());
        assert_eq!(title.as_ref().map(|t| t.as_str()), Ok("hello"));
    }

    #[test]
    fn page_new_generates_valid_id_and_timestamps() {
        let title = PageTitle::try_from("Test".to_owned());
        assert!(title.is_ok());
        let page = Page::new(title.expect("test title"));
        // ID should be a valid UUID string
        let id_str = page.id().to_string();
        assert_eq!(id_str.len(), 36);
        // created_at and updated_at should be equal on construction
        assert_eq!(page.created_at(), page.updated_at());
    }

    #[test]
    fn page_id_display_and_from_str_roundtrip() {
        let id = PageId::new();
        let s = id.to_string();
        let parsed: PageId = s.parse().expect("should parse");
        assert_eq!(id, parsed);
    }
}
