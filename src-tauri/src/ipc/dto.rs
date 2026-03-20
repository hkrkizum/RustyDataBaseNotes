use serde::Serialize;

use crate::domain::block::entity::Block;
use crate::domain::editor::session::EditorSession;
use crate::domain::page::entity::Page;

/// Data transfer object for [`Page`], serialized to the frontend via IPC.
///
/// Field names are converted to camelCase for TypeScript consumption.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageDto {
    /// UUIDv7 identifier.
    pub id: String,
    /// Page title.
    pub title: String,
    /// ISO 8601 creation timestamp.
    pub created_at: String,
    /// ISO 8601 last-updated timestamp.
    pub updated_at: String,
}

impl From<Page> for PageDto {
    fn from(page: Page) -> Self {
        Self {
            id: page.id().to_string(),
            title: page.title().to_string(),
            created_at: page.created_at().to_rfc3339(),
            updated_at: page.updated_at().to_rfc3339(),
        }
    }
}

/// Data transfer object for a single [`Block`].
///
/// Field names are converted to camelCase for TypeScript consumption.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockDto {
    /// UUIDv7 identifier.
    pub id: String,
    /// Parent page ID.
    pub page_id: String,
    /// Block type (e.g. `"text"`).
    pub block_type: String,
    /// Block content.
    pub content: String,
    /// Display position (0-based).
    pub position: i64,
    /// ISO 8601 creation timestamp.
    pub created_at: String,
    /// ISO 8601 last-updated timestamp.
    pub updated_at: String,
}

impl From<&Block> for BlockDto {
    fn from(block: &Block) -> Self {
        Self {
            id: block.id().to_string(),
            page_id: block.page_id().to_string(),
            block_type: block.block_type().to_owned(),
            content: block.content().to_string(),
            position: block.position().value(),
            created_at: block.created_at().to_rfc3339(),
            updated_at: block.updated_at().to_rfc3339(),
        }
    }
}

/// Data transfer object for the editor state returned to the frontend.
///
/// Contains the full block list and dirty state for a page session.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorStateDto {
    /// Page ID.
    pub page_id: String,
    /// All blocks in position order.
    pub blocks: Vec<BlockDto>,
    /// Whether the session has unsaved changes.
    pub is_dirty: bool,
}

impl EditorStateDto {
    /// Creates an [`EditorStateDto`] from an [`EditorSession`] reference.
    pub fn from_session(session: &EditorSession) -> Self {
        Self {
            page_id: session.page_id().to_string(),
            blocks: session.blocks().iter().map(BlockDto::from).collect(),
            is_dirty: session.is_dirty(),
        }
    }
}
