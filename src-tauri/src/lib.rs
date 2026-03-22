#![warn(missing_docs)]
//! RustyDataBaseNotes — a local-first notebook application built with Tauri.

/// Domain layer: entities, value objects, and domain services.
pub mod domain;
mod infrastructure;
mod ipc;

use std::collections::HashMap;

use sqlx::SqlitePool;
use tauri::Manager;

use crate::domain::editor::session::EditorSession;
use crate::domain::page::entity::PageId;
use crate::infrastructure::persistence::database;
use crate::ipc::database_commands::{
    create_database, delete_database, get_database, list_databases, update_database_title,
};
use crate::ipc::editor_commands::{
    add_block, close_editor, edit_block_content, move_block_down, move_block_up, open_editor,
    remove_block, save_editor,
};
use crate::ipc::page_commands::{
    create_page, delete_page, get_page, list_pages, list_sidebar_items, update_page_title,
};
use crate::ipc::property_commands::{
    add_property, clear_property_value, delete_property, list_properties, reorder_properties,
    reset_select_option, set_property_value, update_property_config, update_property_name,
};
use crate::ipc::table_commands::{
    add_existing_page_to_database, add_page_to_database, get_table_data, list_standalone_pages,
    remove_page_from_database,
};
use crate::ipc::view_commands::{
    get_view, reset_view, toggle_group_collapsed, update_filter_conditions, update_group_condition,
    update_sort_conditions,
};

/// Application-wide shared state managed by Tauri.
pub struct AppState {
    /// SQLite connection pool.
    pub db: SqlitePool,
    /// Active editor sessions, keyed by page ID.
    pub sessions: tokio::sync::Mutex<HashMap<PageId, EditorSession>>,
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
            create_database,
            list_databases,
            get_database,
            update_database_title,
            delete_database,
            create_page,
            list_pages,
            get_page,
            update_page_title,
            delete_page,
            list_sidebar_items,
            open_editor,
            close_editor,
            add_block,
            edit_block_content,
            move_block_up,
            move_block_down,
            remove_block,
            save_editor,
            add_property,
            list_properties,
            update_property_name,
            update_property_config,
            reorder_properties,
            delete_property,
            reset_select_option,
            set_property_value,
            clear_property_value,
            add_page_to_database,
            add_existing_page_to_database,
            list_standalone_pages,
            get_table_data,
            remove_page_from_database,
            get_view,
            reset_view,
            update_sort_conditions,
            update_filter_conditions,
            update_group_condition,
            toggle_group_collapsed,
        ])
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            let db_path = data_dir.join("rustydatabasenotes.db");

            let pool = tauri::async_runtime::block_on(database::init_pool(&db_path))?;

            app.manage(AppState {
                db: pool,
                sessions: tokio::sync::Mutex::new(HashMap::new()),
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
