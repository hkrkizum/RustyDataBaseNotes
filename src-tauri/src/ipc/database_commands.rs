// The `#[tauri::command]` macro generates `unreachable!()` in its expansion.
#![allow(clippy::unreachable)]

use tauri::State;

use crate::AppState;
use crate::domain::database::entity::{Database, DatabaseId, DatabaseTitle};
use crate::domain::database::repository::DatabaseRepository;
use crate::infrastructure::persistence::database_repository::SqlxDatabaseRepository;
use crate::ipc::dto::DatabaseDto;
use crate::ipc::error::CommandError;

/// Creates a new database with the given title.
#[tauri::command]
pub async fn create_database(
    state: State<'_, AppState>,
    title: String,
) -> Result<DatabaseDto, CommandError> {
    let title = DatabaseTitle::try_from(title)?;
    let database = Database::new(title);
    let repo = SqlxDatabaseRepository::new(state.db.clone());
    repo.create(&database).await?;
    Ok(DatabaseDto::from(database))
}

/// Returns all databases ordered by creation date (newest first).
#[tauri::command]
pub async fn list_databases(state: State<'_, AppState>) -> Result<Vec<DatabaseDto>, CommandError> {
    let repo = SqlxDatabaseRepository::new(state.db.clone());
    let databases = repo.find_all().await?;
    Ok(databases.into_iter().map(DatabaseDto::from).collect())
}

/// Updates the title of an existing database.
#[tauri::command]
pub async fn update_database_title(
    state: State<'_, AppState>,
    id: String,
    title: String,
) -> Result<DatabaseDto, CommandError> {
    let db_id: DatabaseId =
        id.parse().map_err(
            |_| crate::domain::database::error::DatabaseError::NotFound {
                id: DatabaseId::new(),
            },
        )?;
    let new_title = DatabaseTitle::try_from(title)?;
    let repo = SqlxDatabaseRepository::new(state.db.clone());
    let database = repo.update_title(&db_id, &new_title).await?;
    Ok(DatabaseDto::from(database))
}

/// Deletes a database by its ID.
#[tauri::command]
pub async fn delete_database(state: State<'_, AppState>, id: String) -> Result<(), CommandError> {
    let db_id: DatabaseId =
        id.parse().map_err(
            |_| crate::domain::database::error::DatabaseError::NotFound {
                id: DatabaseId::new(),
            },
        )?;
    let repo = SqlxDatabaseRepository::new(state.db.clone());
    repo.delete(&db_id).await?;
    Ok(())
}

/// Returns a single database by its ID.
#[tauri::command]
pub async fn get_database(
    state: State<'_, AppState>,
    id: String,
) -> Result<DatabaseDto, CommandError> {
    let db_id: DatabaseId = id.parse().map_err(|_| {
        crate::domain::database::error::DatabaseError::NotFound {
            id: DatabaseId::new(), // placeholder — the parse failed so we can't produce the original
        }
    })?;
    let repo = SqlxDatabaseRepository::new(state.db.clone());
    let database = repo.find_by_id(&db_id).await?;
    Ok(DatabaseDto::from(database))
}
