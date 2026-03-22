use std::collections::HashMap;

use serde::Serialize;

use crate::domain::block::entity::Block;
use crate::domain::database::entity::Database;
use crate::domain::editor::session::EditorSession;
use crate::domain::page::entity::Page;
use crate::domain::property::entity::{Property, PropertyConfig, PropertyType, PropertyValue};
use crate::domain::view::entity::{
    FilterCondition, FilterOperator, FilterValue, GroupCondition, SortCondition, SortDirection,
    View, ViewType,
};

/// Data transfer object for [`Page`], serialized to the frontend via IPC.
///
/// Field names are converted to camelCase for TypeScript consumption.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageDto {
    /// UUIDv7 identifier.
    pub id: String,
    /// Page title.
    pub title: String,
    /// Optional parent database ID.
    pub database_id: Option<String>,
    /// Optional parent page ID for hierarchy.
    pub parent_id: Option<String>,
    /// Sort order within the same parent (default 0).
    pub sort_order: i64,
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
            parent_id: page.parent_id().map(|id| id.to_string()),
            sort_order: page.sort_order(),
            created_at: page.created_at().to_rfc3339(),
            updated_at: page.updated_at().to_rfc3339(),
        }
    }
}

/// Data transfer object for sidebar items (pages and databases).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SidebarItemDto {
    /// UUIDv7 identifier.
    pub id: String,
    /// Display title.
    pub title: String,
    /// Whether this item is a page or a database.
    pub item_type: SidebarItemType,
    /// Parent page ID (for standalone pages with a parent).
    pub parent_id: Option<String>,
    /// Database ID (for database-owned pages).
    pub database_id: Option<String>,
    /// ISO 8601 creation timestamp.
    pub created_at: String,
}

/// The type of a sidebar item.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SidebarItemType {
    /// A standalone page.
    Page,
    /// A database.
    Database,
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
/// Contains the full block list for a page session.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorStateDto {
    /// Page ID.
    pub page_id: String,
    /// All blocks in position order.
    pub blocks: Vec<BlockDto>,
}

impl EditorStateDto {
    /// Creates an [`EditorStateDto`] from an [`EditorSession`] reference.
    pub fn from_session(session: &EditorSession) -> Self {
        Self {
            page_id: session.page_id().to_string(),
            blocks: session.blocks().iter().map(BlockDto::from).collect(),
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
#[derive(Debug, Clone, Serialize)]
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
#[derive(Debug, Clone, Serialize)]
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
    /// Current view settings.
    pub view: ViewDto,
    /// Grouping info (null if no grouping).
    pub groups: Option<Vec<GroupInfoDto>>,
}

// ---------------------------------------------------------------------------
// View DTOs
// ---------------------------------------------------------------------------

/// Data transfer object for [`View`].
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewDto {
    /// UUIDv7 identifier.
    pub id: String,
    /// Parent database ID.
    pub database_id: String,
    /// View name.
    pub name: String,
    /// View type.
    pub view_type: ViewType,
    /// Sort conditions in priority order.
    pub sort_conditions: Vec<SortConditionDto>,
    /// Filter conditions (AND combination).
    pub filter_conditions: Vec<FilterConditionDto>,
    /// Group condition (null if none).
    pub group_condition: Option<GroupConditionDto>,
    /// Collapsed group values.
    pub collapsed_groups: Vec<String>,
    /// ISO 8601 creation timestamp.
    pub created_at: String,
    /// ISO 8601 last-updated timestamp.
    pub updated_at: String,
}

impl From<&View> for ViewDto {
    fn from(view: &View) -> Self {
        Self {
            id: view.id().to_string(),
            database_id: view.database_id().to_string(),
            name: view.name().as_str().to_owned(),
            view_type: view.view_type(),
            sort_conditions: view
                .sort_conditions()
                .iter()
                .map(SortConditionDto::from)
                .collect(),
            filter_conditions: view
                .filter_conditions()
                .iter()
                .map(FilterConditionDto::from)
                .collect(),
            group_condition: view.group_condition().map(GroupConditionDto::from),
            collapsed_groups: view.collapsed_groups().iter().cloned().collect(),
            created_at: view.created_at().to_rfc3339(),
            updated_at: view.updated_at().to_rfc3339(),
        }
    }
}

/// Data transfer object for a sort condition.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SortConditionDto {
    /// Property ID.
    pub property_id: String,
    /// Sort direction.
    pub direction: SortDirection,
}

impl From<&SortCondition> for SortConditionDto {
    fn from(c: &SortCondition) -> Self {
        Self {
            property_id: c.property_id.to_string(),
            direction: c.direction,
        }
    }
}

/// Data transfer object for a filter condition.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterConditionDto {
    /// Property ID.
    pub property_id: String,
    /// Filter operator.
    pub operator: FilterOperator,
    /// Comparison value (null for IsEmpty/IsNotEmpty/IsChecked/IsUnchecked).
    pub value: Option<FilterValueDto>,
}

impl From<&FilterCondition> for FilterConditionDto {
    fn from(c: &FilterCondition) -> Self {
        Self {
            property_id: c.property_id.to_string(),
            operator: c.operator,
            value: c.value.as_ref().map(FilterValueDto::from),
        }
    }
}

/// Data transfer object for a filter value.
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum FilterValueDto {
    /// Text comparison value.
    Text(String),
    /// Numeric comparison value.
    Number(f64),
    /// Date comparison value (ISO 8601).
    Date(String),
    /// Select option comparison value.
    SelectOption(String),
}

impl From<&FilterValue> for FilterValueDto {
    fn from(v: &FilterValue) -> Self {
        match v {
            FilterValue::Text(s) => Self::Text(s.clone()),
            FilterValue::Number(n) => Self::Number(*n),
            FilterValue::Date(d) => Self::Date(d.clone()),
            FilterValue::SelectOption(s) => Self::SelectOption(s.clone()),
        }
    }
}

/// Data transfer object for a group condition.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupConditionDto {
    /// Property ID.
    pub property_id: String,
}

impl From<&GroupCondition> for GroupConditionDto {
    fn from(c: &GroupCondition) -> Self {
        Self {
            property_id: c.property_id.to_string(),
        }
    }
}

/// Data transfer object for group information in table data response.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupInfoDto {
    /// Group value (null for the "未設定" group).
    pub value: Option<String>,
    /// Display label for the group.
    pub display_value: String,
    /// Number of rows in this group.
    pub count: usize,
    /// Whether this group is collapsed.
    pub is_collapsed: bool,
}
