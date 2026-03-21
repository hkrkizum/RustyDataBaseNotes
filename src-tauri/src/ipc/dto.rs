use std::collections::HashMap;

use serde::Serialize;

use crate::domain::block::entity::Block;
use crate::domain::database::entity::Database;
use crate::domain::editor::session::EditorSession;
use crate::domain::page::entity::Page;
use crate::domain::property::entity::{Property, PropertyConfig, PropertyType, PropertyValue};

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
    /// Optional parent database ID.
    pub database_id: Option<String>,
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
            database_id: page.database_id().map(|id| id.to_string()),
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

/// Data transfer object for [`Database`].
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseDto {
    /// UUIDv7 identifier.
    pub id: String,
    /// Database title.
    pub title: String,
    /// ISO 8601 creation timestamp.
    pub created_at: String,
    /// ISO 8601 last-updated timestamp.
    pub updated_at: String,
}

impl From<Database> for DatabaseDto {
    fn from(db: Database) -> Self {
        Self {
            id: db.id().to_string(),
            title: db.title().to_string(),
            created_at: db.created_at().to_rfc3339(),
            updated_at: db.updated_at().to_rfc3339(),
        }
    }
}

/// Data transfer object for [`Property`].
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDto {
    /// UUIDv7 identifier.
    pub id: String,
    /// Parent database ID.
    pub database_id: String,
    /// Property name.
    pub name: String,
    /// Property type (text, number, date, select, checkbox).
    pub property_type: PropertyType,
    /// Type-specific configuration (null if none).
    pub config: Option<PropertyConfig>,
    /// Display position (0-based).
    pub position: i64,
    /// ISO 8601 creation timestamp.
    pub created_at: String,
    /// ISO 8601 last-updated timestamp.
    pub updated_at: String,
}

impl From<Property> for PropertyDto {
    fn from(p: Property) -> Self {
        Self {
            id: p.id().to_string(),
            database_id: p.database_id().to_string(),
            name: p.name().to_string(),
            property_type: p.property_type(),
            config: p.config().cloned(),
            position: p.position(),
            created_at: p.created_at().to_rfc3339(),
            updated_at: p.updated_at().to_rfc3339(),
        }
    }
}

/// Data transfer object for [`PropertyValue`].
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyValueDto {
    /// UUIDv7 identifier.
    pub id: String,
    /// Page ID.
    pub page_id: String,
    /// Property ID.
    pub property_id: String,
    /// Text value (for Text and Select types).
    pub text_value: Option<String>,
    /// Number value.
    pub number_value: Option<f64>,
    /// Date value (RFC 3339).
    pub date_value: Option<String>,
    /// Boolean value.
    pub boolean_value: Option<bool>,
    /// ISO 8601 creation timestamp.
    pub created_at: String,
    /// ISO 8601 last-updated timestamp.
    pub updated_at: String,
}

impl From<PropertyValue> for PropertyValueDto {
    fn from(pv: PropertyValue) -> Self {
        Self {
            id: pv.id().to_string(),
            page_id: pv.page_id().to_string(),
            property_id: pv.property_id().to_string(),
            text_value: pv.text_value().map(|s| s.to_owned()),
            number_value: pv.number_value(),
            date_value: pv.date_value().map(|d| d.to_rfc3339()),
            boolean_value: pv.boolean_value(),
            created_at: pv.created_at().to_rfc3339(),
            updated_at: pv.updated_at().to_rfc3339(),
        }
    }
}

/// Data transfer object for a table row (page with its property values).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableRowDto {
    /// The page data.
    pub page: PageDto,
    /// Property values keyed by property ID.
    pub values: HashMap<String, PropertyValueDto>,
}

/// Data transfer object for the full table view data.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableDataDto {
    /// The database.
    pub database: DatabaseDto,
    /// Properties (columns) in position order.
    pub properties: Vec<PropertyDto>,
    /// Table rows (pages with values).
    pub rows: Vec<TableRowDto>,
}
