// The `#[tauri::command]` macro generates `unreachable!()` in its expansion.
#![allow(clippy::unreachable)]

use tauri::State;

use crate::AppState;
use crate::domain::database::entity::DatabaseId;
use crate::domain::database::repository::DatabaseRepository;
use crate::domain::page::entity::{Page, PageId, PageTitle};
use crate::domain::page::error::PageError;
use crate::domain::page::repository::PageRepository;
use crate::infrastructure::persistence::database_repository::SqlxDatabaseRepository;
use crate::infrastructure::persistence::page_repository::SqlxPageRepository;
use crate::ipc::dto::PageDto;
use crate::ipc::error::CommandError;

/// Creates a new page and adds it to a database.
#[tauri::command]
pub async fn add_page_to_database(
    state: State<'_, AppState>,
    database_id: String,
    title: String,
) -> Result<PageDto, CommandError> {
    let db_id: DatabaseId = database_id.parse().map_err(|_| {
        crate::domain::database::error::DatabaseError::NotFound {
            id: DatabaseId::new(),
        }
    })?;

    // Verify database exists
    let db_repo = SqlxDatabaseRepository::new(state.db.clone());
    db_repo.find_by_id(&db_id).await?;

    // Create a new page
    let page_title = PageTitle::try_from(title)?;
    let page = Page::new(page_title);
    let page_id = page.id().clone();

    let page_repo = SqlxPageRepository::new(state.db.clone());
    page_repo.create(&page).await?;

    // Set database_id
    page_repo.set_database_id(&page_id, Some(&db_id)).await?;

    // Refetch to get updated data with database_id
    let updated = page_repo.find_by_id(&page_id).await?;
    Ok(PageDto::from(updated))
}

/// Adds an existing standalone page to a database.
#[tauri::command]
pub async fn add_existing_page_to_database(
    state: State<'_, AppState>,
    database_id: String,
    page_id: String,
) -> Result<PageDto, CommandError> {
    let db_id: DatabaseId = database_id.parse().map_err(|_| {
        crate::domain::database::error::DatabaseError::NotFound {
            id: DatabaseId::new(),
        }
    })?;
    let pg_id: PageId = page_id
        .parse()
        .map_err(|_| PageError::NotFound { id: PageId::new() })?;

    // Verify database exists
    let db_repo = SqlxDatabaseRepository::new(state.db.clone());
    db_repo.find_by_id(&db_id).await?;

    // Find page and check it's not already in any database
    let page_repo = SqlxPageRepository::new(state.db.clone());
    let page = page_repo.find_by_id(&pg_id).await?;

    if let Some(existing_db_id) = page.database_id() {
        return Err(PageError::AlreadyInDatabase {
            page_id: pg_id,
            database_id: existing_db_id.clone(),
        }
        .into());
    }

    // Set database_id
    page_repo.set_database_id(&pg_id, Some(&db_id)).await?;

    // Return updated page
    let updated = page_repo.find_by_id(&pg_id).await?;
    Ok(PageDto::from(updated))
}

/// Returns all pages not belonging to any database.
#[tauri::command]
pub async fn list_standalone_pages(
    state: State<'_, AppState>,
) -> Result<Vec<PageDto>, CommandError> {
    let repo = SqlxPageRepository::new(state.db.clone());
    let pages = repo.find_standalone_pages().await?;
    Ok(pages.into_iter().map(PageDto::from).collect())
}
