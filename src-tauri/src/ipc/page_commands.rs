// The `#[tauri::command]` macro generates `unreachable!()` in its expansion.
#![allow(clippy::unreachable)]

use sqlx::SqlitePool;
use tauri::State;

use crate::AppState;
use crate::domain::database::repository::DatabaseRepository;
use crate::domain::page::entity::{Page, PageId, PageTitle};
use crate::domain::page::hierarchy::PageHierarchyService;
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

/// Deletes a page by its ID, promoting children to the deleted page's parent.
///
/// Before deletion, any child pages are reparented to the deleted page's
/// own parent (or root if the deleted page was root-level). This ensures
/// no child data is lost.
///
/// # Errors
///
/// Returns [`CommandError::Page`] with [`PageError::NotFound`] if the page does not exist.
#[tauri::command]
pub async fn delete_page(state: State<'_, AppState>, id: String) -> Result<(), CommandError> {
    let page_id: PageId = id
        .parse()
        .map_err(|_| crate::domain::page::error::PageError::NotFound { id: PageId::new() })?;
    let repo = SqlxPageRepository::new(state.db.clone());
    build_delete_page_with_promotion(repo, page_id).await
}

/// Creates a child page under an existing parent page.
///
/// Validates that the parent exists, is not a database page, and that
/// the resulting depth does not exceed [`MAX_DEPTH`].
///
/// # Errors
///
/// - [`CommandError::Page`] with [`PageError::NotFound`] if the parent does not exist.
/// - [`CommandError::Page`] with [`PageError::DatabasePageCannotNest`] if the parent is a database page.
/// - [`CommandError::Page`] with [`PageError::MaxDepthExceeded`] if depth would exceed the limit.
#[tauri::command]
pub async fn create_child_page(
    state: State<'_, AppState>,
    parent_id: String,
    title: String,
) -> Result<PageDto, CommandError> {
    let parent_page_id: PageId = parent_id
        .parse()
        .map_err(|_| crate::domain::page::error::PageError::NotFound { id: PageId::new() })?;
    let title = PageTitle::try_from(title)?;
    let repo = SqlxPageRepository::new(state.db.clone());

    build_create_child_page(repo, parent_page_id, title).await
}

/// Moves a page to a new parent, or promotes it to root level.
///
/// When `new_parent_id` is `Some`, validates circular reference and depth
/// constraints. When `None`, promotes the page to root level.
///
/// # Errors
///
/// - [`CommandError::Page`] with [`PageError::NotFound`] if the page or new parent does not exist.
/// - [`CommandError::Page`] with [`PageError::DatabasePageCannotNest`] if either page is database-owned.
/// - [`CommandError::Page`] with [`PageError::CircularReference`] if the move would create a cycle.
/// - [`CommandError::Page`] with [`PageError::MaxDepthExceeded`] if depth would exceed the limit.
#[tauri::command]
pub async fn move_page(
    state: State<'_, AppState>,
    page_id: String,
    new_parent_id: Option<String>,
) -> Result<PageDto, CommandError> {
    let page_id: PageId = page_id
        .parse()
        .map_err(|_| crate::domain::page::error::PageError::NotFound { id: PageId::new() })?;
    let new_parent_id = new_parent_id
        .map(|s| {
            s.parse::<PageId>()
                .map_err(|_| crate::domain::page::error::PageError::NotFound { id: PageId::new() })
        })
        .transpose()?;
    let repo = SqlxPageRepository::new(state.db.clone());

    build_move_page(repo, page_id, new_parent_id).await
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

/// Creates a child page under an existing parent, with hierarchy validation.
///
/// Extracted from the command handler to enable testing without the Tauri runtime.
///
/// # Errors
///
/// Returns [`CommandError`] on validation or storage failures.
async fn build_create_child_page(
    repo: SqlxPageRepository,
    parent_page_id: PageId,
    title: PageTitle,
) -> Result<PageDto, CommandError> {
    let parent = repo.find_by_id(&parent_page_id).await?;

    // Compute parent depth using ancestors from repository.
    let ancestors = repo.find_ancestors(&parent_page_id).await?;
    let parent_depth = ancestors.len() + 1; // ancestors don't include the page itself

    PageHierarchyService::validate_create_child(&parent, parent_depth)?;

    let child = Page::new_child(title, parent_page_id);
    repo.create(&child).await?;
    Ok(PageDto::from(child))
}

/// Moves a page to a new parent or promotes it to root level.
///
/// Extracted from the command handler to enable testing without the Tauri runtime.
///
/// # Errors
///
/// Returns [`CommandError`] on validation or storage failures.
async fn build_move_page(
    repo: SqlxPageRepository,
    page_id: PageId,
    new_parent_id: Option<PageId>,
) -> Result<PageDto, CommandError> {
    let page = repo.find_by_id(&page_id).await?;

    if let Some(ref target_parent_id) = new_parent_id {
        // Ensure new parent exists and is not a DB page.
        let target_parent = repo.find_by_id(target_parent_id).await?;
        if target_parent.is_database_page() {
            return Err(
                crate::domain::page::error::PageError::DatabasePageCannotNest {
                    page_id: target_parent_id.to_string(),
                }
                .into(),
            );
        }

        // Load ancestors of the target parent for circular reference check.
        let ancestor_pages = repo.find_ancestors(target_parent_id).await?;
        let ancestors_of_target: Vec<PageId> =
            ancestor_pages.iter().map(|p| p.id().clone()).collect();

        // Load all pages reachable from `page` for max_descendant_depth.
        let all_pages = repo.find_all().await?;
        let max_desc_depth = PageHierarchyService::max_descendant_depth(page.id(), &all_pages);

        PageHierarchyService::validate_move(
            &page,
            Some(target_parent_id),
            &ancestors_of_target,
            max_desc_depth,
        )?;
    } else {
        // Moving to root — only need to check the page is not a DB page.
        if page.is_database_page() {
            return Err(
                crate::domain::page::error::PageError::DatabasePageCannotNest {
                    page_id: page.id().to_string(),
                }
                .into(),
            );
        }
    }

    let updated = repo
        .update_parent_id(&page_id, new_parent_id.as_ref())
        .await?;
    Ok(PageDto::from(updated))
}

/// Deletes a page after promoting its children to the page's parent.
///
/// Extracted from the command handler to enable testing without the Tauri runtime.
///
/// # Errors
///
/// Returns [`CommandError`] on storage failures.
async fn build_delete_page_with_promotion(
    repo: SqlxPageRepository,
    page_id: PageId,
) -> Result<(), CommandError> {
    let page = repo.find_by_id(&page_id).await?;
    let promotion_target = page.parent_id().cloned();
    let children = repo.find_children(&page_id).await?;

    if !children.is_empty() {
        let child_ids: Vec<PageId> = children.iter().map(|c| c.id().clone()).collect();
        repo.bulk_update_parent_id(&child_ids, promotion_target.as_ref())
            .await?;
    }

    repo.delete(&page_id).await?;
    Ok(())
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

    async fn create_test_database(pool: &SqlitePool) -> String {
        let id = uuid::Uuid::now_v7().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query!(
            "INSERT INTO databases (id, title, created_at, updated_at) VALUES (?, ?, ?, ?)",
            id,
            "Test DB",
            now,
            now
        )
        .execute(pool)
        .await
        .expect("create test database");
        id
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

    // ---------------------------------------------------------------
    // T044: Integration tests for hierarchy IPC commands
    // ---------------------------------------------------------------

    #[tokio::test]
    async fn create_child_page_success() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool);

        let parent = Page::new(PageTitle::try_from("Parent".to_owned()).expect("valid"));
        let parent_id = parent.id().clone();
        repo.create(&parent).await.expect("create parent");

        let result = build_create_child_page(
            repo,
            parent_id.clone(),
            PageTitle::try_from("Child".to_owned()).expect("valid"),
        )
        .await;

        assert!(result.is_ok());
        let child_dto = result.expect("should succeed");
        assert_eq!(child_dto.title, "Child");
        assert_eq!(child_dto.parent_id, Some(parent_id.to_string()));
    }

    #[tokio::test]
    async fn create_child_page_max_depth_exceeded() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool);

        // Build a chain of depth 5 (root → c1 → c2 → c3 → c4).
        let root = Page::new(PageTitle::try_from("Root".to_owned()).expect("valid"));
        repo.create(&root).await.expect("create root");
        let mut current_id = root.id().clone();
        for i in 1..5 {
            let child = Page::new_child(
                PageTitle::try_from(format!("Level {i}")).expect("valid"),
                current_id.clone(),
            );
            let child_id = child.id().clone();
            repo.create(&child).await.expect("create child");
            current_id = child_id;
        }

        // current_id is at depth 5 — creating a child should fail.
        let result = build_create_child_page(
            repo,
            current_id,
            PageTitle::try_from("Too Deep".to_owned()).expect("valid"),
        )
        .await;

        assert!(matches!(
            result,
            Err(CommandError::Page(
                crate::domain::page::error::PageError::MaxDepthExceeded { .. }
            ))
        ));
    }

    #[tokio::test]
    async fn create_child_page_database_page_cannot_nest() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool.clone());

        let page = Page::new(PageTitle::try_from("DB Page".to_owned()).expect("valid"));
        let page_id = page.id().clone();
        repo.create(&page).await.expect("create page");

        // Assign to a database.
        let db_id_str = create_test_database(&pool).await;
        let db_id: crate::domain::database::entity::DatabaseId =
            db_id_str.parse().expect("valid db id");
        repo.set_database_id(&page_id, Some(&db_id))
            .await
            .expect("set db id");

        let result = build_create_child_page(
            repo,
            page_id,
            PageTitle::try_from("Child".to_owned()).expect("valid"),
        )
        .await;

        assert!(matches!(
            result,
            Err(CommandError::Page(
                crate::domain::page::error::PageError::DatabasePageCannotNest { .. }
            ))
        ));
    }

    #[tokio::test]
    async fn move_page_success() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool);

        let parent = Page::new(PageTitle::try_from("Parent".to_owned()).expect("valid"));
        let parent_id = parent.id().clone();
        repo.create(&parent).await.expect("create parent");

        let child = Page::new(PageTitle::try_from("Child".to_owned()).expect("valid"));
        let child_id = child.id().clone();
        repo.create(&child).await.expect("create child");

        let result = build_move_page(repo, child_id, Some(parent_id.clone())).await;
        assert!(result.is_ok());
        let dto = result.expect("should succeed");
        assert_eq!(dto.parent_id, Some(parent_id.to_string()));
    }

    #[tokio::test]
    async fn move_page_circular_reference() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool);

        let parent = Page::new(PageTitle::try_from("Parent".to_owned()).expect("valid"));
        let parent_id = parent.id().clone();
        repo.create(&parent).await.expect("create parent");

        let child = Page::new_child(
            PageTitle::try_from("Child".to_owned()).expect("valid"),
            parent_id.clone(),
        );
        let child_id = child.id().clone();
        repo.create(&child).await.expect("create child");

        // Try to move parent under child → circular reference.
        let result = build_move_page(repo, parent_id, Some(child_id)).await;
        assert!(matches!(
            result,
            Err(CommandError::Page(
                crate::domain::page::error::PageError::CircularReference { .. }
            ))
        ));
    }

    #[tokio::test]
    async fn move_page_root_promotion() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool);

        let parent = Page::new(PageTitle::try_from("Parent".to_owned()).expect("valid"));
        let parent_id = parent.id().clone();
        repo.create(&parent).await.expect("create parent");

        let child = Page::new_child(
            PageTitle::try_from("Child".to_owned()).expect("valid"),
            parent_id,
        );
        let child_id = child.id().clone();
        repo.create(&child).await.expect("create child");

        // Move child to root (newParentId = null).
        let result = build_move_page(repo, child_id, None).await;
        assert!(result.is_ok());
        let dto = result.expect("should succeed");
        assert!(dto.parent_id.is_none());
    }

    #[tokio::test]
    async fn move_page_no_op_same_parent() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool);

        let parent = Page::new(PageTitle::try_from("Parent".to_owned()).expect("valid"));
        let parent_id = parent.id().clone();
        repo.create(&parent).await.expect("create parent");

        let child = Page::new_child(
            PageTitle::try_from("Child".to_owned()).expect("valid"),
            parent_id.clone(),
        );
        let child_id = child.id().clone();
        repo.create(&child).await.expect("create child");

        // Move child to same parent — should succeed (no-op).
        let result = build_move_page(repo, child_id, Some(parent_id.clone())).await;
        assert!(result.is_ok());
        let dto = result.expect("should succeed");
        assert_eq!(dto.parent_id, Some(parent_id.to_string()));
    }

    #[tokio::test]
    async fn delete_page_with_child_promotion_to_grandparent() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool.clone());

        let grandparent = Page::new(PageTitle::try_from("Grandparent".to_owned()).expect("valid"));
        let grandparent_id = grandparent.id().clone();
        repo.create(&grandparent).await.expect("create grandparent");

        let parent = Page::new_child(
            PageTitle::try_from("Parent".to_owned()).expect("valid"),
            grandparent_id.clone(),
        );
        let parent_id = parent.id().clone();
        repo.create(&parent).await.expect("create parent");

        let child = Page::new_child(
            PageTitle::try_from("Child".to_owned()).expect("valid"),
            parent_id.clone(),
        );
        let child_id = child.id().clone();
        repo.create(&child).await.expect("create child");

        // Delete parent → child should be promoted to grandparent.
        build_delete_page_with_promotion(repo, parent_id)
            .await
            .expect("delete parent");

        let repo2 = SqlxPageRepository::new(pool);
        let promoted_child = repo2.find_by_id(&child_id).await.expect("find child");
        assert_eq!(
            promoted_child.parent_id().map(|id| id.to_string()),
            Some(grandparent_id.to_string())
        );
    }

    #[tokio::test]
    async fn delete_page_with_child_promotion_to_root() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool.clone());

        let parent = Page::new(PageTitle::try_from("Parent".to_owned()).expect("valid"));
        let parent_id = parent.id().clone();
        repo.create(&parent).await.expect("create parent");

        let child = Page::new_child(
            PageTitle::try_from("Child".to_owned()).expect("valid"),
            parent_id.clone(),
        );
        let child_id = child.id().clone();
        repo.create(&child).await.expect("create child");

        // Delete parent (root) → child should be promoted to root.
        build_delete_page_with_promotion(repo, parent_id)
            .await
            .expect("delete parent");

        let repo2 = SqlxPageRepository::new(pool);
        let promoted_child = repo2.find_by_id(&child_id).await.expect("find child");
        assert!(promoted_child.parent_id().is_none());
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
