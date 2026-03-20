// The `#[tauri::command]` macro generates `unreachable!()` in its expansion.
#![allow(clippy::unreachable)]

use tauri::State;

use crate::AppState;
use crate::domain::page::entity::{Page, PageId, PageTitle};
use crate::domain::page::repository::PageRepository;
use crate::infrastructure::persistence::page_repository::SqlxPageRepository;
use crate::ipc::dto::PageDto;
use crate::ipc::error::CommandError;

/// Creates a new page with the given title.
#[tauri::command]
pub async fn create_page(
    state: State<'_, AppState>,
    title: String,
) -> Result<PageDto, CommandError> {
    let title = PageTitle::try_from(title)?;
    let page = Page::new(title);
    let repo = SqlxPageRepository::new(state.db.clone());
    repo.create(&page).await?;
    Ok(PageDto::from(page))
}

/// Returns all pages ordered by creation date (newest first).
#[tauri::command]
pub async fn list_pages(state: State<'_, AppState>) -> Result<Vec<PageDto>, CommandError> {
    let repo = SqlxPageRepository::new(state.db.clone());
    let pages = repo.find_all().await?;
    Ok(pages.into_iter().map(PageDto::from).collect())
}

/// Returns a single page by its ID.
#[tauri::command]
pub async fn get_page(state: State<'_, AppState>, id: String) -> Result<PageDto, CommandError> {
    let page_id: PageId =
        id.parse()
            .map_err(|_| crate::domain::page::error::PageError::NotFound {
                id: PageId::new(), // placeholder — the parse failed so we can't produce the original
            })?;
    let repo = SqlxPageRepository::new(state.db.clone());
    let page = repo.find_by_id(&page_id).await?;
    Ok(PageDto::from(page))
}

/// Updates the title of an existing page.
#[tauri::command]
pub async fn update_page_title(
    state: State<'_, AppState>,
    id: String,
    title: String,
) -> Result<PageDto, CommandError> {
    let page_id: PageId = id
        .parse()
        .map_err(|_| crate::domain::page::error::PageError::NotFound { id: PageId::new() })?;
    let new_title = PageTitle::try_from(title)?;
    let repo = SqlxPageRepository::new(state.db.clone());
    let page = repo.update_title(&page_id, &new_title).await?;
    Ok(PageDto::from(page))
}

/// Deletes a page by its ID.
#[tauri::command]
pub async fn delete_page(state: State<'_, AppState>, id: String) -> Result<(), CommandError> {
    let page_id: PageId = id
        .parse()
        .map_err(|_| crate::domain::page::error::PageError::NotFound { id: PageId::new() })?;
    let repo = SqlxPageRepository::new(state.db.clone());
    repo.delete(&page_id).await?;
    Ok(())
}
