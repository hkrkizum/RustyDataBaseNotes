use std::fmt;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::error::BlockError;
use crate::domain::page::entity::PageId;

/// Maximum number of Unicode characters allowed in a block's content.
const MAX_CONTENT_LENGTH: usize = 10_000;

/// A UUIDv7-based identifier for a [`Block`].
///
/// Wraps [`uuid::Uuid`] and is serialized transparently.
///
/// # Examples
///
/// ```
/// # use rustydatabasenotes_lib::domain::block::entity::BlockId;
/// let id = BlockId::new();
/// let s = id.to_string();
/// let parsed: BlockId = s.parse().unwrap();
/// assert_eq!(id, parsed);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BlockId(Uuid);

impl BlockId {
    /// Generates a new time-ordered UUIDv7 identifier.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for BlockId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for BlockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for BlockId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s).map(Self)
    }
}

/// Validated block content (0–10,000 Unicode characters).
///
/// Empty strings are permitted (FR-011). The character count uses
/// [`str::chars`] (Unicode scalar values), which is the authoritative measure.
///
/// # Examples
///
/// ```
/// # use rustydatabasenotes_lib::domain::block::entity::BlockContent;
/// let content = BlockContent::try_from(String::new());
/// assert!(content.is_ok()); // empty is valid
///
/// let long = "a".repeat(10_001);
/// let err = BlockContent::try_from(long);
/// assert!(err.is_err()); // too long
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BlockContent(String);

impl BlockContent {
    /// Returns the inner string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for BlockContent {
    type Error = BlockError;

    /// Creates a new [`BlockContent`] after validating the character count.
    ///
    /// # Errors
    ///
    /// - [`BlockError::ContentTooLong`] if the string exceeds 10,000 characters.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let len = value.chars().count();
        if len > MAX_CONTENT_LENGTH {
            return Err(BlockError::ContentTooLong {
                len,
                max: MAX_CONTENT_LENGTH,
            });
        }
        Ok(Self(value))
    }
}

impl fmt::Display for BlockContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A validated non-negative block position (0-based display order).
///
/// Managed internally by [`EditorSession`](crate::domain::editor::session::EditorSession).
///
/// # Examples
///
/// ```
/// # use rustydatabasenotes_lib::domain::block::entity::BlockPosition;
/// let pos = BlockPosition::try_from(0_i64);
/// assert!(pos.is_ok());
///
/// let neg = BlockPosition::try_from(-1_i64);
/// assert!(neg.is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BlockPosition(i64);

impl BlockPosition {
    /// Returns the inner `i64` value.
    pub fn value(&self) -> i64 {
        self.0
    }
}

impl TryFrom<i64> for BlockPosition {
    type Error = BlockError;

    /// Creates a new [`BlockPosition`] after validating non-negativity.
    ///
    /// # Errors
    ///
    /// - [`BlockError::InvalidPosition`] if the value is negative.
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(BlockError::InvalidPosition { value });
        }
        Ok(Self(value))
    }
}

impl fmt::Display for BlockPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A block entity — a child of [`Page`](crate::domain::page::entity::Page)
/// representing a unit of content.
///
/// Blocks are ordered by [`BlockPosition`] within a page and managed
/// by [`EditorSession`](crate::domain::editor::session::EditorSession).
#[derive(Debug, Clone)]
pub struct Block {
    id: BlockId,
    page_id: PageId,
    block_type: String,
    content: BlockContent,
    position: BlockPosition,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Block {
    /// Creates a new empty text block for the given page at the specified position.
    ///
    /// Generates a UUIDv7 identifier and sets both timestamps to the current UTC time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustydatabasenotes_lib::domain::page::entity::PageId;
    /// # use rustydatabasenotes_lib::domain::block::entity::{Block, BlockPosition};
    /// let page_id = PageId::new();
    /// let pos = BlockPosition::try_from(0_i64).unwrap();
    /// let block = Block::new(page_id, pos);
    /// assert_eq!(block.content().as_str(), "");
    /// assert_eq!(block.block_type(), "text");
    /// ```
    pub fn new(page_id: PageId, position: BlockPosition) -> Self {
        let now = Utc::now();
        Self {
            id: BlockId::new(),
            page_id,
            block_type: "text".to_owned(),
            content: BlockContent(String::new()),
            position,
            created_at: now,
            updated_at: now,
        }
    }

    /// Reconstructs a [`Block`] from stored fields (e.g. database row).
    pub fn from_stored(
        id: BlockId,
        page_id: PageId,
        block_type: String,
        content: BlockContent,
        position: BlockPosition,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            page_id,
            block_type,
            content,
            position,
            created_at,
            updated_at,
        }
    }

    /// Returns a reference to the block's ID.
    pub fn id(&self) -> &BlockId {
        &self.id
    }

    /// Returns a reference to the parent page's ID.
    pub fn page_id(&self) -> &PageId {
        &self.page_id
    }

    /// Returns the block type (e.g. `"text"`).
    pub fn block_type(&self) -> &str {
        &self.block_type
    }

    /// Returns a reference to the block's content.
    pub fn content(&self) -> &BlockContent {
        &self.content
    }

    /// Returns the block's position.
    pub fn position(&self) -> BlockPosition {
        self.position
    }

    /// Returns the block's creation timestamp.
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Returns the block's last-updated timestamp.
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// Updates the block's content.
    pub(crate) fn set_content(&mut self, content: BlockContent) {
        self.content = content;
    }

    /// Updates the block's position.
    pub(crate) fn set_position(&mut self, position: BlockPosition) {
        self.position = position;
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

    // T004: BlockContent::try_from() tests

    #[test]
    fn block_content_empty_string_accepted() {
        let content = BlockContent::try_from(String::new());
        assert!(content.is_ok());
        assert_eq!(content.unwrap().as_str(), "");
    }

    #[test]
    fn block_content_10000_chars_accepted() {
        let s = "a".repeat(10_000);
        let content = BlockContent::try_from(s);
        assert!(content.is_ok());
    }

    #[test]
    fn block_content_10001_chars_rejected() {
        let s = "a".repeat(10_001);
        let result = BlockContent::try_from(s);
        assert!(matches!(
            result,
            Err(BlockError::ContentTooLong {
                len: 10_001,
                max: 10_000
            })
        ));
    }

    // T004: BlockPosition::try_from() tests

    #[test]
    fn block_position_zero_accepted() {
        let pos = BlockPosition::try_from(0_i64);
        assert!(pos.is_ok());
        assert_eq!(pos.unwrap().value(), 0);
    }

    #[test]
    fn block_position_positive_accepted() {
        let pos = BlockPosition::try_from(42_i64);
        assert!(pos.is_ok());
        assert_eq!(pos.unwrap().value(), 42);
    }

    #[test]
    fn block_position_negative_rejected() {
        let result = BlockPosition::try_from(-1_i64);
        assert!(matches!(
            result,
            Err(BlockError::InvalidPosition { value: -1 })
        ));
    }

    // T039: Multi-byte Unicode tests for BlockContent

    #[test]
    fn block_content_multibyte_emoji_counted_correctly() {
        // Each emoji is 1 char in Rust's chars().count()
        let emoji_10000 = "\u{1F600}".repeat(10_000); // 😀 x 10,000
        let result = BlockContent::try_from(emoji_10000);
        assert!(result.is_ok());
    }

    #[test]
    fn block_content_multibyte_emoji_10001_rejected() {
        let emoji_10001 = "\u{1F600}".repeat(10_001);
        let result = BlockContent::try_from(emoji_10001);
        assert!(matches!(
            result,
            Err(BlockError::ContentTooLong {
                len: 10_001,
                max: 10_000
            })
        ));
    }

    #[test]
    fn block_content_mixed_bmp_and_supplementary() {
        // Mix of ASCII, CJK, and emoji
        let mixed = format!(
            "{}{}{}",
            "a".repeat(5_000),
            "\u{4e00}".repeat(3_000),
            "\u{1F600}".repeat(2_000)
        );
        assert_eq!(mixed.chars().count(), 10_000);
        let result = BlockContent::try_from(mixed);
        assert!(result.is_ok());
    }

    // BlockId tests

    #[test]
    fn block_id_display_and_from_str_roundtrip() {
        let id = BlockId::new();
        let s = id.to_string();
        let parsed: BlockId = s.parse().expect("should parse");
        assert_eq!(id, parsed);
    }
}
