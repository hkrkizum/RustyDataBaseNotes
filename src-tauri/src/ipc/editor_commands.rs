// The `#[tauri::command]` macro generates `unreachable!()` in its expansion.
#![allow(clippy::unreachable)]

use std::collections::HashMap;

use tauri::State;

use crate::AppState;
use crate::domain::block::entity::BlockId;
use crate::domain::editor::session::EditorSession;
use crate::domain::page::entity::PageId;
use crate::infrastructure::persistence::block_repository::{BlockRepository, SqlxBlockRepository};
use crate::ipc::dto::EditorStateDto;
use crate::ipc::error::CommandError;

/// Opens an editor session for the specified page.
///
/// Loads blocks from the database and creates an [`EditorSession`] stored
/// in [`AppState`]. If a session already exists for the page, returns it
/// without reloading.
///
/// # Errors
///
/// Returns [`CommandError::Storage`] on database failures.
#[tauri::command]
pub async fn open_editor(
    state: State<'_, AppState>,
    page_id: String,
) -> Result<EditorStateDto, CommandError> {
    let pid: PageId = page_id
        .parse()
        .map_err(|_| crate::domain::page::error::PageError::NotFound { id: PageId::new() })?;

    let mut sessions = state.sessions.lock().await;

    // Return existing session if already open
    if let Some(session) = sessions.get(&pid) {
        return Ok(EditorStateDto::from_session(session));
    }

    let repo = SqlxBlockRepository::new(state.db.clone());
    let blocks = repo.load_blocks(&pid).await?;
    let session = EditorSession::new(pid.clone(), blocks);
    let dto = EditorStateDto::from_session(&session);
    sessions.insert(pid, session);

    Ok(dto)
}

/// Closes an editor session, releasing it from memory.
///
/// Idempotent: does not error if no session exists for the page.
///
/// # Errors
///
/// Returns [`CommandError`] if the page ID cannot be parsed.
#[tauri::command]
pub async fn close_editor(state: State<'_, AppState>, page_id: String) -> Result<(), CommandError> {
    let pid: PageId = page_id
        .parse()
        .map_err(|_| crate::domain::page::error::PageError::NotFound { id: PageId::new() })?;

    let mut sessions = state.sessions.lock().await;
    sessions.remove(&pid);

    Ok(())
}

/// Adds a new empty text block at the end of the session's block list.
///
/// # Errors
///
/// Returns [`CommandError::Storage`] if no session exists for the page.
#[tauri::command]
pub async fn add_block(
    state: State<'_, AppState>,
    page_id: String,
) -> Result<EditorStateDto, CommandError> {
    let pid: PageId = page_id
        .parse()
        .map_err(|_| crate::domain::page::error::PageError::NotFound { id: PageId::new() })?;

    let mut sessions = state.sessions.lock().await;
    let session = get_session_mut(&mut sessions, &pid)?;
    session.add_block();

    Ok(EditorStateDto::from_session(session))
}

/// Updates the content of the specified block.
///
/// # Errors
///
/// - [`CommandError::Block`] with `ContentTooLong` if content exceeds 10,000 characters.
/// - [`CommandError::Block`] with `NotFound` if the block ID is not in the session.
/// - [`CommandError::Storage`] if no session exists for the page.
#[tauri::command]
pub async fn edit_block_content(
    state: State<'_, AppState>,
    page_id: String,
    block_id: String,
    content: String,
) -> Result<EditorStateDto, CommandError> {
    let pid: PageId = page_id
        .parse()
        .map_err(|_| crate::domain::page::error::PageError::NotFound { id: PageId::new() })?;
    let bid: BlockId =
        block_id
            .parse()
            .map_err(|_| crate::domain::block::error::BlockError::NotFound {
                id: block_id.clone(),
            })?;

    let mut sessions = state.sessions.lock().await;
    let session = get_session_mut(&mut sessions, &pid)?;
    session.edit_block_content(&bid, content)?;

    Ok(EditorStateDto::from_session(session))
}

/// Moves the specified block one position up.
///
/// # Errors
///
/// - [`CommandError::Block`] with `CannotMoveUp` if the block is at position 0.
/// - [`CommandError::Block`] with `NotFound` if the block ID is not in the session.
/// - [`CommandError::Storage`] if no session exists for the page.
#[tauri::command]
pub async fn move_block_up(
    state: State<'_, AppState>,
    page_id: String,
    block_id: String,
) -> Result<EditorStateDto, CommandError> {
    let pid: PageId = page_id
        .parse()
        .map_err(|_| crate::domain::page::error::PageError::NotFound { id: PageId::new() })?;
    let bid: BlockId =
        block_id
            .parse()
            .map_err(|_| crate::domain::block::error::BlockError::NotFound {
                id: block_id.clone(),
            })?;

    let mut sessions = state.sessions.lock().await;
    let session = get_session_mut(&mut sessions, &pid)?;
    session.move_block_up(&bid)?;

    Ok(EditorStateDto::from_session(session))
}

/// Moves the specified block one position down.
///
/// # Errors
///
/// - [`CommandError::Block`] with `CannotMoveDown` if the block is at the last position.
/// - [`CommandError::Block`] with `NotFound` if the block ID is not in the session.
/// - [`CommandError::Storage`] if no session exists for the page.
#[tauri::command]
pub async fn move_block_down(
    state: State<'_, AppState>,
    page_id: String,
    block_id: String,
) -> Result<EditorStateDto, CommandError> {
    let pid: PageId = page_id
        .parse()
        .map_err(|_| crate::domain::page::error::PageError::NotFound { id: PageId::new() })?;
    let bid: BlockId =
        block_id
            .parse()
            .map_err(|_| crate::domain::block::error::BlockError::NotFound {
                id: block_id.clone(),
            })?;

    let mut sessions = state.sessions.lock().await;
    let session = get_session_mut(&mut sessions, &pid)?;
    session.move_block_down(&bid)?;

    Ok(EditorStateDto::from_session(session))
}

/// Removes the specified block from the session.
///
/// # Errors
///
/// - [`CommandError::Block`] with `NotFound` if the block ID is not in the session.
/// - [`CommandError::Storage`] if no session exists for the page.
#[tauri::command]
pub async fn remove_block(
    state: State<'_, AppState>,
    page_id: String,
    block_id: String,
) -> Result<EditorStateDto, CommandError> {
    let pid: PageId = page_id
        .parse()
        .map_err(|_| crate::domain::page::error::PageError::NotFound { id: PageId::new() })?;
    let bid: BlockId =
        block_id
            .parse()
            .map_err(|_| crate::domain::block::error::BlockError::NotFound {
                id: block_id.clone(),
            })?;

    let mut sessions = state.sessions.lock().await;
    let session = get_session_mut(&mut sessions, &pid)?;
    session.remove_block(&bid)?;

    Ok(EditorStateDto::from_session(session))
}

/// Persists all blocks in the session to the database.
///
/// Skips the write if the session is not dirty. After a successful save,
/// the session's dirty flag is reset.
///
/// # Errors
///
/// Returns [`CommandError::Storage`] on database failures or if no session exists.
#[tauri::command]
pub async fn save_editor(
    state: State<'_, AppState>,
    page_id: String,
) -> Result<EditorStateDto, CommandError> {
    let pid: PageId = page_id
        .parse()
        .map_err(|_| crate::domain::page::error::PageError::NotFound { id: PageId::new() })?;

    let mut sessions = state.sessions.lock().await;
    let session = get_session_mut(&mut sessions, &pid)?;

    if !session.is_dirty() {
        return Ok(EditorStateDto::from_session(session));
    }

    let repo = SqlxBlockRepository::new(state.db.clone());
    repo.save_all(&pid, session.blocks()).await?;
    session.mark_saved();

    Ok(EditorStateDto::from_session(session))
}

/// Retrieves a mutable reference to the session for the given page.
///
/// # Errors
///
/// Returns [`CommandError::Storage`] if no session exists.
fn get_session_mut<'a>(
    sessions: &'a mut HashMap<PageId, EditorSession>,
    page_id: &PageId,
) -> Result<&'a mut EditorSession, CommandError> {
    sessions.get_mut(page_id).ok_or_else(|| {
        CommandError::Storage(
            crate::infrastructure::persistence::error::StorageError::Sqlx(sqlx::Error::RowNotFound),
        )
    })
}
