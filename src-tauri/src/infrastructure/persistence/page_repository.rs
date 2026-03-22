use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::domain::database::entity::DatabaseId;
use crate::domain::page::entity::{Page, PageId, PageTitle};
use crate::domain::page::error::PageError;
use crate::domain::page::repository::PageRepository;
use crate::ipc::error::CommandError;

/// SQLite-backed implementation of [`PageRepository`].
pub struct SqlxPageRepository {
    pool: SqlitePool,
}

impl SqlxPageRepository {
    /// Creates a new repository backed by the given connection pool.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

/// Converts raw row fields into a [`Page`] entity.
///
/// Centralises the parsing logic shared by all repository queries.
///
/// # Errors
///
/// Returns [`CommandError`] if any field fails to parse.
fn row_to_page(
    id: String,
    title: String,
    database_id: Option<String>,
    parent_id: Option<String>,
    sort_order: i64,
    created_at: String,
    updated_at: String,
) -> Result<Page, CommandError> {
    let page_id: PageId = id.parse().map_err(|_| {
        crate::infrastructure::persistence::error::StorageError::from(sqlx::Error::ColumnDecode {
            index: "id".to_owned(),
            source: Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid page id",
            )),
        })
    })?;
    let title = PageTitle::try_from(title)?;
    let database_id = database_id
        .map(|s| s.parse::<DatabaseId>())
        .transpose()
        .map_err(|_| {
            crate::infrastructure::persistence::error::StorageError::from(
                sqlx::Error::ColumnDecode {
                    index: "database_id".to_owned(),
                    source: Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "invalid database_id",
                    )),
                },
            )
        })?;
    let parent_id = parent_id
        .map(|s| s.parse::<PageId>())
        .transpose()
        .map_err(|_| {
            crate::infrastructure::persistence::error::StorageError::from(
                sqlx::Error::ColumnDecode {
                    index: "parent_id".to_owned(),
                    source: Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "invalid parent_id",
                    )),
                },
            )
        })?;
    let created_at: DateTime<Utc> = created_at.parse().map_err(|_| {
        crate::infrastructure::persistence::error::StorageError::from(sqlx::Error::ColumnDecode {
            index: "created_at".to_owned(),
            source: Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid created_at timestamp",
            )),
        })
    })?;
    let updated_at: DateTime<Utc> = updated_at.parse().map_err(|_| {
        crate::infrastructure::persistence::error::StorageError::from(sqlx::Error::ColumnDecode {
            index: "updated_at".to_owned(),
            source: Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid updated_at timestamp",
            )),
        })
    })?;
    Ok(Page::from_stored(
        page_id,
        title,
        database_id,
        parent_id,
        sort_order,
        created_at,
        updated_at,
    ))
}

/// Converts multiple raw rows into [`Page`] entities.
fn rows_to_pages<I>(rows: I) -> Result<Vec<Page>, CommandError>
where
    I: IntoIterator<
        Item = (
            String,
            String,
            Option<String>,
            Option<String>,
            i64,
            String,
            String,
        ),
    >,
{
    rows.into_iter()
        .map(
            |(id, title, database_id, parent_id, sort_order, created_at, updated_at)| {
                row_to_page(
                    id,
                    title,
                    database_id,
                    parent_id,
                    sort_order,
                    created_at,
                    updated_at,
                )
            },
        )
        .collect()
}

impl PageRepository for SqlxPageRepository {
    type Error = CommandError;

    async fn create(&self, page: &Page) -> Result<(), Self::Error> {
        let id = page.id().to_string();
        let title = page.title().to_string();
        let database_id = page.database_id().map(|id| id.to_string());
        let parent_id = page.parent_id().map(|id| id.to_string());
        let sort_order = page.sort_order();
        let created_at = page.created_at().to_rfc3339();
        let updated_at = page.updated_at().to_rfc3339();

        sqlx::query!(
            "INSERT INTO pages (id, title, database_id, parent_id, sort_order, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
            id,
            title,
            database_id,
            parent_id,
            sort_order,
            created_at,
            updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        Ok(())
    }

    async fn find_by_id(&self, id: &PageId) -> Result<Page, Self::Error> {
        let id_str = id.to_string();

        let row = sqlx::query!(
            "SELECT id, title, database_id, parent_id, sort_order, created_at, updated_at FROM pages WHERE id = ?",
            id_str
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        match row {
            Some(row) => row_to_page(
                row.id,
                row.title,
                row.database_id,
                row.parent_id,
                row.sort_order,
                row.created_at,
                row.updated_at,
            ),
            None => Err(PageError::NotFound { id: id.clone() }.into()),
        }
    }

    async fn find_all(&self) -> Result<Vec<Page>, Self::Error> {
        let rows = sqlx::query!(
            "SELECT id, title, database_id, parent_id, sort_order, created_at, updated_at FROM pages ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        rows_to_pages(rows.into_iter().map(|r| {
            (
                r.id,
                r.title,
                r.database_id,
                r.parent_id,
                r.sort_order,
                r.created_at,
                r.updated_at,
            )
        }))
    }

    async fn update_title(&self, id: &PageId, title: &PageTitle) -> Result<Page, Self::Error> {
        let id_str = id.to_string();
        let title_str = title.to_string();
        let updated_at = Utc::now().to_rfc3339();

        let result = sqlx::query!(
            "UPDATE pages SET title = ?, updated_at = ? WHERE id = ?",
            title_str,
            updated_at,
            id_str
        )
        .execute(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        if result.rows_affected() == 0 {
            return Err(PageError::NotFound { id: id.clone() }.into());
        }

        self.find_by_id(id).await
    }

    async fn delete(&self, id: &PageId) -> Result<(), Self::Error> {
        let id_str = id.to_string();

        let result = sqlx::query!("DELETE FROM pages WHERE id = ?", id_str)
            .execute(&self.pool)
            .await
            .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        if result.rows_affected() == 0 {
            return Err(PageError::NotFound { id: id.clone() }.into());
        }

        Ok(())
    }

    async fn set_database_id(
        &self,
        page_id: &PageId,
        database_id: Option<&DatabaseId>,
    ) -> Result<(), Self::Error> {
        let id_str = page_id.to_string();
        let db_id_str = database_id.map(|id| id.to_string());

        let result = sqlx::query!(
            "UPDATE pages SET database_id = ? WHERE id = ?",
            db_id_str,
            id_str
        )
        .execute(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        if result.rows_affected() == 0 {
            return Err(PageError::NotFound {
                id: page_id.clone(),
            }
            .into());
        }
        Ok(())
    }

    async fn find_standalone_pages(&self) -> Result<Vec<Page>, Self::Error> {
        let rows = sqlx::query!(
            "SELECT id, title, database_id, parent_id, sort_order, created_at, updated_at FROM pages WHERE database_id IS NULL ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        rows_to_pages(rows.into_iter().map(|r| {
            (
                r.id,
                r.title,
                r.database_id,
                r.parent_id,
                r.sort_order,
                r.created_at,
                r.updated_at,
            )
        }))
    }

    async fn find_by_database_id(
        &self,
        database_id: &DatabaseId,
    ) -> Result<Vec<Page>, Self::Error> {
        let db_id_str = database_id.to_string();

        let rows = sqlx::query!(
            "SELECT id, title, database_id, parent_id, sort_order, created_at, updated_at FROM pages WHERE database_id = ? ORDER BY created_at DESC",
            db_id_str
        )
        .fetch_all(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        rows_to_pages(rows.into_iter().map(|r| {
            (
                r.id,
                r.title,
                r.database_id,
                r.parent_id,
                r.sort_order,
                r.created_at,
                r.updated_at,
            )
        }))
    }

    async fn update_parent_id(
        &self,
        page_id: &PageId,
        parent_id: Option<&PageId>,
    ) -> Result<Page, Self::Error> {
        let id_str = page_id.to_string();
        let parent_id_str = parent_id.map(|id| id.to_string());
        let updated_at = Utc::now().to_rfc3339();

        let result = sqlx::query!(
            "UPDATE pages SET parent_id = ?, updated_at = ? WHERE id = ?",
            parent_id_str,
            updated_at,
            id_str
        )
        .execute(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        if result.rows_affected() == 0 {
            return Err(PageError::NotFound {
                id: page_id.clone(),
            }
            .into());
        }

        self.find_by_id(page_id).await
    }

    async fn find_children(&self, parent_id: &PageId) -> Result<Vec<Page>, Self::Error> {
        let parent_id_str = parent_id.to_string();

        let rows = sqlx::query!(
            "SELECT id, title, database_id, parent_id, sort_order, created_at, updated_at FROM pages WHERE parent_id = ? ORDER BY created_at DESC",
            parent_id_str
        )
        .fetch_all(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        rows_to_pages(rows.into_iter().map(|r| {
            (
                r.id,
                r.title,
                r.database_id,
                r.parent_id,
                r.sort_order,
                r.created_at,
                r.updated_at,
            )
        }))
    }

    async fn find_root_pages(&self) -> Result<Vec<Page>, Self::Error> {
        let rows = sqlx::query!(
            "SELECT id, title, database_id, parent_id, sort_order, created_at, updated_at FROM pages WHERE parent_id IS NULL AND database_id IS NULL ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        rows_to_pages(rows.into_iter().map(|r| {
            (
                r.id,
                r.title,
                r.database_id,
                r.parent_id,
                r.sort_order,
                r.created_at,
                r.updated_at,
            )
        }))
    }

    async fn find_ancestors(&self, page_id: &PageId) -> Result<Vec<Page>, Self::Error> {
        let id_str = page_id.to_string();

        let rows = sqlx::query!(
            r#"
            WITH RECURSIVE ancestors AS (
                SELECT id, parent_id, 1 AS depth
                FROM pages
                WHERE id = ?1
                UNION ALL
                SELECT p.id, p.parent_id, a.depth + 1
                FROM pages p
                INNER JOIN ancestors a ON p.id = a.parent_id
                WHERE a.depth < 10
            )
            SELECT p.id, p.title, p.database_id, p.parent_id, p.sort_order,
                   p.created_at, p.updated_at
            FROM ancestors a
            INNER JOIN pages p ON a.id = p.id
            WHERE a.id != ?1
            ORDER BY a.depth ASC
            "#,
            id_str
        )
        .fetch_all(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        rows_to_pages(rows.into_iter().map(|r| {
            (
                r.id,
                r.title,
                r.database_id,
                r.parent_id,
                r.sort_order,
                r.created_at,
                r.updated_at,
            )
        }))
    }

    async fn bulk_update_parent_id(
        &self,
        page_ids: &[PageId],
        new_parent_id: Option<&PageId>,
    ) -> Result<(), Self::Error> {
        if page_ids.is_empty() {
            return Ok(());
        }

        let new_parent_id_str = new_parent_id.map(|id| id.to_string());
        let updated_at = Utc::now().to_rfc3339();

        for page_id in page_ids {
            let id_str = page_id.to_string();
            sqlx::query!(
                "UPDATE pages SET parent_id = ?, updated_at = ? WHERE id = ?",
                new_parent_id_str,
                updated_at,
                id_str
            )
            .execute(&self.pool)
            .await
            .map_err(crate::infrastructure::persistence::error::StorageError::from)?;
        }

        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    async fn setup_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:")
            .await
            .expect("in-memory pool");
        sqlx::migrate!().run(&pool).await.expect("migrations");
        pool
    }

    #[tokio::test]
    async fn create_and_find_by_id_roundtrip() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool);

        let title = PageTitle::try_from("Test Page".to_owned()).expect("valid title");
        let page = Page::new(title);
        let page_id = page.id().clone();

        repo.create(&page).await.expect("create should succeed");

        let found = repo.find_by_id(&page_id).await.expect("should find page");
        assert_eq!(found.id(), page.id());
        assert_eq!(found.title().as_str(), "Test Page");
    }

    #[tokio::test]
    async fn find_by_id_not_found() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool);

        let id = PageId::new();
        let result = repo.find_by_id(&id).await;
        assert!(matches!(
            result,
            Err(CommandError::Page(PageError::NotFound { .. }))
        ));
    }

    #[tokio::test]
    async fn find_all_empty() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool);

        let pages = repo.find_all().await.expect("should succeed");
        assert!(pages.is_empty());
    }

    #[tokio::test]
    async fn find_all_returns_desc_order() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool);

        let title1 = PageTitle::try_from("First".to_owned()).expect("valid");
        let page1 = Page::new(title1);
        repo.create(&page1).await.expect("create 1");

        // Small delay to ensure different timestamps
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        let title2 = PageTitle::try_from("Second".to_owned()).expect("valid");
        let page2 = Page::new(title2);
        repo.create(&page2).await.expect("create 2");

        let pages = repo.find_all().await.expect("should succeed");
        assert_eq!(pages.len(), 2);
        // Most recent first
        assert_eq!(pages[0].title().as_str(), "Second");
        assert_eq!(pages[1].title().as_str(), "First");
    }

    #[tokio::test]
    async fn update_title_success() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool);

        let title = PageTitle::try_from("Original".to_owned()).expect("valid");
        let page = Page::new(title);
        let page_id = page.id().clone();
        repo.create(&page).await.expect("create");

        let new_title = PageTitle::try_from("Updated".to_owned()).expect("valid");
        let updated = repo
            .update_title(&page_id, &new_title)
            .await
            .expect("update should succeed");

        assert_eq!(updated.title().as_str(), "Updated");
        assert!(updated.updated_at() >= page.created_at());
    }

    #[tokio::test]
    async fn update_title_not_found() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool);

        let id = PageId::new();
        let title = PageTitle::try_from("Title".to_owned()).expect("valid");
        let result = repo.update_title(&id, &title).await;
        assert!(matches!(
            result,
            Err(CommandError::Page(PageError::NotFound { .. }))
        ));
    }

    #[tokio::test]
    async fn delete_success() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool);

        let title = PageTitle::try_from("To Delete".to_owned()).expect("valid");
        let page = Page::new(title);
        let page_id = page.id().clone();
        repo.create(&page).await.expect("create");

        repo.delete(&page_id).await.expect("delete should succeed");

        let result = repo.find_by_id(&page_id).await;
        assert!(matches!(
            result,
            Err(CommandError::Page(PageError::NotFound { .. }))
        ));
    }

    #[tokio::test]
    async fn delete_not_found() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool);

        let id = PageId::new();
        let result = repo.delete(&id).await;
        assert!(matches!(
            result,
            Err(CommandError::Page(PageError::NotFound { .. }))
        ));
    }

    #[tokio::test]
    async fn deleted_page_absent_from_find_all() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool);

        let title = PageTitle::try_from("Gone".to_owned()).expect("valid");
        let page = Page::new(title);
        let page_id = page.id().clone();
        repo.create(&page).await.expect("create");

        repo.delete(&page_id).await.expect("delete");

        let pages = repo.find_all().await.expect("find_all");
        assert!(pages.is_empty());
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

    // T034: set_database_id assigns page to database
    #[tokio::test]
    async fn set_database_id_assigns_page_to_database() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool.clone());

        let title = PageTitle::try_from("Test Page".to_owned()).expect("valid title");
        let page = Page::new(title);
        let page_id = page.id().clone();
        repo.create(&page).await.expect("create page");

        let db_id_str = create_test_database(&pool).await;
        let db_id: DatabaseId = db_id_str.parse().expect("valid database id");

        repo.set_database_id(&page_id, Some(&db_id))
            .await
            .expect("set_database_id");

        let found = repo.find_by_id(&page_id).await.expect("find page");
        assert_eq!(
            found.database_id().map(|id| id.to_string()),
            Some(db_id_str)
        );
    }

    // T034: set_database_id on non-existent page returns NotFound
    #[tokio::test]
    async fn set_database_id_not_found() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool);

        let page_id = PageId::new();
        let db_id = DatabaseId::new();
        let result = repo.set_database_id(&page_id, Some(&db_id)).await;
        assert!(matches!(
            result,
            Err(CommandError::Page(PageError::NotFound { .. }))
        ));
    }

    // T034: find_standalone_pages excludes assigned pages
    #[tokio::test]
    async fn find_standalone_pages_excludes_assigned() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool.clone());

        let title1 = PageTitle::try_from("Standalone".to_owned()).expect("valid");
        let page1 = Page::new(title1);
        repo.create(&page1).await.expect("create 1");

        let title2 = PageTitle::try_from("Assigned".to_owned()).expect("valid");
        let page2 = Page::new(title2);
        let page2_id = page2.id().clone();
        repo.create(&page2).await.expect("create 2");

        let db_id_str = create_test_database(&pool).await;
        let db_id: DatabaseId = db_id_str.parse().expect("valid database id");

        repo.set_database_id(&page2_id, Some(&db_id))
            .await
            .expect("set_database_id");

        let standalone = repo.find_standalone_pages().await.expect("standalone");
        assert_eq!(standalone.len(), 1);
        assert_eq!(standalone[0].title().as_str(), "Standalone");
    }

    // T045: Performance — 1,000 pages insert and find_all < 1s
    #[tokio::test]
    async fn performance_1000_pages() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool);

        let start = std::time::Instant::now();
        for i in 0..1000 {
            let title = PageTitle::try_from(format!("Page {i}")).expect("valid title");
            let page = Page::new(title);
            repo.create(&page).await.expect("create");
        }
        let create_elapsed = start.elapsed();
        assert!(
            create_elapsed < std::time::Duration::from_secs(10),
            "1000 creates took {create_elapsed:?}"
        );

        let start = std::time::Instant::now();
        let pages = repo.find_all().await.expect("find_all");
        let find_elapsed = start.elapsed();
        assert_eq!(pages.len(), 1000);
        assert!(
            find_elapsed < std::time::Duration::from_secs(1),
            "find_all for 1000 pages took {find_elapsed:?}"
        );
    }

    // T046a: Title boundary — 255 chars accepted, 256 rejected
    #[tokio::test]
    async fn title_boundary_255_and_256() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool);

        let title_255 = PageTitle::try_from("a".repeat(255)).expect("255 should be valid");
        let page = Page::new(title_255);
        repo.create(&page).await.expect("create 255-char title");

        let result_256 = PageTitle::try_from("a".repeat(256));
        assert!(matches!(
            result_256,
            Err(PageError::TitleTooLong { len: 256, max: 255 })
        ));
    }

    // T046b: Concurrent writes — two spawned tasks both succeed
    #[tokio::test]
    async fn concurrent_writes() {
        let pool = setup_pool().await;

        let pool1 = pool.clone();
        let pool2 = pool.clone();

        let h1 = tokio::spawn(async move {
            let repo = SqlxPageRepository::new(pool1);
            let title = PageTitle::try_from("Concurrent A".to_owned()).expect("valid");
            let page = Page::new(title);
            repo.create(&page).await.expect("create A");
        });

        let h2 = tokio::spawn(async move {
            let repo = SqlxPageRepository::new(pool2);
            let title = PageTitle::try_from("Concurrent B".to_owned()).expect("valid");
            let page = Page::new(title);
            repo.create(&page).await.expect("create B");
        });

        h1.await.expect("task A");
        h2.await.expect("task B");

        let repo = SqlxPageRepository::new(pool);
        let pages = repo.find_all().await.expect("find_all");
        assert_eq!(pages.len(), 2);
    }

    // T046d: WAL mode durability — data persists after pool drop
    #[tokio::test]
    async fn wal_mode_durability() {
        let dir = std::env::temp_dir().join(format!("rdbn_test_wal_{}", uuid::Uuid::now_v7()));
        std::fs::create_dir_all(&dir).expect("create temp dir");
        let db_path = dir.join("test.db");

        // Create page with first pool
        {
            let pool = crate::infrastructure::persistence::database::init_pool(&db_path)
                .await
                .expect("init pool 1");
            let repo = SqlxPageRepository::new(pool.clone());
            let title = PageTitle::try_from("Durable".to_owned()).expect("valid");
            let page = Page::new(title);
            repo.create(&page).await.expect("create");
            pool.close().await;
        }

        // Reconnect with new pool and verify
        {
            let pool = crate::infrastructure::persistence::database::init_pool(&db_path)
                .await
                .expect("init pool 2");
            let repo = SqlxPageRepository::new(pool.clone());
            let pages = repo.find_all().await.expect("find_all");
            assert_eq!(pages.len(), 1);
            assert_eq!(pages[0].title().as_str(), "Durable");
            pool.close().await;
        }

        // Cleanup
        let _ = std::fs::remove_dir_all(&dir);
    }

    // T079: 1-page-1-database constraint — page already assigned is detectable
    #[tokio::test]
    async fn page_already_in_database_detectable() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool.clone());

        let title = PageTitle::try_from("Assigned Page".to_owned()).expect("valid");
        let page = Page::new(title);
        let page_id = page.id().clone();
        repo.create(&page).await.expect("create page");

        let db_id_str = create_test_database(&pool).await;
        let db_id: DatabaseId = db_id_str.parse().expect("valid db id");

        repo.set_database_id(&page_id, Some(&db_id))
            .await
            .expect("assign to db");

        // Refetch and verify database_id is set (mimics add_existing_page_to_database check)
        let found = repo.find_by_id(&page_id).await.expect("find page");
        assert!(
            found.database_id().is_some(),
            "page should have a database_id, which would trigger AlreadyInDatabase at IPC layer"
        );
    }

    // T079: find_by_database_id returns empty for database with no pages
    #[tokio::test]
    async fn find_by_database_id_empty_database() {
        let pool = setup_pool().await;
        let repo = SqlxPageRepository::new(pool.clone());

        let db_id_str = create_test_database(&pool).await;
        let db_id: DatabaseId = db_id_str.parse().expect("valid db id");

        let pages = repo.find_by_database_id(&db_id).await.expect("find pages");
        assert!(pages.is_empty(), "empty database should have no pages");
    }
}
