// The `#[tauri::command]` macro generates `unreachable!()` in its expansion.
#![allow(clippy::unreachable)]

use sqlx::SqlitePool;
use tauri::State;

use crate::AppState;
use crate::domain::database::repository::DatabaseRepository;
use crate::domain::page::entity::{Page, PageId, PageTitle};
use crate::domain::page::repository::PageRepository;
use crate::infrastructure::persistence::database_repository::SqlxDatabaseRepository;
use crate::infrastructure::persistence::page_repository::SqlxPageRepository;
use crate::ipc::dto::{PageDto, SidebarItemDto, SidebarItemType};
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

/// Returns all sidebar items (standalone pages, databases, and DB-owned pages).
///
/// The frontend uses this to build the sidebar tree. Items are returned as a
/// flat list; the frontend constructs the tree using `parentId` and `databaseId`.
///
/// # Errors
///
/// Returns [`CommandError::Storage`] on database failures.
#[tauri::command]
pub async fn list_sidebar_items(
    state: State<'_, AppState>,
) -> Result<Vec<SidebarItemDto>, CommandError> {
    build_sidebar_items(state.db.clone()).await
}

/// Builds the flat list of sidebar items from the database.
///
/// Extracted from the command handler to enable testing without the Tauri
/// runtime.
///
/// # Errors
///
/// Returns [`CommandError`] on database or parse failures.
async fn build_sidebar_items(pool: SqlitePool) -> Result<Vec<SidebarItemDto>, CommandError> {
    let page_repo = SqlxPageRepository::new(pool.clone());
    let db_repo = SqlxDatabaseRepository::new(pool);

    let all_pages = page_repo.find_all().await?;
    let all_databases = db_repo.find_all().await?;

    let mut items = Vec::with_capacity(all_pages.len() + all_databases.len());

    for page in &all_pages {
        items.push(SidebarItemDto {
            id: page.id().to_string(),
            title: page.title().to_string(),
            item_type: SidebarItemType::Page,
            parent_id: page.parent_id().map(|id| id.to_string()),
            database_id: page.database_id().map(|id| id.to_string()),
            created_at: page.created_at().to_rfc3339(),
        });
    }

    for db in &all_databases {
        items.push(SidebarItemDto {
            id: db.id().to_string(),
            title: db.title().to_string(),
            item_type: SidebarItemType::Database,
            parent_id: None,
            database_id: None,
            created_at: db.created_at().to_rfc3339(),
        });
    }

    Ok(items)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use crate::domain::database::entity::{Database, DatabaseTitle};

    async fn setup_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:")
            .await
            .expect("in-memory pool");
        sqlx::migrate!().run(&pool).await.expect("migrations");
        pool
    }

    #[tokio::test]
    async fn list_sidebar_items_empty() {
        let pool = setup_pool().await;
        let items = build_sidebar_items(pool).await.expect("should succeed");
        assert!(items.is_empty());
    }

    #[tokio::test]
    async fn list_sidebar_items_returns_standalone_pages() {
        let pool = setup_pool().await;
        let page_repo = SqlxPageRepository::new(pool.clone());

        let title = PageTitle::try_from("Standalone Page".to_owned()).expect("valid");
        let page = Page::new(title);
        page_repo.create(&page).await.expect("create page");

        let items = build_sidebar_items(pool).await.expect("should succeed");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, "Standalone Page");
        assert!(
            matches!(items[0].item_type, SidebarItemType::Page),
            "standalone page should have item_type Page"
        );
        assert!(items[0].parent_id.is_none());
        assert!(items[0].database_id.is_none());
    }

    #[tokio::test]
    async fn list_sidebar_items_returns_databases() {
        let pool = setup_pool().await;
        let db_repo = SqlxDatabaseRepository::new(pool.clone());

        let title = DatabaseTitle::try_from("My Database".to_owned()).expect("valid");
        let database = Database::new(title);
        db_repo.create(&database).await.expect("create db");

        let items = build_sidebar_items(pool).await.expect("should succeed");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, "My Database");
        assert!(
            matches!(items[0].item_type, SidebarItemType::Database),
            "database should have item_type Database"
        );
        assert!(items[0].parent_id.is_none());
        assert!(items[0].database_id.is_none());
    }

    #[tokio::test]
    async fn list_sidebar_items_returns_db_owned_pages_with_database_id() {
        let pool = setup_pool().await;
        let page_repo = SqlxPageRepository::new(pool.clone());
        let db_repo = SqlxDatabaseRepository::new(pool.clone());

        // Create database
        let db_title = DatabaseTitle::try_from("DB".to_owned()).expect("valid");
        let database = Database::new(db_title);
        let db_id = database.id().clone();
        db_repo.create(&database).await.expect("create db");

        // Create page and assign to database
        let page_title = PageTitle::try_from("DB Page".to_owned()).expect("valid");
        let page = Page::new(page_title);
        let page_id = page.id().clone();
        page_repo.create(&page).await.expect("create page");
        page_repo
            .set_database_id(&page_id, Some(&db_id))
            .await
            .expect("set database_id");

        let items = build_sidebar_items(pool).await.expect("should succeed");
        // Should contain: the DB-owned page + the database = 2 items
        assert_eq!(items.len(), 2);

        let page_item = items
            .iter()
            .find(|i| matches!(i.item_type, SidebarItemType::Page))
            .expect("should have a page item");
        assert_eq!(page_item.title, "DB Page");
        assert_eq!(page_item.database_id, Some(db_id.to_string()));
    }

    #[tokio::test]
    async fn list_sidebar_items_mixed_content() {
        let pool = setup_pool().await;
        let page_repo = SqlxPageRepository::new(pool.clone());
        let db_repo = SqlxDatabaseRepository::new(pool.clone());

        // Create 2 standalone pages
        for title_str in ["Page A", "Page B"] {
            let title = PageTitle::try_from(title_str.to_owned()).expect("valid");
            let page = Page::new(title);
            page_repo.create(&page).await.expect("create page");
        }

        // Create 1 database with 1 DB-owned page
        let db_title = DatabaseTitle::try_from("Tasks DB".to_owned()).expect("valid");
        let database = Database::new(db_title);
        let db_id = database.id().clone();
        db_repo.create(&database).await.expect("create db");

        let db_page_title = PageTitle::try_from("Task Row".to_owned()).expect("valid");
        let db_page = Page::new(db_page_title);
        let db_page_id = db_page.id().clone();
        page_repo.create(&db_page).await.expect("create db page");
        page_repo
            .set_database_id(&db_page_id, Some(&db_id))
            .await
            .expect("set database_id");

        let items = build_sidebar_items(pool).await.expect("should succeed");
        // 2 standalone pages + 1 DB-owned page + 1 database = 4 items
        assert_eq!(items.len(), 4);

        let page_count = items
            .iter()
            .filter(|i| matches!(i.item_type, SidebarItemType::Page))
            .count();
        let db_count = items
            .iter()
            .filter(|i| matches!(i.item_type, SidebarItemType::Database))
            .count();
        assert_eq!(page_count, 3, "should have 3 page items");
        assert_eq!(db_count, 1, "should have 1 database item");
    }

    #[tokio::test]
    async fn list_sidebar_items_child_page_has_parent_id() {
        let pool = setup_pool().await;
        let page_repo = SqlxPageRepository::new(pool.clone());

        // Create parent page
        let parent_title = PageTitle::try_from("Parent".to_owned()).expect("valid");
        let parent = Page::new(parent_title);
        let parent_id = parent.id().clone();
        page_repo.create(&parent).await.expect("create parent");

        // Create child page
        let child_title = PageTitle::try_from("Child".to_owned()).expect("valid");
        let child = Page::new_child(child_title, parent_id.clone());
        page_repo.create(&child).await.expect("create child");

        let items = build_sidebar_items(pool).await.expect("should succeed");
        assert_eq!(items.len(), 2);

        let child_item = items
            .iter()
            .find(|i| i.title == "Child")
            .expect("should have child item");
        assert_eq!(child_item.parent_id, Some(parent_id.to_string()));
    }
}
