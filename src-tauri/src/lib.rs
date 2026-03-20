mod domain;
mod infrastructure;
mod ipc;

use sqlx::SqlitePool;
use tauri::Manager;

use crate::infrastructure::persistence::database;
use crate::ipc::page_commands::{
    create_page, delete_page, get_page, list_pages, update_page_title,
};

/// Application-wide shared state managed by Tauri.
pub struct AppState {
    /// SQLite connection pool.
    pub db: SqlitePool,
}

/// Runs the Tauri application.
///
/// # Errors
///
/// Returns an error if the Tauri builder fails to initialize.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[allow(clippy::expect_used)] // Tauri entry point – no recovery possible
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_page,
            list_pages,
            get_page,
            update_page_title,
            delete_page,
        ])
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            let db_path = data_dir.join("rustydatabasenotes.db");

            let pool = tauri::async_runtime::block_on(database::init_pool(&db_path))?;

            app.manage(AppState { db: pool });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
