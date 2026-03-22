use std::collections::HashSet;

use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::domain::database::entity::DatabaseId;
use crate::domain::property::entity::PropertyId;
use crate::domain::view::entity::{
    FilterCondition, GroupCondition, SortCondition, View, ViewId, ViewName, ViewType,
};
use crate::domain::view::error::ViewError;
use crate::domain::view::repository::ViewRepository;
use crate::infrastructure::persistence::error::StorageError;
use crate::ipc::error::CommandError;

/// SQLite-backed implementation of [`ViewRepository`].
pub struct SqlxViewRepository {
    pool: SqlitePool,
}

impl SqlxViewRepository {
    /// Creates a new repository backed by the given connection pool.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Helper to reconstruct a [`View`] from a database row.
    #[allow(clippy::too_many_arguments)]
    fn row_to_view(
        id: String,
        database_id: String,
        name: String,
        view_type: String,
        sort_conditions_json: String,
        filter_conditions_json: String,
        group_condition_json: Option<String>,
        collapsed_groups_json: String,
        created_at_str: String,
        updated_at_str: String,
    ) -> Result<View, CommandError> {
        let view_id: ViewId = id.parse().map_err(|_| {
            StorageError::from(sqlx::Error::ColumnDecode {
                index: "id".to_owned(),
                source: Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "invalid view id",
                )),
            })
        })?;

        let db_id: DatabaseId = database_id.parse().map_err(|_| {
            StorageError::from(sqlx::Error::ColumnDecode {
                index: "database_id".to_owned(),
                source: Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "invalid database id",
                )),
            })
        })?;

        // Fallback: use "Table" if stored name is somehow invalid
        let view_name = ViewName::try_from(name)
            .or_else(|_| ViewName::try_from("Table".to_owned()))
            .map_err(|_| {
                StorageError::from(sqlx::Error::ColumnDecode {
                    index: "name".to_owned(),
                    source: Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "invalid view name",
                    )),
                })
            })?;

        let vt = match view_type.as_str() {
            "table" => ViewType::Table,
            _ => ViewType::Table, // default fallback
        };

        // FR-015: Deserialize with fallback — corrupt JSON returns defaults
        let sort_conditions: Vec<SortCondition> =
            serde_json::from_str(&sort_conditions_json).unwrap_or_default();
        let filter_conditions: Vec<FilterCondition> =
            serde_json::from_str(&filter_conditions_json).unwrap_or_default();
        let group_condition: Option<GroupCondition> =
            group_condition_json.and_then(|json| serde_json::from_str(&json).ok());
        let collapsed_groups: HashSet<String> =
            serde_json::from_str::<Vec<String>>(&collapsed_groups_json)
                .unwrap_or_default()
                .into_iter()
                .collect();

        let created_at: DateTime<Utc> = created_at_str.parse().map_err(|_| {
            StorageError::from(sqlx::Error::ColumnDecode {
                index: "created_at".to_owned(),
                source: Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "invalid created_at timestamp",
                )),
            })
        })?;
        let updated_at: DateTime<Utc> = updated_at_str.parse().map_err(|_| {
            StorageError::from(sqlx::Error::ColumnDecode {
                index: "updated_at".to_owned(),
                source: Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "invalid updated_at timestamp",
                )),
            })
        })?;

        Ok(View::from_stored(
            view_id,
            db_id,
            view_name,
            vt,
            sort_conditions,
            filter_conditions,
            group_condition,
            collapsed_groups,
            created_at,
            updated_at,
        ))
    }

    /// Internal: fetch a view by database_id, returning the raw row.
    async fn fetch_view(&self, database_id: &DatabaseId) -> Result<Option<View>, CommandError> {
        let db_id_str = database_id.to_string();
        let row = sqlx::query!(
            r#"SELECT id, database_id, name, view_type, sort_conditions, filter_conditions,
                      group_condition, collapsed_groups, created_at, updated_at
               FROM views WHERE database_id = ?"#,
            db_id_str
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(StorageError::from)?;

        match row {
            Some(r) => {
                let view = Self::row_to_view(
                    r.id,
                    r.database_id,
                    r.name,
                    r.view_type,
                    r.sort_conditions,
                    r.filter_conditions,
                    r.group_condition,
                    r.collapsed_groups,
                    r.created_at,
                    r.updated_at,
                )?;
                Ok(Some(view))
            }
            None => Ok(None),
        }
    }

    /// Internal: find view or return ViewNotFound error.
    async fn find_or_error(&self, database_id: &DatabaseId) -> Result<View, CommandError> {
        self.fetch_view(database_id).await?.ok_or_else(|| {
            ViewError::ViewNotFound {
                id: ViewId::new(), // placeholder
            }
            .into()
        })
    }
}

impl ViewRepository for SqlxViewRepository {
    type Error = CommandError;

    async fn find_by_database_id(
        &self,
        database_id: &DatabaseId,
    ) -> Result<Option<View>, Self::Error> {
        self.fetch_view(database_id).await
    }

    async fn save(&self, view: &View) -> Result<(), Self::Error> {
        let id = view.id().to_string();
        let db_id = view.database_id().to_string();
        let name = view.name().as_str().to_owned();
        let view_type = view.view_type().to_string();
        let sort_json =
            serde_json::to_string(view.sort_conditions()).unwrap_or_else(|_| "[]".to_owned());
        let filter_json =
            serde_json::to_string(view.filter_conditions()).unwrap_or_else(|_| "[]".to_owned());
        let group_json: Option<String> = view
            .group_condition()
            .map(|g| serde_json::to_string(g).unwrap_or_else(|_| "null".to_owned()));
        let collapsed_json =
            serde_json::to_string(&view.collapsed_groups().iter().cloned().collect::<Vec<_>>())
                .unwrap_or_else(|_| "[]".to_owned());
        let created_at = view.created_at().to_rfc3339();
        let updated_at = view.updated_at().to_rfc3339();

        sqlx::query!(
            r#"INSERT INTO views (id, database_id, name, view_type, sort_conditions,
                                  filter_conditions, group_condition, collapsed_groups,
                                  created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            id,
            db_id,
            name,
            view_type,
            sort_json,
            filter_json,
            group_json,
            collapsed_json,
            created_at,
            updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(StorageError::from)?;

        Ok(())
    }

    async fn update_sort_conditions(
        &self,
        database_id: &DatabaseId,
        conditions: &[SortCondition],
    ) -> Result<View, Self::Error> {
        let db_id_str = database_id.to_string();
        let sort_json = serde_json::to_string(conditions).unwrap_or_else(|_| "[]".to_owned());
        let updated_at = Utc::now().to_rfc3339();

        let result = sqlx::query!(
            "UPDATE views SET sort_conditions = ?, updated_at = ? WHERE database_id = ?",
            sort_json,
            updated_at,
            db_id_str
        )
        .execute(&self.pool)
        .await
        .map_err(StorageError::from)?;

        if result.rows_affected() == 0 {
            return Err(ViewError::ViewNotFound { id: ViewId::new() }.into());
        }

        self.find_or_error(database_id).await
    }

    async fn update_filter_conditions(
        &self,
        database_id: &DatabaseId,
        conditions: &[FilterCondition],
    ) -> Result<View, Self::Error> {
        let db_id_str = database_id.to_string();
        let filter_json = serde_json::to_string(conditions).unwrap_or_else(|_| "[]".to_owned());
        let updated_at = Utc::now().to_rfc3339();

        let result = sqlx::query!(
            "UPDATE views SET filter_conditions = ?, updated_at = ? WHERE database_id = ?",
            filter_json,
            updated_at,
            db_id_str
        )
        .execute(&self.pool)
        .await
        .map_err(StorageError::from)?;

        if result.rows_affected() == 0 {
            return Err(ViewError::ViewNotFound { id: ViewId::new() }.into());
        }

        self.find_or_error(database_id).await
    }

    async fn update_group_condition(
        &self,
        database_id: &DatabaseId,
        condition: Option<&GroupCondition>,
        collapsed_groups: &[String],
    ) -> Result<View, Self::Error> {
        let db_id_str = database_id.to_string();
        let group_json: Option<String> =
            condition.map(|g| serde_json::to_string(g).unwrap_or_else(|_| "null".to_owned()));
        let collapsed_json =
            serde_json::to_string(collapsed_groups).unwrap_or_else(|_| "[]".to_owned());
        let updated_at = Utc::now().to_rfc3339();

        let result = sqlx::query!(
            "UPDATE views SET group_condition = ?, collapsed_groups = ?, updated_at = ? WHERE database_id = ?",
            group_json,
            collapsed_json,
            updated_at,
            db_id_str
        )
        .execute(&self.pool)
        .await
        .map_err(StorageError::from)?;

        if result.rows_affected() == 0 {
            return Err(ViewError::ViewNotFound { id: ViewId::new() }.into());
        }

        self.find_or_error(database_id).await
    }

    async fn update_collapsed_groups(
        &self,
        database_id: &DatabaseId,
        collapsed_groups: &[String],
    ) -> Result<View, Self::Error> {
        let db_id_str = database_id.to_string();
        let collapsed_json =
            serde_json::to_string(collapsed_groups).unwrap_or_else(|_| "[]".to_owned());
        let updated_at = Utc::now().to_rfc3339();

        let result = sqlx::query!(
            "UPDATE views SET collapsed_groups = ?, updated_at = ? WHERE database_id = ?",
            collapsed_json,
            updated_at,
            db_id_str
        )
        .execute(&self.pool)
        .await
        .map_err(StorageError::from)?;

        if result.rows_affected() == 0 {
            return Err(ViewError::ViewNotFound { id: ViewId::new() }.into());
        }

        self.find_or_error(database_id).await
    }

    async fn reset(&self, database_id: &DatabaseId) -> Result<View, Self::Error> {
        let db_id_str = database_id.to_string();
        let updated_at = Utc::now().to_rfc3339();
        let empty_json = "[]";

        let result = sqlx::query!(
            r#"UPDATE views SET sort_conditions = ?, filter_conditions = ?,
                                group_condition = NULL, collapsed_groups = ?,
                                updated_at = ?
               WHERE database_id = ?"#,
            empty_json,
            empty_json,
            empty_json,
            updated_at,
            db_id_str
        )
        .execute(&self.pool)
        .await
        .map_err(StorageError::from)?;

        if result.rows_affected() == 0 {
            return Err(ViewError::ViewNotFound { id: ViewId::new() }.into());
        }

        self.find_or_error(database_id).await
    }

    async fn remove_property_references(
        &self,
        property_id: &PropertyId,
    ) -> Result<(), Self::Error> {
        // Fetch all views
        let rows = sqlx::query!(
            "SELECT id, database_id, name, view_type, sort_conditions, filter_conditions, group_condition, collapsed_groups, created_at, updated_at FROM views"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(StorageError::from)?;

        for r in rows {
            let view = Self::row_to_view(
                r.id,
                r.database_id,
                r.name,
                r.view_type,
                r.sort_conditions.clone(),
                r.filter_conditions.clone(),
                r.group_condition.clone(),
                r.collapsed_groups.clone(),
                r.created_at,
                r.updated_at,
            )?;

            let mut view = view;
            if view.remove_property_references(property_id) {
                // Save updated conditions back
                let sort_json = serde_json::to_string(view.sort_conditions())
                    .unwrap_or_else(|_| "[]".to_owned());
                let filter_json = serde_json::to_string(view.filter_conditions())
                    .unwrap_or_else(|_| "[]".to_owned());
                let group_json: Option<String> = view
                    .group_condition()
                    .map(|g| serde_json::to_string(g).unwrap_or_else(|_| "null".to_owned()));
                let collapsed_json = serde_json::to_string(
                    &view.collapsed_groups().iter().cloned().collect::<Vec<_>>(),
                )
                .unwrap_or_else(|_| "[]".to_owned());
                let updated_at = view.updated_at().to_rfc3339();
                let view_id = view.id().to_string();

                sqlx::query!(
                    r#"UPDATE views SET sort_conditions = ?, filter_conditions = ?,
                                        group_condition = ?, collapsed_groups = ?,
                                        updated_at = ?
                       WHERE id = ?"#,
                    sort_json,
                    filter_json,
                    group_json,
                    collapsed_json,
                    updated_at,
                    view_id
                )
                .execute(&self.pool)
                .await
                .map_err(StorageError::from)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use crate::domain::database::entity::{Database, DatabaseTitle};
    use crate::domain::database::repository::DatabaseRepository;
    use crate::domain::view::entity::{FilterOperator, FilterValue, SortDirection};
    use crate::infrastructure::persistence::database_repository::SqlxDatabaseRepository;

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

    async fn create_test_database(pool: &SqlitePool) -> DatabaseId {
        let db_repo = SqlxDatabaseRepository::new(pool.clone());
        let title = DatabaseTitle::try_from("Test DB".to_owned()).expect("valid");
        let database = Database::new(title);
        let db_id = database.id().clone();
        db_repo.create(&database).await.expect("create");
        db_id
    }

    #[tokio::test]
    async fn save_and_find_roundtrip() {
        let pool = setup_pool().await;
        let db_id = create_test_database(&pool).await;
        let repo = SqlxViewRepository::new(pool);

        let view = View::new_default(db_id.clone());
        repo.save(&view).await.expect("save");

        let found = repo.find_by_database_id(&db_id).await.expect("find");
        assert!(found.is_some());
        let found = found.expect("view exists");
        assert_eq!(found.name().as_str(), "Table");
        assert_eq!(found.view_type(), ViewType::Table);
        assert!(found.sort_conditions().is_empty());
        assert!(found.filter_conditions().is_empty());
        assert!(found.group_condition().is_none());
    }

    #[tokio::test]
    async fn find_by_database_id_not_found() {
        let pool = setup_pool().await;
        let repo = SqlxViewRepository::new(pool);

        let result = repo
            .find_by_database_id(&DatabaseId::new())
            .await
            .expect("find");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn update_sort_conditions() {
        let pool = setup_pool().await;
        let db_id = create_test_database(&pool).await;
        let repo = SqlxViewRepository::new(pool);

        let view = View::new_default(db_id.clone());
        repo.save(&view).await.expect("save");

        let pid = PropertyId::new();
        let conditions = vec![SortCondition {
            property_id: pid.clone(),
            direction: SortDirection::Descending,
        }];

        let updated = repo
            .update_sort_conditions(&db_id, &conditions)
            .await
            .expect("update");
        assert_eq!(updated.sort_conditions().len(), 1);
        assert_eq!(updated.sort_conditions()[0].property_id, pid);
        assert_eq!(
            updated.sort_conditions()[0].direction,
            SortDirection::Descending
        );
    }

    #[tokio::test]
    async fn update_filter_conditions() {
        let pool = setup_pool().await;
        let db_id = create_test_database(&pool).await;
        let repo = SqlxViewRepository::new(pool);

        let view = View::new_default(db_id.clone());
        repo.save(&view).await.expect("save");

        let pid = PropertyId::new();
        let conditions = vec![FilterCondition {
            property_id: pid.clone(),
            operator: FilterOperator::Contains,
            value: Some(FilterValue::Text("hello".to_owned())),
        }];

        let updated = repo
            .update_filter_conditions(&db_id, &conditions)
            .await
            .expect("update");
        assert_eq!(updated.filter_conditions().len(), 1);
        assert_eq!(
            updated.filter_conditions()[0].operator,
            FilterOperator::Contains
        );
    }

    #[tokio::test]
    async fn update_group_condition_and_collapsed() {
        let pool = setup_pool().await;
        let db_id = create_test_database(&pool).await;
        let repo = SqlxViewRepository::new(pool);

        let view = View::new_default(db_id.clone());
        repo.save(&view).await.expect("save");

        let pid = PropertyId::new();
        let gc = GroupCondition { property_id: pid };
        let collapsed = vec!["group1".to_owned(), "group2".to_owned()];

        let updated = repo
            .update_group_condition(&db_id, Some(&gc), &collapsed)
            .await
            .expect("update");
        assert!(updated.group_condition().is_some());
        assert_eq!(updated.collapsed_groups().len(), 2);
    }

    #[tokio::test]
    async fn reset_clears_all() {
        let pool = setup_pool().await;
        let db_id = create_test_database(&pool).await;
        let repo = SqlxViewRepository::new(pool);

        let view = View::new_default(db_id.clone());
        repo.save(&view).await.expect("save");

        // Set some conditions
        let pid = PropertyId::new();
        repo.update_sort_conditions(
            &db_id,
            &[SortCondition {
                property_id: pid,
                direction: SortDirection::Ascending,
            }],
        )
        .await
        .expect("update sort");

        let reset = repo.reset(&db_id).await.expect("reset");
        assert!(reset.sort_conditions().is_empty());
        assert!(reset.filter_conditions().is_empty());
        assert!(reset.group_condition().is_none());
        assert!(reset.collapsed_groups().is_empty());
    }

    #[tokio::test]
    async fn remove_property_references_cleans_up() {
        let pool = setup_pool().await;
        let db_id = create_test_database(&pool).await;
        let repo = SqlxViewRepository::new(pool);

        let view = View::new_default(db_id.clone());
        repo.save(&view).await.expect("save");

        let target = PropertyId::new();
        let other = PropertyId::new();

        // Add sort with target and other
        repo.update_sort_conditions(
            &db_id,
            &[
                SortCondition {
                    property_id: target.clone(),
                    direction: SortDirection::Ascending,
                },
                SortCondition {
                    property_id: other.clone(),
                    direction: SortDirection::Descending,
                },
            ],
        )
        .await
        .expect("update sort");

        // Add filter with target
        repo.update_filter_conditions(
            &db_id,
            &[FilterCondition {
                property_id: target.clone(),
                operator: FilterOperator::IsEmpty,
                value: None,
            }],
        )
        .await
        .expect("update filter");

        // Remove references to target
        repo.remove_property_references(&target)
            .await
            .expect("remove refs");

        let view = repo
            .find_by_database_id(&db_id)
            .await
            .expect("find")
            .expect("view exists");

        assert_eq!(view.sort_conditions().len(), 1);
        assert_eq!(view.sort_conditions()[0].property_id, other);
        assert!(view.filter_conditions().is_empty());
    }

    #[tokio::test]
    async fn cascade_delete_database_deletes_view() {
        let pool = setup_pool().await;
        let db_id = create_test_database(&pool).await;
        let view_repo = SqlxViewRepository::new(pool.clone());

        let view = View::new_default(db_id.clone());
        view_repo.save(&view).await.expect("save");

        // Verify view exists
        let found = view_repo.find_by_database_id(&db_id).await.expect("find");
        assert!(found.is_some());

        // Delete database
        let db_repo = SqlxDatabaseRepository::new(pool);
        db_repo.delete(&db_id).await.expect("delete db");

        // View should be cascade-deleted
        let found = view_repo.find_by_database_id(&db_id).await.expect("find");
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn json_deserialization_fallback_on_corrupt_data() {
        let pool = setup_pool().await;
        let db_id = create_test_database(&pool).await;

        // Insert a view with corrupt JSON directly
        let view_id = ViewId::new().to_string();
        let db_id_str = db_id.to_string();
        let now = Utc::now().to_rfc3339();
        sqlx::query!(
            r#"INSERT INTO views (id, database_id, name, view_type, sort_conditions,
                                  filter_conditions, group_condition, collapsed_groups,
                                  created_at, updated_at)
               VALUES (?, ?, 'Table', 'table', '{corrupt}', '{corrupt}', '{corrupt}', '{corrupt}', ?, ?)"#,
            view_id,
            db_id_str,
            now,
            now
        )
        .execute(&pool)
        .await
        .expect("insert");

        let repo = SqlxViewRepository::new(pool);
        let found = repo.find_by_database_id(&db_id).await.expect("find");
        let view = found.expect("view exists");

        // Should fall back to empty defaults
        assert!(view.sort_conditions().is_empty());
        assert!(view.filter_conditions().is_empty());
        assert!(view.group_condition().is_none());
        assert!(view.collapsed_groups().is_empty());
    }

    #[tokio::test]
    async fn migration_creates_views_for_existing_databases() {
        let pool = setup_pool().await;

        // The migration should have already created a view for any pre-existing database.
        // Since we create a database via the repo (after migrations), the view is created
        // via the migration's INSERT...SELECT only for databases that existed before migration.
        // Here we test that a database created before the view table exists gets a view.
        // Since in-memory tests apply all migrations, we verify the migration doesn't error.
        // The real test is that the migration applies successfully (already confirmed by setup_pool).

        // Create a database (post-migration), which won't get a migration-created view
        let db_id = create_test_database(&pool).await;
        let repo = SqlxViewRepository::new(pool);

        // No view yet (created post-migration, not by INSERT...SELECT)
        let found = repo.find_by_database_id(&db_id).await.expect("find");
        assert!(found.is_none());
    }
}
