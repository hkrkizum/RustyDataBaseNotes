use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::domain::database::entity::{Database, DatabaseId, DatabaseTitle};
use crate::domain::database::error::DatabaseError;
use crate::domain::database::repository::DatabaseRepository;
use crate::ipc::error::CommandError;

/// SQLite-backed implementation of [`DatabaseRepository`].
pub struct SqlxDatabaseRepository {
    pool: SqlitePool,
}

impl SqlxDatabaseRepository {
    /// Creates a new repository backed by the given connection pool.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl DatabaseRepository for SqlxDatabaseRepository {
    type Error = CommandError;

    async fn create(&self, database: &Database) -> Result<(), Self::Error> {
        let id = database.id().to_string();
        let title = database.title().to_string();
        let created_at = database.created_at().to_rfc3339();
        let updated_at = database.updated_at().to_rfc3339();

        sqlx::query!(
            "INSERT INTO databases (id, title, created_at, updated_at) VALUES (?, ?, ?, ?)",
            id,
            title,
            created_at,
            updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        Ok(())
    }

    async fn find_by_id(&self, id: &DatabaseId) -> Result<Database, Self::Error> {
        let id_str = id.to_string();

        let row = sqlx::query!(
            "SELECT id, title, created_at, updated_at FROM databases WHERE id = ?",
            id_str
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        match row {
            Some(row) => {
                let db_id: DatabaseId = row.id.parse().map_err(|_| {
                    crate::infrastructure::persistence::error::StorageError::from(
                        sqlx::Error::ColumnDecode {
                            index: "id".to_owned(),
                            source: Box::new(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "invalid database id",
                            )),
                        },
                    )
                })?;
                let title = DatabaseTitle::try_from(row.title)?;
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
                Ok(Database::from_stored(db_id, title, created_at, updated_at))
            }
            None => Err(DatabaseError::NotFound { id: id.clone() }.into()),
        }
    }

    async fn find_all(&self) -> Result<Vec<Database>, Self::Error> {
        let rows = sqlx::query!(
            "SELECT id, title, created_at, updated_at FROM databases ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        let mut databases = Vec::with_capacity(rows.len());
        for row in rows {
            let db_id: DatabaseId = row.id.parse().map_err(|_| {
                crate::infrastructure::persistence::error::StorageError::from(
                    sqlx::Error::ColumnDecode {
                        index: "id".to_owned(),
                        source: Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "invalid database id",
                        )),
                    },
                )
            })?;
            let title = DatabaseTitle::try_from(row.title)?;
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
            databases.push(Database::from_stored(db_id, title, created_at, updated_at));
        }

        Ok(databases)
    }

    async fn update_title(
        &self,
        id: &DatabaseId,
        title: &DatabaseTitle,
    ) -> Result<Database, Self::Error> {
        let id_str = id.to_string();
        let title_str = title.to_string();
        let updated_at = Utc::now().to_rfc3339();

        let result = sqlx::query!(
            "UPDATE databases SET title = ?, updated_at = ? WHERE id = ?",
            title_str,
            updated_at,
            id_str
        )
        .execute(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        if result.rows_affected() == 0 {
            return Err(DatabaseError::NotFound { id: id.clone() }.into());
        }

        self.find_by_id(id).await
    }

    async fn delete(&self, id: &DatabaseId) -> Result<(), Self::Error> {
        let id_str = id.to_string();

        let result = sqlx::query!("DELETE FROM databases WHERE id = ?", id_str)
            .execute(&self.pool)
            .await
            .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        if result.rows_affected() == 0 {
            return Err(DatabaseError::NotFound { id: id.clone() }.into());
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
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .expect("pragma");
        pool
    }

    #[tokio::test]
    async fn create_and_find_by_id_roundtrip() {
        let pool = setup_pool().await;
        let repo = SqlxDatabaseRepository::new(pool);

        let title = DatabaseTitle::try_from("Test Database".to_owned()).expect("valid title");
        let database = Database::new(title);
        let db_id = database.id().clone();

        repo.create(&database).await.expect("create should succeed");

        let found = repo.find_by_id(&db_id).await.expect("should find database");
        assert_eq!(found.id(), database.id());
        assert_eq!(found.title().as_str(), "Test Database");
    }

    #[tokio::test]
    async fn find_by_id_not_found() {
        let pool = setup_pool().await;
        let repo = SqlxDatabaseRepository::new(pool);

        let id = DatabaseId::new();
        let result = repo.find_by_id(&id).await;
        assert!(matches!(
            result,
            Err(CommandError::Database(DatabaseError::NotFound { .. }))
        ));
    }

    #[tokio::test]
    async fn find_all_empty() {
        let pool = setup_pool().await;
        let repo = SqlxDatabaseRepository::new(pool);

        let databases = repo.find_all().await.expect("should succeed");
        assert!(databases.is_empty());
    }

    #[tokio::test]
    async fn find_all_returns_desc_order() {
        let pool = setup_pool().await;
        let repo = SqlxDatabaseRepository::new(pool);

        let title1 = DatabaseTitle::try_from("First".to_owned()).expect("valid");
        let db1 = Database::new(title1);
        repo.create(&db1).await.expect("create 1");

        // Small delay to ensure different timestamps
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        let title2 = DatabaseTitle::try_from("Second".to_owned()).expect("valid");
        let db2 = Database::new(title2);
        repo.create(&db2).await.expect("create 2");

        let databases = repo.find_all().await.expect("should succeed");
        assert_eq!(databases.len(), 2);
        // Most recent first
        assert_eq!(databases[0].title().as_str(), "Second");
        assert_eq!(databases[1].title().as_str(), "First");
    }

    #[tokio::test]
    async fn update_title_success() {
        let pool = setup_pool().await;
        let repo = SqlxDatabaseRepository::new(pool);

        let title = DatabaseTitle::try_from("Original".to_owned()).expect("valid");
        let database = Database::new(title);
        let db_id = database.id().clone();
        repo.create(&database).await.expect("create");

        let new_title = DatabaseTitle::try_from("Updated".to_owned()).expect("valid");
        let updated = repo
            .update_title(&db_id, &new_title)
            .await
            .expect("update should succeed");

        assert_eq!(updated.title().as_str(), "Updated");
        assert!(updated.updated_at() >= database.created_at());
    }

    #[tokio::test]
    async fn update_title_not_found() {
        let pool = setup_pool().await;
        let repo = SqlxDatabaseRepository::new(pool);

        let id = DatabaseId::new();
        let title = DatabaseTitle::try_from("Title".to_owned()).expect("valid");
        let result = repo.update_title(&id, &title).await;
        assert!(matches!(
            result,
            Err(CommandError::Database(DatabaseError::NotFound { .. }))
        ));
    }

    #[tokio::test]
    async fn delete_success() {
        let pool = setup_pool().await;
        let repo = SqlxDatabaseRepository::new(pool);

        let title = DatabaseTitle::try_from("To Delete".to_owned()).expect("valid title");
        let database = Database::new(title);
        let db_id = database.id().clone();
        repo.create(&database).await.expect("create");

        repo.delete(&db_id).await.expect("delete should succeed");

        let result = repo.find_by_id(&db_id).await;
        assert!(matches!(
            result,
            Err(CommandError::Database(DatabaseError::NotFound { .. }))
        ));
    }

    #[tokio::test]
    async fn delete_cascades_properties_and_values() {
        use crate::domain::page::entity::{Page, PageTitle};
        use crate::domain::page::repository::PageRepository;
        use crate::domain::property::entity::{
            Property, PropertyName, PropertyType, PropertyValue, PropertyValueInput,
        };
        use crate::domain::property::repository::{PropertyRepository, PropertyValueRepository};
        use crate::infrastructure::persistence::page_repository::SqlxPageRepository;
        use crate::infrastructure::persistence::property_repository::SqlxPropertyRepository;
        use crate::infrastructure::persistence::property_value_repository::SqlxPropertyValueRepository;

        let pool = setup_pool().await;
        let db_repo = SqlxDatabaseRepository::new(pool.clone());

        // Create database
        let title = DatabaseTitle::try_from("Cascade DB".to_owned()).expect("valid");
        let database = Database::new(title);
        let db_id = database.id().clone();
        db_repo.create(&database).await.expect("create db");

        // Create property
        let prop_repo = SqlxPropertyRepository::new(pool.clone());
        let pname = PropertyName::try_from("Status".to_owned()).expect("valid");
        let prop = Property::new(db_id.clone(), pname, PropertyType::Text, None, 0).expect("valid");
        let prop_id = prop.id().clone();
        prop_repo.create(&prop).await.expect("create prop");

        // Create page and attach to database
        let page_repo = SqlxPageRepository::new(pool.clone());
        let page_title = PageTitle::try_from("Test Page".to_owned()).expect("valid");
        let page = Page::new(page_title);
        let page_id = page.id().clone();
        page_repo.create(&page).await.expect("create page");
        page_repo
            .set_database_id(&page_id, Some(&db_id))
            .await
            .expect("set db id");

        // Create property value
        let pv_repo = SqlxPropertyValueRepository::new(pool.clone());
        let pv = PropertyValue::new_validated(
            page_id.clone(),
            prop_id.clone(),
            PropertyType::Text,
            None,
            PropertyValueInput::Text("hello".to_owned()),
        )
        .expect("valid");
        pv_repo.upsert(&pv).await.expect("upsert");

        // Delete database
        db_repo.delete(&db_id).await.expect("delete db");

        // Properties should be cascade-deleted
        let props = prop_repo.find_by_database_id(&db_id).await.expect("find");
        assert!(props.is_empty());

        // Property values should be cascade-deleted (via property CASCADE)
        let values = pv_repo.find_by_property_id(&prop_id).await.expect("find");
        assert!(values.is_empty());

        // Page should still exist, but with database_id = NULL
        let page_found = page_repo.find_by_id(&page_id).await.expect("find page");
        assert!(page_found.database_id().is_none());
    }

    #[tokio::test]
    async fn delete_not_found() {
        let pool = setup_pool().await;
        let repo = SqlxDatabaseRepository::new(pool);

        let id = DatabaseId::new();
        let result = repo.delete(&id).await;
        assert!(matches!(
            result,
            Err(CommandError::Database(DatabaseError::NotFound { .. }))
        ));
    }
}
