use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::domain::block::entity::{Block, BlockContent, BlockId, BlockPosition};
use crate::domain::page::entity::PageId;
use crate::infrastructure::persistence::error::StorageError;
use crate::ipc::error::CommandError;

/// Trait defining persistence operations for [`Block`] entities.
///
/// # Errors
///
/// All methods return [`CommandError`] on storage failures.
#[allow(async_fn_in_trait)]
pub trait BlockRepository {
    /// Loads all blocks for the given page, sorted by position ascending.
    ///
    /// Returns an empty `Vec` if the page has no blocks.
    ///
    /// # Errors
    ///
    /// Returns [`CommandError::Storage`] on database failures.
    async fn load_blocks(&self, page_id: &PageId) -> Result<Vec<Block>, CommandError>;

    /// Persists all blocks for the given page using a delete-and-reinsert
    /// strategy within a transaction.
    ///
    /// - Deletes all existing blocks for the page.
    /// - Inserts each block with `updated_at` set to the current time.
    /// - Preserves `created_at` from the in-memory block data.
    ///
    /// # Errors
    ///
    /// Returns [`CommandError::Storage`] on database failures.
    /// The transaction is rolled back on error.
    async fn save_all(&self, page_id: &PageId, blocks: &[Block]) -> Result<(), CommandError>;
}

/// SQLite-backed implementation of [`BlockRepository`].
pub struct SqlxBlockRepository {
    pool: SqlitePool,
}

impl SqlxBlockRepository {
    /// Creates a new repository backed by the given connection pool.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl BlockRepository for SqlxBlockRepository {
    async fn load_blocks(&self, page_id: &PageId) -> Result<Vec<Block>, CommandError> {
        let page_id_str = page_id.to_string();

        let rows = sqlx::query!(
            "SELECT id, page_id, block_type, content, position, created_at, updated_at \
             FROM blocks WHERE page_id = ? ORDER BY position ASC",
            page_id_str
        )
        .fetch_all(&self.pool)
        .await
        .map_err(StorageError::from)?;

        let mut blocks = Vec::with_capacity(rows.len());
        for row in rows {
            let block_id: BlockId = row.id.parse().map_err(|_| {
                StorageError::from(sqlx::Error::ColumnDecode {
                    index: "id".to_owned(),
                    source: Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "invalid block id",
                    )),
                })
            })?;
            let p_id: PageId = row.page_id.parse().map_err(|_| {
                StorageError::from(sqlx::Error::ColumnDecode {
                    index: "page_id".to_owned(),
                    source: Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "invalid page id in block",
                    )),
                })
            })?;
            let content = BlockContent::try_from(row.content)?;
            let position = BlockPosition::try_from(row.position)?;
            let created_at: DateTime<Utc> = row.created_at.parse().map_err(|_| {
                StorageError::from(sqlx::Error::ColumnDecode {
                    index: "created_at".to_owned(),
                    source: Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "invalid created_at timestamp",
                    )),
                })
            })?;
            let updated_at: DateTime<Utc> = row.updated_at.parse().map_err(|_| {
                StorageError::from(sqlx::Error::ColumnDecode {
                    index: "updated_at".to_owned(),
                    source: Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "invalid updated_at timestamp",
                    )),
                })
            })?;

            blocks.push(Block::from_stored(
                block_id,
                p_id,
                row.block_type,
                content,
                position,
                created_at,
                updated_at,
            ));
        }

        Ok(blocks)
    }

    async fn save_all(&self, page_id: &PageId, blocks: &[Block]) -> Result<(), CommandError> {
        let page_id_str = page_id.to_string();
        let now = Utc::now().to_rfc3339();

        let mut tx = self.pool.begin().await.map_err(StorageError::from)?;

        sqlx::query!("DELETE FROM blocks WHERE page_id = ?", page_id_str)
            .execute(&mut *tx)
            .await
            .map_err(StorageError::from)?;

        for block in blocks {
            let id = block.id().to_string();
            let block_type = block.block_type().to_owned();
            let content = block.content().to_string();
            let position = block.position().value();
            let created_at = block.created_at().to_rfc3339();

            sqlx::query!(
                "INSERT INTO blocks (id, page_id, block_type, content, position, created_at, updated_at) \
                 VALUES (?, ?, ?, ?, ?, ?, ?)",
                id,
                page_id_str,
                block_type,
                content,
                position,
                created_at,
                now
            )
            .execute(&mut *tx)
            .await
            .map_err(StorageError::from)?;
        }

        tx.commit().await.map_err(StorageError::from)?;

        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

    async fn setup_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:")
            .await
            .expect("in-memory pool");
        sqlx::migrate!().run(&pool).await.expect("migrations");
        // Enable foreign keys
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .expect("pragma");
        pool
    }

    async fn create_test_page(pool: &SqlitePool) -> PageId {
        let page_id = PageId::new();
        let id_str = page_id.to_string();
        let now = Utc::now().to_rfc3339();
        sqlx::query!(
            "INSERT INTO pages (id, title, created_at, updated_at) VALUES (?, ?, ?, ?)",
            id_str,
            "Test Page",
            now,
            now
        )
        .execute(pool)
        .await
        .expect("create test page");
        page_id
    }

    // T019: BlockRepository::load_blocks() tests

    #[tokio::test]
    async fn load_blocks_empty() {
        let pool = setup_pool().await;
        let page_id = create_test_page(&pool).await;
        let repo = SqlxBlockRepository::new(pool);

        let blocks = repo.load_blocks(&page_id).await.unwrap();
        assert!(blocks.is_empty());
    }

    #[tokio::test]
    async fn load_blocks_ordered_by_position() {
        let pool = setup_pool().await;
        let page_id = create_test_page(&pool).await;

        // Insert blocks out of order
        let id1 = BlockId::new().to_string();
        let id2 = BlockId::new().to_string();
        let page_id_str = page_id.to_string();
        let now = Utc::now().to_rfc3339();

        sqlx::query!(
            "INSERT INTO blocks (id, page_id, block_type, content, position, created_at, updated_at) VALUES (?, ?, 'text', 'second', 1, ?, ?)",
            id2, page_id_str, now, now
        ).execute(&pool).await.unwrap();

        sqlx::query!(
            "INSERT INTO blocks (id, page_id, block_type, content, position, created_at, updated_at) VALUES (?, ?, 'text', 'first', 0, ?, ?)",
            id1, page_id_str, now, now
        ).execute(&pool).await.unwrap();

        let repo = SqlxBlockRepository::new(pool);
        let blocks = repo.load_blocks(&page_id).await.unwrap();

        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].content().as_str(), "first");
        assert_eq!(blocks[0].position().value(), 0);
        assert_eq!(blocks[1].content().as_str(), "second");
        assert_eq!(blocks[1].position().value(), 1);
    }

    // T047: BlockRepository::save_all() tests

    #[tokio::test]
    async fn save_all_delete_and_reinsert() {
        let pool = setup_pool().await;
        let page_id = create_test_page(&pool).await;
        let repo = SqlxBlockRepository::new(pool.clone());

        // Create blocks in memory
        let pos0 = BlockPosition::try_from(0_i64).unwrap();
        let pos1 = BlockPosition::try_from(1_i64).unwrap();
        let b0 = Block::new(page_id.clone(), pos0);
        let b1 = Block::new(page_id.clone(), pos1);

        repo.save_all(&page_id, &[b0, b1]).await.unwrap();

        let loaded = repo.load_blocks(&page_id).await.unwrap();
        assert_eq!(loaded.len(), 2);

        // Save again with only 1 block — old blocks should be deleted
        let pos0 = BlockPosition::try_from(0_i64).unwrap();
        let b2 = Block::new(page_id.clone(), pos0);
        repo.save_all(&page_id, &[b2]).await.unwrap();

        let loaded = repo.load_blocks(&page_id).await.unwrap();
        assert_eq!(loaded.len(), 1);
    }

    #[tokio::test]
    async fn save_all_preserves_created_at() {
        let pool = setup_pool().await;
        let page_id = create_test_page(&pool).await;
        let repo = SqlxBlockRepository::new(pool);

        let pos0 = BlockPosition::try_from(0_i64).unwrap();
        let block = Block::new(page_id.clone(), pos0);
        let original_created_at = block.created_at();

        repo.save_all(&page_id, &[block]).await.unwrap();

        let loaded = repo.load_blocks(&page_id).await.unwrap();
        assert_eq!(loaded.len(), 1);
        // created_at should be preserved (within 1 second tolerance for RFC3339 rounding)
        let diff = (loaded[0].created_at() - original_created_at)
            .num_seconds()
            .abs();
        assert!(diff <= 1, "created_at should be preserved, diff: {diff}s");
    }

    #[tokio::test]
    async fn save_all_updates_updated_at() {
        let pool = setup_pool().await;
        let page_id = create_test_page(&pool).await;
        let repo = SqlxBlockRepository::new(pool);

        let pos0 = BlockPosition::try_from(0_i64).unwrap();
        let block = Block::new(page_id.clone(), pos0);
        let original_updated_at = block.updated_at();

        // Wait a bit to ensure time difference
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        repo.save_all(&page_id, &[block]).await.unwrap();

        let loaded = repo.load_blocks(&page_id).await.unwrap();
        assert_eq!(loaded.len(), 1);
        assert!(loaded[0].updated_at() >= original_updated_at);
    }

    #[tokio::test]
    async fn save_all_normalizes_position() {
        let pool = setup_pool().await;
        let page_id = create_test_page(&pool).await;
        let repo = SqlxBlockRepository::new(pool);

        let pos0 = BlockPosition::try_from(0_i64).unwrap();
        let pos1 = BlockPosition::try_from(1_i64).unwrap();
        let b0 = Block::new(page_id.clone(), pos0);
        let b1 = Block::new(page_id.clone(), pos1);

        repo.save_all(&page_id, &[b0, b1]).await.unwrap();

        let loaded = repo.load_blocks(&page_id).await.unwrap();
        assert_eq!(loaded[0].position().value(), 0);
        assert_eq!(loaded[1].position().value(), 1);
    }

    // T073: PRAGMA foreign_keys CASCADE delete test

    #[tokio::test]
    async fn cascade_delete_removes_blocks() {
        let pool = setup_pool().await;
        let page_id = create_test_page(&pool).await;
        let repo = SqlxBlockRepository::new(pool.clone());

        let pos0 = BlockPosition::try_from(0_i64).unwrap();
        let block = Block::new(page_id.clone(), pos0);
        repo.save_all(&page_id, &[block]).await.unwrap();

        // Verify block exists
        let loaded = repo.load_blocks(&page_id).await.unwrap();
        assert_eq!(loaded.len(), 1);

        // Delete the page — blocks should cascade
        let page_id_str = page_id.to_string();
        sqlx::query!("DELETE FROM pages WHERE id = ?", page_id_str)
            .execute(&pool)
            .await
            .unwrap();

        // Blocks should be gone
        let loaded = repo.load_blocks(&page_id).await.unwrap();
        assert!(loaded.is_empty());
    }

    // T074: Performance test — 1,000 blocks save and reload

    #[tokio::test]
    async fn performance_1000_blocks() {
        let pool = setup_pool().await;
        let page_id = create_test_page(&pool).await;
        let repo = SqlxBlockRepository::new(pool);

        // Create 1,000 blocks in memory
        let mut blocks = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            let pos = BlockPosition::try_from(i as i64).unwrap();
            let mut block = Block::new(page_id.clone(), pos);
            let content = BlockContent::try_from(format!("Block content {i}")).unwrap();
            block.set_content(content);
            blocks.push(block);
        }

        // Save all
        let start = std::time::Instant::now();
        repo.save_all(&page_id, &blocks).await.unwrap();
        let save_elapsed = start.elapsed();
        assert!(
            save_elapsed < std::time::Duration::from_secs(1),
            "save_all for 1000 blocks took {save_elapsed:?}"
        );

        // Reload all
        let start = std::time::Instant::now();
        let loaded = repo.load_blocks(&page_id).await.unwrap();
        let load_elapsed = start.elapsed();
        assert_eq!(loaded.len(), 1_000);
        assert!(
            load_elapsed < std::time::Duration::from_secs(1),
            "load_blocks for 1000 blocks took {load_elapsed:?}"
        );
    }
}
