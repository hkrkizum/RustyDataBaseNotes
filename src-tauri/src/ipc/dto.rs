use serde::Serialize;

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
