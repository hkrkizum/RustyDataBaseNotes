// The `#[tauri::command]` macro generates `unreachable!()` in its expansion.
#![allow(clippy::unreachable)]

use std::collections::HashSet;

use serde::Deserialize;
use tauri::State;

use crate::AppState;
use crate::domain::database::entity::DatabaseId;
use crate::domain::property::entity::PropertyId;
use crate::domain::property::repository::PropertyRepository;
use crate::domain::view::entity::{
    FilterCondition, FilterOperator, FilterValue, GroupCondition, SortCondition, SortDirection,
    View,
};
use crate::domain::view::error::ViewError;
use crate::domain::view::repository::ViewRepository;
use crate::infrastructure::persistence::property_repository::SqlxPropertyRepository;
use crate::infrastructure::persistence::view_repository::SqlxViewRepository;
use crate::ipc::dto::ViewDto;
use crate::ipc::error::CommandError;

/// Helper: find or create a default view for the given database.
async fn find_or_create_view(
    pool: &sqlx::SqlitePool,
    database_id: &DatabaseId,
) -> Result<View, CommandError> {
    let repo = SqlxViewRepository::new(pool.clone());
    match repo.find_by_database_id(database_id).await? {
        Some(view) => Ok(view),
        None => {
            let view = View::new_default(database_id.clone());
            repo.save(&view).await?;
            Ok(view)
        }
    }
}

/// Helper: parse a database ID string.
fn parse_database_id(database_id: &str) -> Result<DatabaseId, CommandError> {
    database_id.parse().map_err(|_| {
        crate::domain::database::error::DatabaseError::NotFound {
            id: DatabaseId::new(),
        }
        .into()
    })
}

/// Helper: verify all property IDs exist in the given database.
async fn verify_properties_exist(
    pool: &sqlx::SqlitePool,
    database_id: &DatabaseId,
    property_ids: &[PropertyId],
) -> Result<(), CommandError> {
    let prop_repo = SqlxPropertyRepository::new(pool.clone());
    let properties = prop_repo.find_by_database_id(database_id).await?;
    let known_ids: HashSet<String> = properties.iter().map(|p| p.id().to_string()).collect();

    for pid in property_ids {
        if !known_ids.contains(&pid.to_string()) {
            return Err(ViewError::PropertyNotFound { id: pid.clone() }.into());
        }
    }
    Ok(())
}

/// Returns the view settings for a database.
#[tauri::command]
pub async fn get_view(
    state: State<'_, AppState>,
    database_id: String,
) -> Result<ViewDto, CommandError> {
    let db_id = parse_database_id(&database_id)?;
    let view = find_or_create_view(&state.db, &db_id).await?;
    Ok(ViewDto::from(&view))
}

/// Resets the view settings for a database to defaults.
#[tauri::command]
pub async fn reset_view(
    state: State<'_, AppState>,
    database_id: String,
) -> Result<ViewDto, CommandError> {
    let db_id = parse_database_id(&database_id)?;

    // Ensure view exists
    find_or_create_view(&state.db, &db_id).await?;

    let repo = SqlxViewRepository::new(state.db.clone());
    let view = repo.reset(&db_id).await?;
    Ok(ViewDto::from(&view))
}

/// Input DTO for sort condition from frontend.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortConditionInput {
    /// Property ID string.
    pub property_id: String,
    /// Sort direction.
    pub direction: SortDirection,
}

/// Updates sort conditions for a database's view.
#[tauri::command]
pub async fn update_sort_conditions(
    state: State<'_, AppState>,
    database_id: String,
    conditions: Vec<SortConditionInput>,
) -> Result<ViewDto, CommandError> {
    let db_id = parse_database_id(&database_id)?;

    // Parse property IDs
    let parsed: Vec<SortCondition> = conditions
        .into_iter()
        .map(|c| {
            let property_id: PropertyId =
                c.property_id
                    .parse()
                    .map_err(|_| ViewError::PropertyNotFound {
                        id: PropertyId::new(),
                    })?;
            Ok(SortCondition {
                property_id,
                direction: c.direction,
            })
        })
        .collect::<Result<Vec<_>, ViewError>>()?;

    // Validate via entity (max 5, no duplicates)
    let mut view = find_or_create_view(&state.db, &db_id).await?;
    view.set_sort_conditions(parsed.clone())?;

    // Verify all properties exist in the database
    let prop_ids: Vec<PropertyId> = parsed.iter().map(|c| c.property_id.clone()).collect();
    verify_properties_exist(&state.db, &db_id, &prop_ids).await?;

    // Persist
    let repo = SqlxViewRepository::new(state.db.clone());
    let updated = repo.update_sort_conditions(&db_id, &parsed).await?;
    Ok(ViewDto::from(&updated))
}

/// Input DTO for filter value from frontend.
#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum FilterValueInput {
    /// Text comparison value.
    Text(String),
    /// Numeric comparison value.
    Number(f64),
    /// Date comparison value (ISO 8601).
    Date(String),
    /// Select option comparison value.
    SelectOption(String),
}

impl From<FilterValueInput> for FilterValue {
    fn from(input: FilterValueInput) -> Self {
        match input {
            FilterValueInput::Text(s) => FilterValue::Text(s),
            FilterValueInput::Number(n) => FilterValue::Number(n),
            FilterValueInput::Date(d) => FilterValue::Date(d),
            FilterValueInput::SelectOption(s) => FilterValue::SelectOption(s),
        }
    }
}

/// Input DTO for filter condition from frontend.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterConditionInput {
    /// Property ID string.
    pub property_id: String,
    /// Filter operator.
    pub operator: FilterOperator,
    /// Comparison value (null for IsEmpty/IsNotEmpty/IsChecked/IsUnchecked).
    pub value: Option<FilterValueInput>,
}

/// Updates filter conditions for a database's view.
#[tauri::command]
pub async fn update_filter_conditions(
    state: State<'_, AppState>,
    database_id: String,
    conditions: Vec<FilterConditionInput>,
) -> Result<ViewDto, CommandError> {
    let db_id = parse_database_id(&database_id)?;

    // Parse property IDs and build FilterConditions
    let parsed: Vec<FilterCondition> = conditions
        .into_iter()
        .map(|c| {
            let property_id: PropertyId =
                c.property_id
                    .parse()
                    .map_err(|_| ViewError::PropertyNotFound {
                        id: PropertyId::new(),
                    })?;
            Ok(FilterCondition {
                property_id,
                operator: c.operator,
                value: c.value.map(FilterValue::from),
            })
        })
        .collect::<Result<Vec<_>, ViewError>>()?;

    // Validate via entity (max 20)
    let mut view = find_or_create_view(&state.db, &db_id).await?;
    view.set_filter_conditions(parsed.clone())?;

    // Verify all properties exist and validate operator-type compatibility
    let prop_repo = SqlxPropertyRepository::new(state.db.clone());
    let properties = prop_repo.find_by_database_id(&db_id).await?;
    let prop_map: std::collections::HashMap<String, crate::domain::property::entity::PropertyType> =
        properties
            .iter()
            .map(|p| (p.id().to_string(), p.property_type()))
            .collect();

    for cond in &parsed {
        let pid_str = cond.property_id.to_string();
        let prop_type = prop_map
            .get(&pid_str)
            .ok_or_else(|| ViewError::PropertyNotFound {
                id: cond.property_id.clone(),
            })?;

        validate_filter_operator(*prop_type, cond.operator)?;
        validate_filter_value(*prop_type, cond.operator, &cond.value)?;
    }

    // Persist
    let repo = SqlxViewRepository::new(state.db.clone());
    let updated = repo.update_filter_conditions(&db_id, &parsed).await?;
    Ok(ViewDto::from(&updated))
}

/// Validates that a filter operator is compatible with a property type.
fn validate_filter_operator(
    prop_type: crate::domain::property::entity::PropertyType,
    operator: FilterOperator,
) -> Result<(), ViewError> {
    use crate::domain::property::entity::PropertyType;

    let valid = match prop_type {
        PropertyType::Text => matches!(
            operator,
            FilterOperator::Equals
                | FilterOperator::NotEquals
                | FilterOperator::Contains
                | FilterOperator::NotContains
                | FilterOperator::IsEmpty
                | FilterOperator::IsNotEmpty
        ),
        PropertyType::Number => matches!(
            operator,
            FilterOperator::Equals
                | FilterOperator::NotEquals
                | FilterOperator::GreaterThan
                | FilterOperator::LessThan
                | FilterOperator::GreaterOrEqual
                | FilterOperator::LessOrEqual
                | FilterOperator::IsEmpty
                | FilterOperator::IsNotEmpty
        ),
        PropertyType::Date => matches!(
            operator,
            FilterOperator::Equals
                | FilterOperator::Before
                | FilterOperator::After
                | FilterOperator::IsEmpty
                | FilterOperator::IsNotEmpty
        ),
        PropertyType::Select => matches!(
            operator,
            FilterOperator::Is
                | FilterOperator::IsNot
                | FilterOperator::IsEmpty
                | FilterOperator::IsNotEmpty
        ),
        PropertyType::Checkbox => {
            matches!(
                operator,
                FilterOperator::IsChecked | FilterOperator::IsUnchecked
            )
        }
    };

    if !valid {
        return Err(ViewError::InvalidFilterOperator {
            operator: operator.to_string(),
            property_type: prop_type.to_string(),
        });
    }
    Ok(())
}

/// Validates that a filter value matches the expected type for the operator.
fn validate_filter_value(
    prop_type: crate::domain::property::entity::PropertyType,
    operator: FilterOperator,
    value: &Option<FilterValue>,
) -> Result<(), ViewError> {
    use crate::domain::property::entity::PropertyType;

    // Operators that require no value
    if matches!(
        operator,
        FilterOperator::IsEmpty
            | FilterOperator::IsNotEmpty
            | FilterOperator::IsChecked
            | FilterOperator::IsUnchecked
    ) {
        if value.is_some() {
            return Err(ViewError::InvalidFilterValue {
                reason: format!("operator '{operator}' must not have a value"),
            });
        }
        return Ok(());
    }

    // All other operators require a value
    let val = value
        .as_ref()
        .ok_or_else(|| ViewError::InvalidFilterValue {
            reason: format!("operator '{operator}' requires a value"),
        })?;

    let type_matches = matches!(
        (prop_type, val),
        (PropertyType::Text, FilterValue::Text(_))
            | (PropertyType::Number, FilterValue::Number(_))
            | (PropertyType::Date, FilterValue::Date(_))
            | (PropertyType::Select, FilterValue::SelectOption(_))
    );

    if !type_matches {
        return Err(ViewError::InvalidFilterValue {
            reason: format!("value type does not match property type '{prop_type}'"),
        });
    }

    Ok(())
}

/// Input DTO for group condition from frontend.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupConditionInput {
    /// Property ID string.
    pub property_id: String,
}

/// Updates group condition for a database's view.
#[tauri::command]
pub async fn update_group_condition(
    state: State<'_, AppState>,
    database_id: String,
    condition: Option<GroupConditionInput>,
) -> Result<ViewDto, CommandError> {
    let db_id = parse_database_id(&database_id)?;

    let mut view = find_or_create_view(&state.db, &db_id).await?;

    let parsed = match condition {
        Some(c) => {
            let property_id: PropertyId =
                c.property_id
                    .parse()
                    .map_err(|_| ViewError::PropertyNotFound {
                        id: PropertyId::new(),
                    })?;
            // Verify property exists
            verify_properties_exist(&state.db, &db_id, std::slice::from_ref(&property_id)).await?;
            Some(GroupCondition { property_id })
        }
        None => None,
    };

    view.set_group_condition(parsed.clone());

    // Persist
    let repo = SqlxViewRepository::new(state.db.clone());
    let collapsed: Vec<String> = view.collapsed_groups().iter().cloned().collect();
    let updated = repo
        .update_group_condition(&db_id, parsed.as_ref(), &collapsed)
        .await?;
    Ok(ViewDto::from(&updated))
}

/// Toggles the collapsed state of a group.
#[tauri::command]
pub async fn toggle_group_collapsed(
    state: State<'_, AppState>,
    database_id: String,
    group_value: Option<String>,
) -> Result<ViewDto, CommandError> {
    let db_id = parse_database_id(&database_id)?;

    let mut view = find_or_create_view(&state.db, &db_id).await?;
    view.toggle_collapsed_group(group_value)?;

    // Persist
    let repo = SqlxViewRepository::new(state.db.clone());
    let collapsed: Vec<String> = view.collapsed_groups().iter().cloned().collect();
    let updated = repo.update_collapsed_groups(&db_id, &collapsed).await?;
    Ok(ViewDto::from(&updated))
}
