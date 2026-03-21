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

impl PageRepository for SqlxPageRepository {
    type Error = CommandError;

    async fn create(&self, page: &Page) -> Result<(), Self::Error> {
        let id = page.id().to_string();
        let title = page.title().to_string();
        let database_id = page.database_id().map(|id| id.to_string());
        let created_at = page.created_at().to_rfc3339();
        let updated_at = page.updated_at().to_rfc3339();

        sqlx::query!(
            "INSERT INTO pages (id, title, database_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
            id,
            title,
            database_id,
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
            "SELECT id, title, database_id, created_at, updated_at FROM pages WHERE id = ?",
            id_str
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        match row {
            Some(row) => {
                let page_id: PageId = row
                    .id
                    .parse()
                    .map_err(|_| PageError::NotFound { id: id.clone() })?;
                let title = PageTitle::try_from(row.title)?;
                let database_id = row
                    .database_id
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
                let created_at: DateTime<Utc> = row.created_at.parse().map_err(|_| {
                    crate::infrastructure::persistence::error::StorageError::from(
                        sqlx::Error::ColumnDecode {
                            index: "created_at".to_owned(),
                            source: Box::new(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "invalid created_at timestamp",
                            )),
                        },
                    )
                })?;
                let updated_at: DateTime<Utc> = row.updated_at.parse().map_err(|_| {
                    crate::infrastructure::persistence::error::StorageError::from(
                        sqlx::Error::ColumnDecode {
                            index: "updated_at".to_owned(),
                            source: Box::new(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "invalid updated_at timestamp",
                            )),
                        },
                    )
                })?;
                Ok(Page::from_stored(
                    page_id,
                    title,
                    database_id,
                    created_at,
                    updated_at,
                ))
            }
            None => Err(PageError::NotFound { id: id.clone() }.into()),
        }
    }

    async fn find_all(&self) -> Result<Vec<Page>, Self::Error> {
        let rows = sqlx::query!(
            "SELECT id, title, database_id, created_at, updated_at FROM pages ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        let mut pages = Vec::with_capacity(rows.len());
        for row in rows {
            let page_id: PageId = row.id.parse().map_err(|_| {
                crate::infrastructure::persistence::error::StorageError::from(
                    sqlx::Error::ColumnDecode {
                        index: "id".to_owned(),
                        source: Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "invalid page id",
                        )),
                    },
                )
            })?;
            let title = PageTitle::try_from(row.title)?;
            let database_id = row
                .database_id
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
            let created_at: DateTime<Utc> = row.created_at.parse().map_err(|_| {
                crate::infrastructure::persistence::error::StorageError::from(
                    sqlx::Error::ColumnDecode {
                        index: "created_at".to_owned(),
                        source: Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "invalid created_at timestamp",
                        )),
                    },
                )
            })?;
            let updated_at: DateTime<Utc> = row.updated_at.parse().map_err(|_| {
                crate::infrastructure::persistence::error::StorageError::from(
                    sqlx::Error::ColumnDecode {
                        index: "updated_at".to_owned(),
                        source: Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "invalid updated_at timestamp",
                        )),
                    },
                )
            })?;
            pages.push(Page::from_stored(
                page_id,
                title,
                database_id,
                created_at,
                updated_at,
            ));
        }

        Ok(pages)
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
            "SELECT id, title, database_id, created_at, updated_at FROM pages WHERE database_id IS NULL ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        let mut pages = Vec::with_capacity(rows.len());
        for row in rows {
            let page_id: PageId = row.id.parse().map_err(|_| {
                crate::infrastructure::persistence::error::StorageError::from(
                    sqlx::Error::ColumnDecode {
                        index: "id".to_owned(),
                        source: Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "invalid page id",
                        )),
                    },
                )
            })?;
            let title = PageTitle::try_from(row.title)?;
            let database_id = row
                .database_id
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
            let created_at: DateTime<Utc> = row.created_at.parse().map_err(|_| {
                crate::infrastructure::persistence::error::StorageError::from(
                    sqlx::Error::ColumnDecode {
                        index: "created_at".to_owned(),
                        source: Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "invalid created_at timestamp",
                        )),
                    },
                )
            })?;
            let updated_at: DateTime<Utc> = row.updated_at.parse().map_err(|_| {
                crate::infrastructure::persistence::error::StorageError::from(
                    sqlx::Error::ColumnDecode {
                        index: "updated_at".to_owned(),
                        source: Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "invalid updated_at timestamp",
                        )),
                    },
                )
            })?;
            pages.push(Page::from_stored(
                page_id,
                title,
                database_id,
                created_at,
                updated_at,
            ));
        }

        Ok(pages)
    }

    async fn find_by_database_id(
        &self,
        database_id: &DatabaseId,
    ) -> Result<Vec<Page>, Self::Error> {
        let db_id_str = database_id.to_string();

        let rows = sqlx::query!(
            "SELECT id, title, database_id, created_at, updated_at FROM pages WHERE database_id = ? ORDER BY created_at DESC",
            db_id_str
        )
        .fetch_all(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        let mut pages = Vec::with_capacity(rows.len());
        for row in rows {
            let page_id: PageId = row.id.parse().map_err(|_| {
                crate::infrastructure::persistence::error::StorageError::from(
                    sqlx::Error::ColumnDecode {
                        index: "id".to_owned(),
                        source: Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "invalid page id",
                        )),
                    },
                )
            })?;
            let title = PageTitle::try_from(row.title)?;
            let db_id = row
                .database_id
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
            let created_at: DateTime<Utc> = row.created_at.parse().map_err(|_| {
                crate::infrastructure::persistence::error::StorageError::from(
                    sqlx::Error::ColumnDecode {
                        index: "created_at".to_owned(),
                        source: Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "invalid created_at timestamp",
                        )),
                    },
                )
            })?;
            let updated_at: DateTime<Utc> = row.updated_at.parse().map_err(|_| {
                crate::infrastructure::persistence::error::StorageError::from(
                    sqlx::Error::ColumnDecode {
                        index: "updated_at".to_owned(),
                        source: Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "invalid updated_at timestamp",
                        )),
                    },
                )
            })?;
            pages.push(Page::from_stored(
                page_id, title, db_id, created_at, updated_at,
            ));
        }

        Ok(pages)
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
}
