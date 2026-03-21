use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::domain::database::entity::DatabaseId;
use crate::domain::page::entity::PageId;
use crate::domain::property::entity::{PropertyId, PropertyValue, PropertyValueId};
use crate::domain::property::repository::PropertyValueRepository;
use crate::ipc::error::CommandError;

/// SQLite-backed implementation of [`PropertyValueRepository`].
pub struct SqlxPropertyValueRepository {
    pool: SqlitePool,
}

impl SqlxPropertyValueRepository {
    /// Creates a new repository backed by the given connection pool.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
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

impl PropertyValueRepository for SqlxPropertyValueRepository {
    type Error = CommandError;

    async fn upsert(&self, value: &PropertyValue) -> Result<(), Self::Error> {
        let id = value.id().to_string();
        let page_id = value.page_id().to_string();
        let property_id = value.property_id().to_string();
        let text_value = value.text_value().map(|s| s.to_owned());
        let number_value = value.number_value();
        let date_value = value.date_value().map(|d| d.to_rfc3339());
        let boolean_value = value.boolean_value().map(|b| if b { 1_i32 } else { 0_i32 });
        let created_at = value.created_at().to_rfc3339();
        let updated_at = value.updated_at().to_rfc3339();

        sqlx::query!(
            r#"INSERT INTO property_values (id, page_id, property_id, text_value, number_value, date_value, boolean_value, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
               ON CONFLICT (page_id, property_id) DO UPDATE SET
                   text_value = excluded.text_value,
                   number_value = excluded.number_value,
                   date_value = excluded.date_value,
                   boolean_value = excluded.boolean_value,
                   updated_at = excluded.updated_at"#,
            id,
            page_id,
            property_id,
            text_value,
            number_value,
            date_value,
            boolean_value,
            created_at,
            updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        Ok(())
    }

    async fn find_by_page_and_property(
        &self,
        page_id: &PageId,
        property_id: &PropertyId,
    ) -> Result<Option<PropertyValue>, Self::Error> {
        let page_id_str = page_id.to_string();
        let property_id_str = property_id.to_string();

        let row = sqlx::query!(
            r#"SELECT id, page_id, property_id, text_value, number_value, date_value, boolean_value, created_at, updated_at
               FROM property_values
               WHERE page_id = ? AND property_id = ?"#,
            page_id_str,
            property_id_str
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        match row {
            Some(row) => {
                let pv = map_row(
                    &row.id,
                    &row.page_id,
                    &row.property_id,
                    row.text_value.as_deref(),
                    row.number_value,
                    row.date_value.as_deref(),
                    row.boolean_value,
                    &row.created_at,
                    &row.updated_at,
                )?;
                Ok(Some(pv))
            }
            None => Ok(None),
        }
    }

    async fn find_by_page_id(&self, page_id: &PageId) -> Result<Vec<PropertyValue>, Self::Error> {
        let page_id_str = page_id.to_string();

        let rows = sqlx::query!(
            r#"SELECT id, page_id, property_id, text_value, number_value, date_value, boolean_value, created_at, updated_at
               FROM property_values
               WHERE page_id = ?"#,
            page_id_str
        )
        .fetch_all(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        let mut values = Vec::with_capacity(rows.len());
        for row in rows {
            values.push(map_row(
                &row.id,
                &row.page_id,
                &row.property_id,
                row.text_value.as_deref(),
                row.number_value,
                row.date_value.as_deref(),
                row.boolean_value,
                &row.created_at,
                &row.updated_at,
            )?);
        }
        Ok(values)
    }

    async fn find_by_property_id(
        &self,
        property_id: &PropertyId,
    ) -> Result<Vec<PropertyValue>, Self::Error> {
        let property_id_str = property_id.to_string();

        let rows = sqlx::query!(
            r#"SELECT id, page_id, property_id, text_value, number_value, date_value, boolean_value, created_at, updated_at
               FROM property_values
               WHERE property_id = ?"#,
            property_id_str
        )
        .fetch_all(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        let mut values = Vec::with_capacity(rows.len());
        for row in rows {
            values.push(map_row(
                &row.id,
                &row.page_id,
                &row.property_id,
                row.text_value.as_deref(),
                row.number_value,
                row.date_value.as_deref(),
                row.boolean_value,
                &row.created_at,
                &row.updated_at,
            )?);
        }
        Ok(values)
    }

    async fn delete_by_page_and_property(
        &self,
        page_id: &PageId,
        property_id: &PropertyId,
    ) -> Result<(), Self::Error> {
        let page_id_str = page_id.to_string();
        let property_id_str = property_id.to_string();

        sqlx::query!(
            "DELETE FROM property_values WHERE page_id = ? AND property_id = ?",
            page_id_str,
            property_id_str
        )
        .execute(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        Ok(())
    }

    async fn delete_by_page_and_database(
        &self,
        page_id: &PageId,
        database_id: &DatabaseId,
    ) -> Result<(), Self::Error> {
        let page_id_str = page_id.to_string();
        let database_id_str = database_id.to_string();

        sqlx::query!(
            r#"DELETE FROM property_values WHERE page_id = ? AND property_id IN (
                   SELECT id FROM properties WHERE database_id = ?
               )"#,
            page_id_str,
            database_id_str
        )
        .execute(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        Ok(())
    }

    async fn reset_select_option(
        &self,
        property_id: &PropertyId,
        option_id: &str,
    ) -> Result<(), Self::Error> {
        let property_id_str = property_id.to_string();
        let updated_at = Utc::now().to_rfc3339();

        sqlx::query!(
            r#"UPDATE property_values SET text_value = NULL, updated_at = ?
               WHERE property_id = ? AND text_value = ?"#,
            updated_at,
            property_id_str,
            option_id
        )
        .execute(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        Ok(())
    }

    async fn find_all_for_database(
        &self,
        database_id: &DatabaseId,
    ) -> Result<Vec<PropertyValue>, Self::Error> {
        let database_id_str = database_id.to_string();

        let rows = sqlx::query!(
            r#"SELECT pv.id, pv.page_id, pv.property_id, pv.text_value, pv.number_value, pv.date_value, pv.boolean_value, pv.created_at, pv.updated_at
               FROM property_values pv
               INNER JOIN pages p ON pv.page_id = p.id
               WHERE p.database_id = ?"#,
            database_id_str
        )
        .fetch_all(&self.pool)
        .await
        .map_err(crate::infrastructure::persistence::error::StorageError::from)?;

        let mut values = Vec::with_capacity(rows.len());
        for row in rows {
            values.push(map_row(
                &row.id,
                &row.page_id,
                &row.property_id,
                row.text_value.as_deref(),
                row.number_value,
                row.date_value.as_deref(),
                row.boolean_value,
                &row.created_at,
                &row.updated_at,
            )?);
        }
        Ok(values)
    }
}

/// Maps raw SQL row fields into a [`PropertyValue`] domain entity.
#[allow(clippy::too_many_arguments)]
fn map_row(
    id: &str,
    page_id: &str,
    property_id: &str,
    text_value: Option<&str>,
    number_value: Option<f64>,
    date_value: Option<&str>,
    boolean_value: Option<i64>,
    created_at: &str,
    updated_at: &str,
) -> Result<PropertyValue, CommandError> {
    let pv_id: PropertyValueId = id
        .parse()
        .map_err(|_| decode_error("id", "invalid property value id"))?;
    let pg_id: PageId = page_id
        .parse()
        .map_err(|_| decode_error("page_id", "invalid page id"))?;
    let prop_id: PropertyId = property_id
        .parse()
        .map_err(|_| decode_error("property_id", "invalid property id"))?;
    let text = text_value.map(|s| s.to_owned());
    let date: Option<DateTime<Utc>> = date_value
        .map(|s| s.parse::<DateTime<Utc>>())
        .transpose()
        .map_err(|_| decode_error("date_value", "invalid date_value timestamp"))?;
    let boolean = boolean_value.map(|v| v != 0);
    let ca: DateTime<Utc> = created_at
        .parse()
        .map_err(|_| decode_error("created_at", "invalid created_at timestamp"))?;
    let ua: DateTime<Utc> = updated_at
        .parse()
        .map_err(|_| decode_error("updated_at", "invalid updated_at timestamp"))?;

    Ok(PropertyValue::from_stored(
        pv_id,
        pg_id,
        prop_id,
        text,
        number_value,
        date,
        boolean,
        ca,
        ua,
    ))
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::domain::database::entity::{Database, DatabaseTitle};
    use crate::domain::database::repository::DatabaseRepository;
    use crate::domain::page::entity::{Page, PageTitle};
    use crate::domain::page::repository::PageRepository;
    use crate::domain::property::entity::{
        Property, PropertyConfig, PropertyName, PropertyType, PropertyValueInput, SelectOption,
        SelectOptionId,
    };
    use crate::domain::property::repository::PropertyRepository;
    use crate::infrastructure::persistence::database_repository::SqlxDatabaseRepository;
    use crate::infrastructure::persistence::page_repository::SqlxPageRepository;
    use crate::infrastructure::persistence::property_repository::SqlxPropertyRepository;

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

    async fn create_test_property(pool: &SqlitePool, db_id: &DatabaseId) -> Property {
        let prop_repo = SqlxPropertyRepository::new(pool.clone());
        let name = PropertyName::try_from("Status".to_owned()).expect("valid name");
        let prop = Property::new(db_id.clone(), name, PropertyType::Text, None, 0)
            .expect("valid property");
        prop_repo.create(&prop).await.expect("create property");
        prop
    }

    async fn create_test_page(pool: &SqlitePool, db_id: &DatabaseId) -> Page {
        let page_repo = SqlxPageRepository::new(pool.clone());
        let title = PageTitle::try_from("Test Page".to_owned()).expect("valid title");
        let page = Page::new(title);
        page_repo.create(&page).await.expect("create page");
        page_repo
            .set_database_id(page.id(), Some(db_id))
            .await
            .expect("set database_id");
        page
    }

    // T043: upsert_insert_and_find
    #[tokio::test]
    async fn upsert_insert_and_find() {
        let pool = setup_pool().await;
        let database = create_test_database(&pool).await;
        let prop = create_test_property(&pool, database.id()).await;
        let page = create_test_page(&pool, database.id()).await;

        let pv = PropertyValue::new_validated(
            page.id().clone(),
            prop.id().clone(),
            PropertyType::Text,
            None,
            PropertyValueInput::Text("hello".to_owned()),
        )
        .expect("valid value");

        let repo = SqlxPropertyValueRepository::new(pool.clone());
        repo.upsert(&pv).await.expect("upsert");

        let found = repo
            .find_by_page_and_property(page.id(), prop.id())
            .await
            .expect("find");
        assert!(found.is_some());
        let found = found.expect("has value");
        assert_eq!(found.text_value(), Some("hello"));
        assert_eq!(found.page_id(), page.id());
        assert_eq!(found.property_id(), prop.id());
    }

    // T043: upsert_update_existing
    #[tokio::test]
    async fn upsert_update_existing() {
        let pool = setup_pool().await;
        let database = create_test_database(&pool).await;
        let prop = create_test_property(&pool, database.id()).await;
        let page = create_test_page(&pool, database.id()).await;

        let pv1 = PropertyValue::new_validated(
            page.id().clone(),
            prop.id().clone(),
            PropertyType::Text,
            None,
            PropertyValueInput::Text("first".to_owned()),
        )
        .expect("valid value");

        let repo = SqlxPropertyValueRepository::new(pool.clone());
        repo.upsert(&pv1).await.expect("upsert 1");

        let pv2 = PropertyValue::new_validated(
            page.id().clone(),
            prop.id().clone(),
            PropertyType::Text,
            None,
            PropertyValueInput::Text("second".to_owned()),
        )
        .expect("valid value");
        repo.upsert(&pv2).await.expect("upsert 2");

        let found = repo
            .find_by_page_and_property(page.id(), prop.id())
            .await
            .expect("find");
        assert!(found.is_some());
        let found = found.expect("has value");
        assert_eq!(found.text_value(), Some("second"));
    }

    // T043: find_by_page_and_property_none
    #[tokio::test]
    async fn find_by_page_and_property_none() {
        let pool = setup_pool().await;
        let repo = SqlxPropertyValueRepository::new(pool);

        let found = repo
            .find_by_page_and_property(&PageId::new(), &PropertyId::new())
            .await
            .expect("find");
        assert!(found.is_none());
    }

    // T043: find_by_page_id
    #[tokio::test]
    async fn find_by_page_id_returns_values() {
        let pool = setup_pool().await;
        let database = create_test_database(&pool).await;
        let prop1 = create_test_property(&pool, database.id()).await;

        let prop_repo = SqlxPropertyRepository::new(pool.clone());
        let name2 = PropertyName::try_from("Count".to_owned()).expect("valid name");
        let prop2 = Property::new(database.id().clone(), name2, PropertyType::Number, None, 1)
            .expect("valid property");
        prop_repo.create(&prop2).await.expect("create property 2");

        let page = create_test_page(&pool, database.id()).await;

        let pv1 = PropertyValue::new_validated(
            page.id().clone(),
            prop1.id().clone(),
            PropertyType::Text,
            None,
            PropertyValueInput::Text("hello".to_owned()),
        )
        .expect("valid value");

        let pv2 = PropertyValue::new_validated(
            page.id().clone(),
            prop2.id().clone(),
            PropertyType::Number,
            None,
            PropertyValueInput::Number(42.0),
        )
        .expect("valid value");

        let repo = SqlxPropertyValueRepository::new(pool.clone());
        repo.upsert(&pv1).await.expect("upsert 1");
        repo.upsert(&pv2).await.expect("upsert 2");

        let values = repo.find_by_page_id(page.id()).await.expect("find");
        assert_eq!(values.len(), 2);
    }

    // T043: delete_by_page_and_property
    #[tokio::test]
    async fn delete_by_page_and_property_removes_value() {
        let pool = setup_pool().await;
        let database = create_test_database(&pool).await;
        let prop = create_test_property(&pool, database.id()).await;
        let page = create_test_page(&pool, database.id()).await;

        let pv = PropertyValue::new_validated(
            page.id().clone(),
            prop.id().clone(),
            PropertyType::Text,
            None,
            PropertyValueInput::Text("to delete".to_owned()),
        )
        .expect("valid value");

        let repo = SqlxPropertyValueRepository::new(pool.clone());
        repo.upsert(&pv).await.expect("upsert");

        repo.delete_by_page_and_property(page.id(), prop.id())
            .await
            .expect("delete");

        let found = repo
            .find_by_page_and_property(page.id(), prop.id())
            .await
            .expect("find");
        assert!(found.is_none());
    }

    // T043: delete idempotent — no error if value does not exist
    #[tokio::test]
    async fn delete_by_page_and_property_no_op_if_absent() {
        let pool = setup_pool().await;
        let repo = SqlxPropertyValueRepository::new(pool);

        let result = repo
            .delete_by_page_and_property(&PageId::new(), &PropertyId::new())
            .await;
        assert!(result.is_ok());
    }

    // T043: find_all_for_database
    #[tokio::test]
    async fn find_all_for_database_returns_values() {
        let pool = setup_pool().await;
        let database = create_test_database(&pool).await;
        let prop = create_test_property(&pool, database.id()).await;
        let page = create_test_page(&pool, database.id()).await;

        let pv = PropertyValue::new_validated(
            page.id().clone(),
            prop.id().clone(),
            PropertyType::Text,
            None,
            PropertyValueInput::Text("hello".to_owned()),
        )
        .expect("valid value");

        let repo = SqlxPropertyValueRepository::new(pool.clone());
        repo.upsert(&pv).await.expect("upsert");

        let values = repo
            .find_all_for_database(database.id())
            .await
            .expect("find");
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].text_value(), Some("hello"));
    }

    // T043: find_by_property_id
    #[tokio::test]
    async fn find_by_property_id_returns_values() {
        let pool = setup_pool().await;
        let database = create_test_database(&pool).await;
        let prop = create_test_property(&pool, database.id()).await;

        let page1 = create_test_page(&pool, database.id()).await;
        let page_repo = SqlxPageRepository::new(pool.clone());
        let title2 = PageTitle::try_from("Page 2".to_owned()).expect("valid");
        let page2 = Page::new(title2);
        page_repo.create(&page2).await.expect("create page 2");
        page_repo
            .set_database_id(page2.id(), Some(database.id()))
            .await
            .expect("set db_id");

        let pv1 = PropertyValue::new_validated(
            page1.id().clone(),
            prop.id().clone(),
            PropertyType::Text,
            None,
            PropertyValueInput::Text("a".to_owned()),
        )
        .expect("valid");
        let pv2 = PropertyValue::new_validated(
            page2.id().clone(),
            prop.id().clone(),
            PropertyType::Text,
            None,
            PropertyValueInput::Text("b".to_owned()),
        )
        .expect("valid");

        let repo = SqlxPropertyValueRepository::new(pool.clone());
        repo.upsert(&pv1).await.expect("upsert 1");
        repo.upsert(&pv2).await.expect("upsert 2");

        let values = repo.find_by_property_id(prop.id()).await.expect("find");
        assert_eq!(values.len(), 2);
    }

    // T043: boolean and date value roundtrip
    #[tokio::test]
    async fn boolean_and_date_roundtrip() {
        let pool = setup_pool().await;
        let database = create_test_database(&pool).await;
        let db_id = database.id().clone();

        let prop_repo = SqlxPropertyRepository::new(pool.clone());

        let name_bool = PropertyName::try_from("Done".to_owned()).expect("valid name");
        let prop_bool = Property::new(db_id.clone(), name_bool, PropertyType::Checkbox, None, 0)
            .expect("valid");
        prop_repo.create(&prop_bool).await.expect("create");

        let name_date = PropertyName::try_from("Due".to_owned()).expect("valid name");
        let prop_date =
            Property::new(db_id.clone(), name_date, PropertyType::Date, None, 1).expect("valid");
        prop_repo.create(&prop_date).await.expect("create");

        let page = create_test_page(&pool, &db_id).await;

        let pv_bool = PropertyValue::new_validated(
            page.id().clone(),
            prop_bool.id().clone(),
            PropertyType::Checkbox,
            None,
            PropertyValueInput::Checkbox(true),
        )
        .expect("valid");

        let now = Utc::now();
        let pv_date = PropertyValue::new_validated(
            page.id().clone(),
            prop_date.id().clone(),
            PropertyType::Date,
            None,
            PropertyValueInput::Date(now),
        )
        .expect("valid");

        let repo = SqlxPropertyValueRepository::new(pool.clone());
        repo.upsert(&pv_bool).await.expect("upsert bool");
        repo.upsert(&pv_date).await.expect("upsert date");

        let found_bool = repo
            .find_by_page_and_property(page.id(), prop_bool.id())
            .await
            .expect("find bool");
        assert_eq!(
            found_bool.as_ref().and_then(|v| v.boolean_value()),
            Some(true)
        );

        let found_date = repo
            .find_by_page_and_property(page.id(), prop_date.id())
            .await
            .expect("find date");
        assert!(found_date.as_ref().and_then(|v| v.date_value()).is_some());
    }

    // T043: number value roundtrip
    #[tokio::test]
    async fn number_value_roundtrip() {
        let pool = setup_pool().await;
        let database = create_test_database(&pool).await;

        let prop_repo = SqlxPropertyRepository::new(pool.clone());
        let name = PropertyName::try_from("Amount".to_owned()).expect("valid name");
        let prop = Property::new(database.id().clone(), name, PropertyType::Number, None, 0)
            .expect("valid");
        prop_repo.create(&prop).await.expect("create");

        let page = create_test_page(&pool, database.id()).await;

        let pv = PropertyValue::new_validated(
            page.id().clone(),
            prop.id().clone(),
            PropertyType::Number,
            None,
            PropertyValueInput::Number(42.5),
        )
        .expect("valid");

        let repo = SqlxPropertyValueRepository::new(pool.clone());
        repo.upsert(&pv).await.expect("upsert");

        let found = repo
            .find_by_page_and_property(page.id(), prop.id())
            .await
            .expect("find");
        let found = found.expect("has value");
        let n = found.number_value().expect("has number");
        assert!((n - 42.5).abs() < f64::EPSILON);
    }

    // T043: select value roundtrip
    #[tokio::test]
    async fn select_value_roundtrip() {
        let pool = setup_pool().await;
        let database = create_test_database(&pool).await;

        let opt = SelectOption {
            id: SelectOptionId::new(),
            value: "Alpha".to_owned(),
        };
        let opt_id_str = opt.id.to_string();
        let config = PropertyConfig::Select { options: vec![opt] };

        let prop_repo = SqlxPropertyRepository::new(pool.clone());
        let name = PropertyName::try_from("Category".to_owned()).expect("valid name");
        let prop = Property::new(
            database.id().clone(),
            name,
            PropertyType::Select,
            Some(config.clone()),
            0,
        )
        .expect("valid");
        prop_repo.create(&prop).await.expect("create");

        let page = create_test_page(&pool, database.id()).await;

        let pv = PropertyValue::new_validated(
            page.id().clone(),
            prop.id().clone(),
            PropertyType::Select,
            Some(&config),
            PropertyValueInput::Select(opt_id_str.clone()),
        )
        .expect("valid");

        let repo = SqlxPropertyValueRepository::new(pool.clone());
        repo.upsert(&pv).await.expect("upsert");

        let found = repo
            .find_by_page_and_property(page.id(), prop.id())
            .await
            .expect("find");
        let found = found.expect("has value");
        assert_eq!(found.text_value(), Some(opt_id_str.as_str()));
    }

    // T043: reset_select_option
    #[tokio::test]
    async fn reset_select_option_clears_matching() {
        let pool = setup_pool().await;
        let database = create_test_database(&pool).await;

        let opt = SelectOption {
            id: SelectOptionId::new(),
            value: "Alpha".to_owned(),
        };
        let opt_id_str = opt.id.to_string();
        let config = PropertyConfig::Select { options: vec![opt] };

        let prop_repo = SqlxPropertyRepository::new(pool.clone());
        let name = PropertyName::try_from("Category".to_owned()).expect("valid name");
        let prop = Property::new(
            database.id().clone(),
            name,
            PropertyType::Select,
            Some(config.clone()),
            0,
        )
        .expect("valid");
        prop_repo.create(&prop).await.expect("create");

        let page = create_test_page(&pool, database.id()).await;

        let pv = PropertyValue::new_validated(
            page.id().clone(),
            prop.id().clone(),
            PropertyType::Select,
            Some(&config),
            PropertyValueInput::Select(opt_id_str.clone()),
        )
        .expect("valid");

        let repo = SqlxPropertyValueRepository::new(pool.clone());
        repo.upsert(&pv).await.expect("upsert");

        repo.reset_select_option(prop.id(), &opt_id_str)
            .await
            .expect("reset");

        let found = repo
            .find_by_page_and_property(page.id(), prop.id())
            .await
            .expect("find");
        let found = found.expect("has value");
        assert_eq!(found.text_value(), None);
    }

    // T077: no orphaned values after property delete
    #[tokio::test]
    async fn no_orphaned_values_after_property_delete() {
        let pool = setup_pool().await;
        let database = create_test_database(&pool).await;
        let prop = create_test_property(&pool, database.id()).await;
        let page = create_test_page(&pool, database.id()).await;

        let pv = PropertyValue::new_validated(
            page.id().clone(),
            prop.id().clone(),
            PropertyType::Text,
            None,
            PropertyValueInput::Text("will be orphaned?".to_owned()),
        )
        .expect("valid value");

        let repo = SqlxPropertyValueRepository::new(pool.clone());
        repo.upsert(&pv).await.expect("upsert");

        // Verify value exists before delete
        let before = repo
            .find_by_property_id(prop.id())
            .await
            .expect("find before");
        assert_eq!(before.len(), 1);

        // Delete the property
        let prop_repo = SqlxPropertyRepository::new(pool.clone());
        prop_repo.delete(prop.id()).await.expect("delete property");

        // Property values should be cascade-deleted
        let after = repo
            .find_by_property_id(prop.id())
            .await
            .expect("find after");
        assert!(
            after.is_empty(),
            "property_values should be cascade-deleted when property is deleted"
        );
    }

    // T077: no orphaned values after page delete
    #[tokio::test]
    async fn no_orphaned_values_after_page_delete() {
        let pool = setup_pool().await;
        let database = create_test_database(&pool).await;
        let prop = create_test_property(&pool, database.id()).await;
        let page = create_test_page(&pool, database.id()).await;

        let pv = PropertyValue::new_validated(
            page.id().clone(),
            prop.id().clone(),
            PropertyType::Text,
            None,
            PropertyValueInput::Text("will be orphaned?".to_owned()),
        )
        .expect("valid value");

        let repo = SqlxPropertyValueRepository::new(pool.clone());
        repo.upsert(&pv).await.expect("upsert");

        // Verify value exists before delete
        let before = repo.find_by_page_id(page.id()).await.expect("find before");
        assert_eq!(before.len(), 1);

        // Delete the page
        let page_repo = SqlxPageRepository::new(pool.clone());
        page_repo.delete(page.id()).await.expect("delete page");

        // Property values should be cascade-deleted
        let after = repo.find_by_page_id(page.id()).await.expect("find after");
        assert!(
            after.is_empty(),
            "property_values should be cascade-deleted when page is deleted"
        );
    }

    // T077: no orphaned values after database delete
    #[tokio::test]
    async fn no_orphaned_values_after_database_delete() {
        let pool = setup_pool().await;
        let database = create_test_database(&pool).await;
        let prop = create_test_property(&pool, database.id()).await;
        let page = create_test_page(&pool, database.id()).await;

        let pv = PropertyValue::new_validated(
            page.id().clone(),
            prop.id().clone(),
            PropertyType::Text,
            None,
            PropertyValueInput::Text("cascade through db".to_owned()),
        )
        .expect("valid value");

        let repo = SqlxPropertyValueRepository::new(pool.clone());
        repo.upsert(&pv).await.expect("upsert");

        // Delete the database
        let db_repo = SqlxDatabaseRepository::new(pool.clone());
        db_repo
            .delete(database.id())
            .await
            .expect("delete database");

        // Properties should be cascade-deleted
        let prop_repo = SqlxPropertyRepository::new(pool.clone());
        let props = prop_repo
            .find_by_database_id(database.id())
            .await
            .expect("find props");
        assert!(
            props.is_empty(),
            "properties should be cascade-deleted when database is deleted"
        );

        // Property values should be cascade-deleted (via property CASCADE)
        let values = repo
            .find_by_property_id(prop.id())
            .await
            .expect("find values");
        assert!(
            values.is_empty(),
            "property_values should be cascade-deleted when database is deleted"
        );

        // Page should still exist with database_id = NULL
        let page_repo = SqlxPageRepository::new(pool.clone());
        let found_page = page_repo
            .find_by_id(page.id())
            .await
            .expect("page should still exist");
        assert!(
            found_page.database_id().is_none(),
            "page.database_id should be NULL after database delete"
        );
    }

    // T078: Performance — 100 pages x 10 properties
    #[tokio::test]
    async fn performance_100_pages_10_properties() {
        let pool = setup_pool().await;
        let database = create_test_database(&pool).await;
        let db_id = database.id().clone();

        // Create 10 properties
        let prop_repo = SqlxPropertyRepository::new(pool.clone());
        let mut prop_ids = Vec::new();
        for i in 0..10 {
            let name = PropertyName::try_from(format!("Prop {i}")).expect("valid name");
            let prop = Property::new(db_id.clone(), name, PropertyType::Text, None, i64::from(i))
                .expect("valid property");
            prop_ids.push(prop.id().clone());
            prop_repo.create(&prop).await.expect("create property");
        }

        // Create 100 pages with property values
        let page_repo = SqlxPageRepository::new(pool.clone());
        let pv_repo = SqlxPropertyValueRepository::new(pool.clone());
        for i in 0..100 {
            let title = PageTitle::try_from(format!("Page {i}")).expect("valid title");
            let page = Page::new(title);
            page_repo.create(&page).await.expect("create page");
            page_repo
                .set_database_id(page.id(), Some(&db_id))
                .await
                .expect("set db_id");
            for prop_id in &prop_ids {
                let pv = PropertyValue::new_validated(
                    page.id().clone(),
                    prop_id.clone(),
                    PropertyType::Text,
                    None,
                    PropertyValueInput::Text(format!("val-{i}")),
                )
                .expect("valid value");
                pv_repo.upsert(&pv).await.expect("upsert");
            }
        }

        // Measure: find_all_for_database should complete in < 1s
        let start = std::time::Instant::now();
        let values = pv_repo
            .find_all_for_database(&db_id)
            .await
            .expect("find_all_for_database");
        let elapsed = start.elapsed();

        assert_eq!(values.len(), 1000); // 100 pages x 10 properties
        assert!(
            elapsed < std::time::Duration::from_secs(1),
            "find_all_for_database took {elapsed:?}"
        );
    }

    // T079: find_all_for_database returns empty when no values exist
    #[tokio::test]
    async fn find_all_for_database_empty() {
        let pool = setup_pool().await;
        let database = create_test_database(&pool).await;

        let repo = SqlxPropertyValueRepository::new(pool.clone());
        let values = repo
            .find_all_for_database(database.id())
            .await
            .expect("find_all_for_database");
        assert!(
            values.is_empty(),
            "empty database should have no property values"
        );
    }

    // T079: select option limit 100 verified at domain level
    #[tokio::test]
    async fn select_option_limit_100_at_domain() {
        // Create 100 options (the maximum)
        let options: Vec<SelectOption> = (0..100)
            .map(|i| SelectOption {
                id: SelectOptionId::new(),
                value: format!("Option {i}"),
            })
            .collect();
        let config = PropertyConfig::Select { options };
        let name = PropertyName::try_from("Select Prop".to_owned()).expect("valid name");
        let result = Property::new(
            DatabaseId::new(),
            name,
            PropertyType::Select,
            Some(config),
            0,
        );
        assert!(result.is_ok(), "100 options should be accepted");

        // Try 101 — should fail
        let options_101: Vec<SelectOption> = (0..101)
            .map(|i| SelectOption {
                id: SelectOptionId::new(),
                value: format!("Option {i}"),
            })
            .collect();
        let config_101 = PropertyConfig::Select {
            options: options_101,
        };
        let name2 = PropertyName::try_from("Select Prop 2".to_owned()).expect("valid name");
        let result_101 = Property::new(
            DatabaseId::new(),
            name2,
            PropertyType::Select,
            Some(config_101),
            0,
        );
        assert!(
            result_101.is_err(),
            "101 options should be rejected by domain validation"
        );
    }

    // T043: delete_by_page_and_database
    #[tokio::test]
    async fn delete_by_page_and_database_removes_values() {
        let pool = setup_pool().await;
        let database = create_test_database(&pool).await;
        let prop = create_test_property(&pool, database.id()).await;
        let page = create_test_page(&pool, database.id()).await;

        let pv = PropertyValue::new_validated(
            page.id().clone(),
            prop.id().clone(),
            PropertyType::Text,
            None,
            PropertyValueInput::Text("to remove".to_owned()),
        )
        .expect("valid");

        let repo = SqlxPropertyValueRepository::new(pool.clone());
        repo.upsert(&pv).await.expect("upsert");

        repo.delete_by_page_and_database(page.id(), database.id())
            .await
            .expect("delete");

        let found = repo
            .find_by_page_and_property(page.id(), prop.id())
            .await
            .expect("find");
        assert!(found.is_none());
    }
}
