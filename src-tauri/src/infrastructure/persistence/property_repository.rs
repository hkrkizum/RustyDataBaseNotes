use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::domain::database::entity::DatabaseId;
use crate::domain::property::entity::{
    Property, PropertyConfig, PropertyId, PropertyName, PropertyType,
};
use crate::domain::property::error::PropertyError;
use crate::domain::property::repository::PropertyRepository;
use crate::ipc::error::CommandError;

/// SQLite-backed implementation of [`PropertyRepository`].
pub struct SqlxPropertyRepository {
    pool: SqlitePool,
}

impl SqlxPropertyRepository {
    /// Creates a new repository backed by the given connection pool.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

/// Parses a stored property-type string into a [`PropertyType`] enum.
fn parse_property_type(s: &str) -> Result<PropertyType, CommandError> {
    match s {
        "text" => Ok(PropertyType::Text),
        "number" => Ok(PropertyType::Number),
        "date" => Ok(PropertyType::Date),
        "select" => Ok(PropertyType::Select),
        "checkbox" => Ok(PropertyType::Checkbox),
        other => Err(PropertyError::InvalidType {
            value: other.to_owned(),
        }
        .into()),
    }
}

/// Helper to decode a column value, producing a [`StorageError`] on failure.
fn decode_error(
    column: &str,
    message: &str,
) -> crate::infrastructure::persistence::error::StorageError {
    crate::infrastructure::persistence::error::StorageError::from(sqlx::Error::ColumnDecode {
        index: column.to_owned(),
        source: Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            message.to_owned(),
        )),
    })
}

impl PropertyRepository for SqlxPropertyRepository {
    type Error = CommandError;

    async fn create(&self, property: &Property) -> Result<(), Self::Error> {
        let id = property.id().to_string();
        let database_id = property.database_id().to_string();
        let name = property.name().to_string();
        let property_type = property.property_type().to_string();
        let config = property
            .config()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| {
                crate::infrastructure::persistence::error::StorageError::from(
                    sqlx::Error::ColumnDecode {
                        index: "config".to_owned(),
                        source: Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            e.to_string(),
                        )),
                    },
                )
            })?;
        let position = property.position();
        let created_at = property.created_at().to_rfc3339();
        let updated_at = property.updated_at().to_rfc3339();

        sqlx::query!(
            r#"INSERT INTO properties (id, database_id, name, property_type, config, position, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
            id,
            database_id,
            name,
            property_type,
            config,
            position,
            created_at,
            updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            // Check for UNIQUE constraint violation on (database_id, name)
            if let sqlx::Error::Database(ref db_err) = e
                && db_err.message().contains("UNIQUE")
                && db_err.message().contains("properties")
            {
                return CommandError::Property(PropertyError::DuplicateName {
                    name: property.name().to_string(),
                    database_id: property.database_id().clone(),
                });
            }
            CommandError::Storage(
                crate::infrastructure::persistence::error::StorageError::from(e),
            )
        })?;

        Ok(())
    }

    async fn find_by_database_id(
        &self,
        database_id: &DatabaseId,
    ) -> Result<Vec<Property>, Self::Error> {
        let db_id_str = database_id.to_string();

        let rows = sqlx::query!(
            r#"SELECT id, database_id, name, property_type, config, position, created_at, updated_at
               FROM properties
               WHERE database_id = ?
               ORDER BY position ASC"#,
            db_id_str
        )
        .fetch_all(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        let mut properties = Vec::with_capacity(rows.len());
        for row in rows {
            let prop_id: PropertyId = row
                .id
                .parse()
                .map_err(|_| decode_error("id", "invalid property id"))?;
            let prop_db_id: DatabaseId = row
                .database_id
                .parse()
                .map_err(|_| decode_error("database_id", "invalid database id"))?;
            let prop_name = PropertyName::try_from(row.name)?;
            let prop_type = parse_property_type(&row.property_type)?;
            let prop_config: Option<PropertyConfig> = row
                .config
                .map(|s| serde_json::from_str(&s))
                .transpose()
                .map_err(|_| decode_error("config", "invalid property config JSON"))?;
            let created_at: DateTime<Utc> = row
                .created_at
                .parse()
                .map_err(|_| decode_error("created_at", "invalid created_at timestamp"))?;
            let updated_at: DateTime<Utc> = row
                .updated_at
                .parse()
                .map_err(|_| decode_error("updated_at", "invalid updated_at timestamp"))?;

            properties.push(Property::from_stored(
                prop_id,
                prop_db_id,
                prop_name,
                prop_type,
                prop_config,
                row.position,
                created_at,
                updated_at,
            ));
        }

        Ok(properties)
    }

    async fn find_by_id(&self, id: &PropertyId) -> Result<Property, Self::Error> {
        let id_str = id.to_string();

        let row = sqlx::query!(
            r#"SELECT id, database_id, name, property_type, config, position, created_at, updated_at
               FROM properties
               WHERE id = ?"#,
            id_str
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        match row {
            Some(row) => {
                let prop_id: PropertyId = row
                    .id
                    .parse()
                    .map_err(|_| decode_error("id", "invalid property id"))?;
                let prop_db_id: DatabaseId = row
                    .database_id
                    .parse()
                    .map_err(|_| decode_error("database_id", "invalid database id"))?;
                let prop_name = PropertyName::try_from(row.name)?;
                let prop_type = parse_property_type(&row.property_type)?;
                let prop_config: Option<PropertyConfig> = row
                    .config
                    .map(|s| serde_json::from_str(&s))
                    .transpose()
                    .map_err(|_| decode_error("config", "invalid property config JSON"))?;
                let created_at: DateTime<Utc> = row
                    .created_at
                    .parse()
                    .map_err(|_| decode_error("created_at", "invalid created_at timestamp"))?;
                let updated_at: DateTime<Utc> = row
                    .updated_at
                    .parse()
                    .map_err(|_| decode_error("updated_at", "invalid updated_at timestamp"))?;

                Ok(Property::from_stored(
                    prop_id,
                    prop_db_id,
                    prop_name,
                    prop_type,
                    prop_config,
                    row.position,
                    created_at,
                    updated_at,
                ))
            }
            None => Err(PropertyError::NotFound { id: id.clone() }.into()),
        }
    }

    async fn update_name(
        &self,
        id: &PropertyId,
        _name: &PropertyName,
    ) -> Result<Property, Self::Error> {
        // Stub: will be implemented in Phase 8
        Err(PropertyError::NotFound { id: id.clone() }.into())
    }

    async fn update_config(
        &self,
        id: &PropertyId,
        _config: &PropertyConfig,
    ) -> Result<Property, Self::Error> {
        // Stub: will be implemented in Phase 8
        Err(PropertyError::NotFound { id: id.clone() }.into())
    }

    async fn update_positions(&self, _updates: &[(PropertyId, i64)]) -> Result<(), Self::Error> {
        // Stub: will be implemented in Phase 8
        Ok(())
    }

    async fn delete(&self, id: &PropertyId) -> Result<(), Self::Error> {
        // Stub: will be implemented in Phase 8
        Err(PropertyError::NotFound { id: id.clone() }.into())
    }

    async fn count_by_database_id(&self, database_id: &DatabaseId) -> Result<usize, Self::Error> {
        let db_id_str = database_id.to_string();

        let row = sqlx::query!(
            "SELECT COUNT(*) as count FROM properties WHERE database_id = ?",
            db_id_str
        )
        .fetch_one(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        // SQLite COUNT(*) returns an integer; cast safely to usize.
        let count = row.count as usize;
        Ok(count)
    }

    async fn next_position(&self, database_id: &DatabaseId) -> Result<i64, Self::Error> {
        let db_id_str = database_id.to_string();

        let row = sqlx::query!(
            "SELECT COALESCE(MAX(position) + 1, 0) as next_pos FROM properties WHERE database_id = ?",
            db_id_str
        )
        .fetch_one(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        Ok(row.next_pos)
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::domain::database::entity::{Database, DatabaseTitle};
    use crate::domain::database::repository::DatabaseRepository;
    use crate::domain::property::entity::{DateMode, MAX_PROPERTIES, SelectOption, SelectOptionId};
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

    async fn create_test_database(pool: &SqlitePool) -> Database {
        let db_repo = SqlxDatabaseRepository::new(pool.clone());
        let title = DatabaseTitle::try_from("Test DB".to_owned()).expect("valid title");
        let database = Database::new(title);
        db_repo.create(&database).await.expect("create database");
        database
    }

    // ---- T026: Property domain tests ----

    #[test]
    fn property_create_valid() {
        let db_id = DatabaseId::new();
        let name = PropertyName::try_from("Status".to_owned()).expect("valid name");
        let prop = Property::new(db_id, name, PropertyType::Text, None, 0);
        assert!(prop.is_ok());
        let p = prop.expect("valid property");
        assert_eq!(p.property_type(), PropertyType::Text);
        assert!(p.config().is_none());
    }

    #[test]
    fn property_create_with_select_config() {
        let db_id = DatabaseId::new();
        let name = PropertyName::try_from("Category".to_owned()).expect("valid name");
        let config = Some(PropertyConfig::Select {
            options: vec![
                SelectOption {
                    id: SelectOptionId::new(),
                    value: "Alpha".to_owned(),
                },
                SelectOption {
                    id: SelectOptionId::new(),
                    value: "Beta".to_owned(),
                },
            ],
        });
        let prop = Property::new(db_id, name, PropertyType::Select, config, 0);
        assert!(prop.is_ok());
        let p = prop.expect("valid property");
        assert!(p.config().is_some());
    }

    #[test]
    fn property_name_empty_rejected() {
        let result = PropertyName::try_from(String::new());
        assert!(matches!(result, Err(PropertyError::NameEmpty)));
    }

    #[test]
    fn property_name_too_long_rejected() {
        let long = "a".repeat(101);
        let result = PropertyName::try_from(long);
        assert!(matches!(
            result,
            Err(PropertyError::NameTooLong { len: 101, max: 100 })
        ));
    }

    #[test]
    fn property_config_mismatch_rejected() {
        let db_id = DatabaseId::new();
        let name = PropertyName::try_from("Col".to_owned()).expect("valid name");
        // Text config for a Number type
        let config = Some(PropertyConfig::Text);
        let result = Property::new(db_id, name, PropertyType::Number, config, 0);
        assert!(matches!(result, Err(PropertyError::InvalidConfig { .. })));
    }

    #[tokio::test]
    async fn property_count_limit() {
        let pool = setup_pool().await;
        let repo = SqlxPropertyRepository::new(pool.clone());
        let database = create_test_database(&pool).await;
        let db_id = database.id().clone();

        let count = repo.count_by_database_id(&db_id).await.expect("count");
        assert_eq!(count, 0);
        assert!(count < MAX_PROPERTIES);
    }

    // ---- T027: Repository tests ----

    #[tokio::test]
    async fn create_and_find_by_id() {
        let pool = setup_pool().await;
        let repo = SqlxPropertyRepository::new(pool.clone());
        let database = create_test_database(&pool).await;

        let name = PropertyName::try_from("Title".to_owned()).expect("valid name");
        let prop = Property::new(database.id().clone(), name, PropertyType::Text, None, 0)
            .expect("valid property");
        let prop_id = prop.id().clone();

        repo.create(&prop).await.expect("create should succeed");

        let found = repo
            .find_by_id(&prop_id)
            .await
            .expect("should find property");
        assert_eq!(found.id(), &prop_id);
        assert_eq!(found.name().as_str(), "Title");
        assert_eq!(found.property_type(), PropertyType::Text);
        assert!(found.config().is_none());
        assert_eq!(found.position(), 0);
    }

    #[tokio::test]
    async fn find_by_database_id_ordered_by_position() {
        let pool = setup_pool().await;
        let repo = SqlxPropertyRepository::new(pool.clone());
        let database = create_test_database(&pool).await;
        let db_id = database.id().clone();

        // Create properties at positions 2, 0, 1
        for (name, pos) in [("Col C", 2), ("Col A", 0), ("Col B", 1)] {
            let pname = PropertyName::try_from(name.to_owned()).expect("valid name");
            let prop = Property::new(db_id.clone(), pname, PropertyType::Text, None, pos)
                .expect("valid property");
            repo.create(&prop).await.expect("create");
        }

        let props = repo.find_by_database_id(&db_id).await.expect("find");
        assert_eq!(props.len(), 3);
        assert_eq!(props[0].name().as_str(), "Col A");
        assert_eq!(props[0].position(), 0);
        assert_eq!(props[1].name().as_str(), "Col B");
        assert_eq!(props[1].position(), 1);
        assert_eq!(props[2].name().as_str(), "Col C");
        assert_eq!(props[2].position(), 2);
    }

    #[tokio::test]
    async fn find_by_database_id_empty() {
        let pool = setup_pool().await;
        let repo = SqlxPropertyRepository::new(pool.clone());
        let database = create_test_database(&pool).await;

        let props = repo.find_by_database_id(database.id()).await.expect("find");
        assert!(props.is_empty());
    }

    #[tokio::test]
    async fn count_by_database_id() {
        let pool = setup_pool().await;
        let repo = SqlxPropertyRepository::new(pool.clone());
        let database = create_test_database(&pool).await;
        let db_id = database.id().clone();

        let count = repo.count_by_database_id(&db_id).await.expect("count");
        assert_eq!(count, 0);

        let name = PropertyName::try_from("Col".to_owned()).expect("valid name");
        let prop = Property::new(db_id.clone(), name, PropertyType::Text, None, 0)
            .expect("valid property");
        repo.create(&prop).await.expect("create");

        let count = repo.count_by_database_id(&db_id).await.expect("count");
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn next_position_empty_db() {
        let pool = setup_pool().await;
        let repo = SqlxPropertyRepository::new(pool.clone());
        let database = create_test_database(&pool).await;

        let pos = repo
            .next_position(database.id())
            .await
            .expect("next_position");
        assert_eq!(pos, 0);
    }

    #[tokio::test]
    async fn next_position_after_inserts() {
        let pool = setup_pool().await;
        let repo = SqlxPropertyRepository::new(pool.clone());
        let database = create_test_database(&pool).await;
        let db_id = database.id().clone();

        for (name, pos) in [("A", 0), ("B", 1), ("C", 2)] {
            let pname = PropertyName::try_from(name.to_owned()).expect("valid name");
            let prop = Property::new(db_id.clone(), pname, PropertyType::Text, None, pos)
                .expect("valid property");
            repo.create(&prop).await.expect("create");
        }

        let pos = repo.next_position(&db_id).await.expect("next_position");
        assert_eq!(pos, 3);
    }

    #[tokio::test]
    async fn create_with_select_config_roundtrip() {
        let pool = setup_pool().await;
        let repo = SqlxPropertyRepository::new(pool.clone());
        let database = create_test_database(&pool).await;

        let opt1 = SelectOption {
            id: SelectOptionId::new(),
            value: "Alpha".to_owned(),
        };
        let opt2 = SelectOption {
            id: SelectOptionId::new(),
            value: "Beta".to_owned(),
        };
        let config = Some(PropertyConfig::Select {
            options: vec![opt1.clone(), opt2.clone()],
        });

        let name = PropertyName::try_from("Status".to_owned()).expect("valid name");
        let prop = Property::new(database.id().clone(), name, PropertyType::Select, config, 0)
            .expect("valid property");
        let prop_id = prop.id().clone();

        repo.create(&prop).await.expect("create");

        let found = repo.find_by_id(&prop_id).await.expect("find");
        assert_eq!(found.property_type(), PropertyType::Select);
        let cfg = found.config().expect("should have config");
        assert!(
            matches!(cfg, PropertyConfig::Select { options } if options.len() == 2
                && options[0].value == "Alpha"
                && options[1].value == "Beta"
            ),
            "expected Select config with Alpha and Beta options"
        );
    }

    #[tokio::test]
    async fn create_with_date_config_roundtrip() {
        let pool = setup_pool().await;
        let repo = SqlxPropertyRepository::new(pool.clone());
        let database = create_test_database(&pool).await;

        let config = Some(PropertyConfig::Date {
            mode: DateMode::DateTime,
        });
        let name = PropertyName::try_from("Due Date".to_owned()).expect("valid name");
        let prop = Property::new(database.id().clone(), name, PropertyType::Date, config, 0)
            .expect("valid property");
        let prop_id = prop.id().clone();

        repo.create(&prop).await.expect("create");

        let found = repo.find_by_id(&prop_id).await.expect("find");
        assert_eq!(found.property_type(), PropertyType::Date);
        let cfg = found.config().expect("should have config");
        assert!(matches!(
            cfg,
            PropertyConfig::Date {
                mode: DateMode::DateTime
            }
        ));
    }

    #[tokio::test]
    async fn duplicate_name_rejected() {
        let pool = setup_pool().await;
        let repo = SqlxPropertyRepository::new(pool.clone());
        let database = create_test_database(&pool).await;
        let db_id = database.id().clone();

        let name1 = PropertyName::try_from("Title".to_owned()).expect("valid name");
        let prop1 = Property::new(db_id.clone(), name1, PropertyType::Text, None, 0)
            .expect("valid property");
        repo.create(&prop1).await.expect("create first");

        let name2 = PropertyName::try_from("Title".to_owned()).expect("valid name");
        let prop2 =
            Property::new(db_id, name2, PropertyType::Number, None, 1).expect("valid property");
        let result = repo.create(&prop2).await;
        assert!(matches!(
            result,
            Err(CommandError::Property(PropertyError::DuplicateName { .. }))
        ));
    }
}
