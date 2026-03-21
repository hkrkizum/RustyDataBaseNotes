// The `#[tauri::command]` macro generates `unreachable!()` in its expansion.
#![allow(clippy::unreachable)]

use tauri::State;

use crate::AppState;
use crate::domain::database::entity::DatabaseId;
use crate::domain::database::repository::DatabaseRepository;
use crate::domain::property::entity::{
    MAX_PROPERTIES, Property, PropertyConfig, PropertyName, PropertyType,
};
use crate::domain::property::error::PropertyError;
use crate::domain::property::repository::PropertyRepository;
use crate::infrastructure::persistence::database_repository::SqlxDatabaseRepository;
use crate::infrastructure::persistence::property_repository::SqlxPropertyRepository;
use crate::ipc::dto::PropertyDto;
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
