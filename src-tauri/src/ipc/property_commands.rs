// The `#[tauri::command]` macro generates `unreachable!()` in its expansion.
#![allow(clippy::unreachable)]

use chrono::{DateTime, Utc};
use tauri::State;

use crate::AppState;
use crate::domain::database::entity::DatabaseId;
use crate::domain::database::repository::DatabaseRepository;
use crate::domain::page::entity::PageId;
use crate::domain::page::repository::PageRepository;
use crate::domain::property::entity::{
    MAX_PROPERTIES, Property, PropertyConfig, PropertyName, PropertyType, PropertyValue,
    PropertyValueInput,
};
use crate::domain::property::error::{PropertyError, PropertyValueError};
use crate::domain::property::repository::{PropertyRepository, PropertyValueRepository};
use crate::infrastructure::persistence::database_repository::SqlxDatabaseRepository;
use crate::infrastructure::persistence::page_repository::SqlxPageRepository;
use crate::infrastructure::persistence::property_repository::SqlxPropertyRepository;
use crate::infrastructure::persistence::property_value_repository::SqlxPropertyValueRepository;
use crate::ipc::dto::{PropertyDto, PropertyValueDto};
use crate::ipc::error::CommandError;

/// Adds a new property (column) to a database.
#[tauri::command]
pub async fn add_property(
    state: State<'_, AppState>,
    database_id: String,
    name: String,
    property_type: PropertyType,
    config: Option<PropertyConfig>,
) -> Result<PropertyDto, CommandError> {
    // 1. Parse database_id
    let db_id: DatabaseId = database_id.parse().map_err(|_| {
        crate::domain::database::error::DatabaseError::NotFound {
            id: DatabaseId::new(),
        }
    })?;

    // 2. Verify database exists
    let db_repo = SqlxDatabaseRepository::new(state.db.clone());
    db_repo.find_by_id(&db_id).await?;

    // 3. Validate name
    let prop_name = PropertyName::try_from(name)?;

    // 4. Check property count limit
    let prop_repo = SqlxPropertyRepository::new(state.db.clone());
    let count = prop_repo.count_by_database_id(&db_id).await?;
    if count >= MAX_PROPERTIES {
        return Err(PropertyError::TooManyProperties {
            count,
            max: MAX_PROPERTIES,
        }
        .into());
    }

    // 5. Get next position
    let position = prop_repo.next_position(&db_id).await?;

    // 6. Create Property entity (validates config consistency)
    let property = Property::new(db_id, prop_name, property_type, config, position)?;

    // 7-8. Save via repository (UNIQUE constraint catches duplicate names)
    prop_repo.create(&property).await?;

    // 9. Return DTO
    Ok(PropertyDto::from(property))
}

/// Returns all properties for a database, ordered by position.
#[tauri::command]
pub async fn list_properties(
    state: State<'_, AppState>,
    database_id: String,
) -> Result<Vec<PropertyDto>, CommandError> {
    let db_id: DatabaseId = database_id.parse().map_err(|_| {
        crate::domain::database::error::DatabaseError::NotFound {
            id: DatabaseId::new(),
        }
    })?;

    // Verify database exists
    let db_repo = SqlxDatabaseRepository::new(state.db.clone());
    db_repo.find_by_id(&db_id).await?;

    let prop_repo = SqlxPropertyRepository::new(state.db.clone());
    let properties = prop_repo.find_by_database_id(&db_id).await?;

    Ok(properties.into_iter().map(PropertyDto::from).collect())
}

/// Parses a `serde_json::Value` into a [`PropertyValueInput`].
fn parse_value_input(
    value: &serde_json::Value,
    property_type: PropertyType,
) -> Result<PropertyValueInput, CommandError> {
    let type_str = value.get("type").and_then(|v| v.as_str()).ok_or_else(|| {
        PropertyValueError::TypeMismatch {
            expected: property_type,
            property_id: crate::domain::property::entity::PropertyId::new(),
        }
    })?;

    match type_str {
        "text" => {
            let s = value.get("value").and_then(|v| v.as_str()).ok_or_else(|| {
                PropertyValueError::TypeMismatch {
                    expected: PropertyType::Text,
                    property_id: crate::domain::property::entity::PropertyId::new(),
                }
            })?;
            Ok(PropertyValueInput::Text(s.to_owned()))
        }
        "number" => {
            let n = value.get("value").and_then(|v| v.as_f64()).ok_or_else(|| {
                PropertyValueError::InvalidNumber {
                    reason: "missing or invalid number value".to_owned(),
                }
            })?;
            Ok(PropertyValueInput::Number(n))
        }
        "date" => {
            let date_str = value.get("value").and_then(|v| v.as_str()).ok_or_else(|| {
                PropertyValueError::InvalidDate {
                    reason: "missing date value".to_owned(),
                }
            })?;
            let dt =
                date_str
                    .parse::<DateTime<Utc>>()
                    .map_err(|e| PropertyValueError::InvalidDate {
                        reason: e.to_string(),
                    })?;
            Ok(PropertyValueInput::Date(dt))
        }
        "select" => {
            let option_id = value
                .get("optionId")
                .and_then(|v| v.as_str())
                .ok_or_else(|| PropertyValueError::TypeMismatch {
                    expected: PropertyType::Select,
                    property_id: crate::domain::property::entity::PropertyId::new(),
                })?;
            Ok(PropertyValueInput::Select(option_id.to_owned()))
        }
        "checkbox" => {
            let b = value
                .get("value")
                .and_then(|v| v.as_bool())
                .ok_or_else(|| PropertyValueError::TypeMismatch {
                    expected: PropertyType::Checkbox,
                    property_id: crate::domain::property::entity::PropertyId::new(),
                })?;
            Ok(PropertyValueInput::Checkbox(b))
        }
        _ => Err(PropertyValueError::TypeMismatch {
            expected: property_type,
            property_id: crate::domain::property::entity::PropertyId::new(),
        }
        .into()),
    }
}

/// Sets (upserts) a property value for a page.
#[tauri::command]
pub async fn set_property_value(
    state: State<'_, AppState>,
    page_id: String,
    property_id: String,
    value: serde_json::Value,
) -> Result<PropertyValueDto, CommandError> {
    // 1. Parse IDs
    let pg_id: PageId = page_id
        .parse()
        .map_err(|_| crate::domain::page::error::PageError::NotFound { id: PageId::new() })?;
    let prop_id: crate::domain::property::entity::PropertyId =
        property_id.parse().map_err(|_| PropertyError::NotFound {
            id: crate::domain::property::entity::PropertyId::new(),
        })?;

    // 2. Find property (get type and config)
    let prop_repo = SqlxPropertyRepository::new(state.db.clone());
    let property = prop_repo.find_by_id(&prop_id).await?;

    // 3. Find page, verify page.database_id matches property.database_id
    let page_repo = SqlxPageRepository::new(state.db.clone());
    let page = page_repo.find_by_id(&pg_id).await?;

    match page.database_id() {
        Some(page_db_id) if page_db_id == property.database_id() => {}
        _ => {
            return Err(PropertyValueError::PageNotInDatabase {
                page_id: pg_id,
                database_id: property.database_id().clone(),
            }
            .into());
        }
    }

    // 4. Parse value input
    let input = parse_value_input(&value, property.property_type())?;

    // 5. Create PropertyValue with validation
    let pv = PropertyValue::new_validated(
        pg_id,
        prop_id,
        property.property_type(),
        property.config(),
        input,
    )?;

    // 6. Upsert via repository
    let pv_repo = SqlxPropertyValueRepository::new(state.db.clone());
    pv_repo.upsert(&pv).await?;

    // 7. Return DTO
    Ok(PropertyValueDto::from(pv))
}

/// Clears (deletes) a property value for a page.
#[tauri::command]
pub async fn clear_property_value(
    state: State<'_, AppState>,
    page_id: String,
    property_id: String,
) -> Result<(), CommandError> {
    let pg_id: PageId = page_id
        .parse()
        .map_err(|_| crate::domain::page::error::PageError::NotFound { id: PageId::new() })?;
    let prop_id: crate::domain::property::entity::PropertyId =
        property_id.parse().map_err(|_| PropertyError::NotFound {
            id: crate::domain::property::entity::PropertyId::new(),
        })?;

    let pv_repo = SqlxPropertyValueRepository::new(state.db.clone());
    pv_repo
        .delete_by_page_and_property(&pg_id, &prop_id)
        .await?;

    Ok(())
}
