use std::fmt;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::error::DatabaseError;

/// Maximum number of characters allowed in a database title.
const MAX_TITLE_LENGTH: usize = 255;

/// A UUIDv7-based identifier for a [`Database`].
///
/// Wraps [`uuid::Uuid`] and is serialized transparently.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DatabaseId(Uuid);

impl DatabaseId {
    /// Generates a new time-ordered UUIDv7 identifier.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Returns the inner [`Uuid`] value.
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for DatabaseId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DatabaseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for DatabaseId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s).map(Self)
    }
}

/// A validated database title (1-255 Unicode characters after trimming).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DatabaseTitle(String);

impl DatabaseTitle {
    /// Returns the inner string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for DatabaseTitle {
    type Error = DatabaseError;

    /// Creates a new [`DatabaseTitle`] after trimming and validating the input.
    ///
    /// # Errors
    ///
    /// - [`DatabaseError::TitleEmpty`] if the trimmed string is empty.
    /// - [`DatabaseError::TitleTooLong`] if the trimmed string exceeds 255 characters.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim().to_owned();
        if trimmed.is_empty() {
            return Err(DatabaseError::TitleEmpty);
        }
        let len = trimmed.chars().count();
        if len > MAX_TITLE_LENGTH {
            return Err(DatabaseError::TitleTooLong {
                len,
                max: MAX_TITLE_LENGTH,
            });
        }
        Ok(Self(trimmed))
    }
}

impl fmt::Display for DatabaseTitle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A database entity — the aggregate root for user-created databases.
#[derive(Debug, Clone)]
pub struct Database {
    id: DatabaseId,
    title: DatabaseTitle,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Database {
    /// Creates a new [`Database`] with a generated UUIDv7 ID and the current timestamp.
    pub fn new(title: DatabaseTitle) -> Self {
        let now = Utc::now();
        Self {
            id: DatabaseId::new(),
            title,
            created_at: now,
            updated_at: now,
        }
    }

    /// Reconstructs a [`Database`] from stored fields (e.g. database row).
    pub fn from_stored(
        id: DatabaseId,
        title: DatabaseTitle,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            title,
            created_at,
            updated_at,
        }
    }

    /// Returns a reference to the database's ID.
    pub fn id(&self) -> &DatabaseId {
        &self.id
    }

    /// Returns a reference to the database's title.
    pub fn title(&self) -> &DatabaseTitle {
        &self.title
    }

    /// Returns the database's creation timestamp.
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Returns the database's last-updated timestamp.
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn database_title_valid() {
        let title = DatabaseTitle::try_from("My Database".to_owned());
        assert!(title.is_ok());
        assert_eq!(title.as_ref().map(|t| t.as_str()), Ok("My Database"));
    }

    #[test]
    fn database_title_empty_string_is_rejected() {
        let result = DatabaseTitle::try_from(String::new());
        assert!(matches!(result, Err(DatabaseError::TitleEmpty)));
    }

    #[test]
    fn database_title_whitespace_only_is_rejected() {
        let result = DatabaseTitle::try_from("   \t\n  ".to_owned());
        assert!(matches!(result, Err(DatabaseError::TitleEmpty)));
    }

    #[test]
    fn database_title_255_chars_is_accepted() {
        let s: String = "a".repeat(255);
        let result = DatabaseTitle::try_from(s);
        assert!(result.is_ok());
    }

    #[test]
    fn database_title_256_chars_is_rejected() {
        let s: String = "a".repeat(256);
        let result = DatabaseTitle::try_from(s);
        assert!(matches!(
            result,
            Err(DatabaseError::TitleTooLong { len: 256, max: 255 })
        ));
    }

    #[test]
    fn database_title_trims_whitespace() {
        let title = DatabaseTitle::try_from("  hello  ".to_owned());
        assert!(title.is_ok());
        assert_eq!(title.as_ref().map(|t| t.as_str()), Ok("hello"));
    }

    #[test]
    fn database_new_generates_valid_id_and_timestamps() {
        let title = DatabaseTitle::try_from("Test".to_owned());
        assert!(title.is_ok());
        let database = Database::new(title.expect("test title"));
        // ID should be a valid UUID string
        let id_str = database.id().to_string();
        assert_eq!(id_str.len(), 36);
        // created_at and updated_at should be equal on construction
        assert_eq!(database.created_at(), database.updated_at());
    }

    #[test]
    fn database_id_display_and_from_str_roundtrip() {
        let id = DatabaseId::new();
        let s = id.to_string();
        let parsed: DatabaseId = s.parse().expect("should parse");
        assert_eq!(id, parsed);
    }
}
