// The `#[tauri::command]` macro generates `unreachable!()` in its expansion.
#![allow(clippy::unreachable)]

use tauri::State;

use crate::AppState;
use crate::domain::database::entity::DatabaseId;
use crate::domain::view::entity::View;
use crate::domain::view::repository::ViewRepository;
use crate::infrastructure::persistence::view_repository::SqlxViewRepository;
use crate::ipc::dto::ViewDto;
use crate::ipc::error::CommandError;

/// Helper: find or create a default view for the given database.
async fn find_or_create_view(
    pool: &sqlx::SqlitePool,
    database_id: &DatabaseId,
) -> Result<View, CommandError> {
    let repo = SqlxViewRepository::new(pool.clone());
    match repo.find_by_database_id(database_id).await? {
        Some(view) => Ok(view),
        None => {
            let view = View::new_default(database_id.clone());
            repo.save(&view).await?;
            Ok(view)
        }
    }
}

/// Returns the view settings for a database.
#[tauri::command]
pub async fn get_view(
    state: State<'_, AppState>,
    database_id: String,
) -> Result<ViewDto, CommandError> {
    let db_id: DatabaseId = database_id.parse().map_err(|_| {
        crate::domain::database::error::DatabaseError::NotFound {
            id: DatabaseId::new(),
        }
    })?;

    let view = find_or_create_view(&state.db, &db_id).await?;
    Ok(ViewDto::from(&view))
}

/// Resets the view settings for a database to defaults.
#[tauri::command]
pub async fn reset_view(
    state: State<'_, AppState>,
    database_id: String,
) -> Result<ViewDto, CommandError> {
    let db_id: DatabaseId = database_id.parse().map_err(|_| {
        crate::domain::database::error::DatabaseError::NotFound {
            id: DatabaseId::new(),
        }
    })?;

    // Ensure view exists
    find_or_create_view(&state.db, &db_id).await?;

    let repo = SqlxViewRepository::new(state.db.clone());
    let view = repo.reset(&db_id).await?;
    Ok(ViewDto::from(&view))
}
