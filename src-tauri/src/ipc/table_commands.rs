// The `#[tauri::command]` macro generates `unreachable!()` in its expansion.
#![allow(clippy::unreachable)]

use std::collections::HashMap;

use tauri::State;

use crate::AppState;
use crate::domain::database::entity::DatabaseId;
use crate::domain::database::repository::DatabaseRepository;
use crate::domain::page::entity::{Page, PageId, PageTitle};
use crate::domain::page::error::PageError;
use crate::domain::page::repository::PageRepository;
use crate::domain::property::repository::{PropertyRepository, PropertyValueRepository};
use crate::domain::view::entity::View;
use crate::domain::view::repository::ViewRepository;
use crate::domain::view::sort::{RowPropertyValue, SortPropertyInfo};
use crate::infrastructure::persistence::database_repository::SqlxDatabaseRepository;
use crate::infrastructure::persistence::page_repository::SqlxPageRepository;
use crate::infrastructure::persistence::property_repository::SqlxPropertyRepository;
use crate::infrastructure::persistence::property_value_repository::SqlxPropertyValueRepository;
use crate::infrastructure::persistence::view_repository::SqlxViewRepository;
use crate::ipc::dto::{
    DatabaseDto, PageDto, PropertyDto, PropertyValueDto, TableDataDto, TableRowDto, ViewDto,
};
use crate::ipc::error::CommandError;

/// Creates a new page and adds it to a database.
#[tauri::command]
pub async fn add_page_to_database(
    state: State<'_, AppState>,
    database_id: String,
    title: String,
) -> Result<PageDto, CommandError> {
    let db_id: DatabaseId = database_id.parse().map_err(|_| {
        crate::domain::database::error::DatabaseError::NotFound {
            id: DatabaseId::new(),
        }
    })?;

    // Verify database exists
    let db_repo = SqlxDatabaseRepository::new(state.db.clone());
    db_repo.find_by_id(&db_id).await?;

    // Create a new page
    let page_title = PageTitle::try_from(title)?;
    let page = Page::new(page_title);
    let page_id = page.id().clone();

    let page_repo = SqlxPageRepository::new(state.db.clone());
    page_repo.create(&page).await?;

    // Set database_id
    page_repo.set_database_id(&page_id, Some(&db_id)).await?;

    // Refetch to get updated data with database_id
    let updated = page_repo.find_by_id(&page_id).await?;
    Ok(PageDto::from(updated))
}

/// Adds an existing standalone page to a database.
#[tauri::command]
pub async fn add_existing_page_to_database(
    state: State<'_, AppState>,
    database_id: String,
    page_id: String,
) -> Result<PageDto, CommandError> {
    let db_id: DatabaseId = database_id.parse().map_err(|_| {
        crate::domain::database::error::DatabaseError::NotFound {
            id: DatabaseId::new(),
        }
    })?;
    let pg_id: PageId = page_id
        .parse()
        .map_err(|_| PageError::NotFound { id: PageId::new() })?;

    // Verify database exists
    let db_repo = SqlxDatabaseRepository::new(state.db.clone());
    db_repo.find_by_id(&db_id).await?;

    // Find page and check it's not already in any database
    let page_repo = SqlxPageRepository::new(state.db.clone());
    let page = page_repo.find_by_id(&pg_id).await?;

    if let Some(existing_db_id) = page.database_id() {
        return Err(PageError::AlreadyInDatabase {
            page_id: pg_id,
            database_id: existing_db_id.clone(),
        }
        .into());
    }

    // Set database_id
    page_repo.set_database_id(&pg_id, Some(&db_id)).await?;

    // Return updated page
    let updated = page_repo.find_by_id(&pg_id).await?;
    Ok(PageDto::from(updated))
}

/// Returns all pages not belonging to any database.
#[tauri::command]
pub async fn list_standalone_pages(
    state: State<'_, AppState>,
) -> Result<Vec<PageDto>, CommandError> {
    let repo = SqlxPageRepository::new(state.db.clone());
    let pages = repo.find_standalone_pages().await?;
    Ok(pages.into_iter().map(PageDto::from).collect())
}

/// Removes a page from its database, deleting associated property values and
/// setting its `database_id` to NULL. No-op if the page is already standalone.
#[tauri::command]
pub async fn remove_page_from_database(
    state: State<'_, AppState>,
    page_id: String,
) -> Result<(), CommandError> {
    let pid: PageId = page_id
        .parse()
        .map_err(|_| PageError::NotFound { id: PageId::new() })?;

    let page_repo = SqlxPageRepository::new(state.db.clone());
    let page = page_repo.find_by_id(&pid).await?;

    // If already standalone, no-op
    if let Some(db_id) = page.database_id() {
        // Delete property values for this page in this database
        let pv_repo = SqlxPropertyValueRepository::new(state.db.clone());
        pv_repo.delete_by_page_and_database(&pid, db_id).await?;
        // Set database_id to NULL
        page_repo.set_database_id(&pid, None).await?;
    }

    Ok(())
}

/// Returns the full table view data for a database (pages, properties, values).
#[tauri::command]
pub async fn get_table_data(
    state: State<'_, AppState>,
    database_id: String,
) -> Result<TableDataDto, CommandError> {
    // 1. Parse database_id, find database
    let db_id: DatabaseId = database_id.parse().map_err(|_| {
        crate::domain::database::error::DatabaseError::NotFound {
            id: DatabaseId::new(),
        }
    })?;

    let db_repo = SqlxDatabaseRepository::new(state.db.clone());
    let database = db_repo.find_by_id(&db_id).await?;

    // 2. Get properties for database
    let prop_repo = SqlxPropertyRepository::new(state.db.clone());
    let properties = prop_repo.find_by_database_id(&db_id).await?;

    // 3. Get pages with this database_id
    let page_repo = SqlxPageRepository::new(state.db.clone());
    let pages = page_repo.find_by_database_id(&db_id).await?;

    // 4. Get all property values for database
    let pv_repo = SqlxPropertyValueRepository::new(state.db.clone());
    let all_values = pv_repo.find_all_for_database(&db_id).await?;

    // 5. Build a lookup: (page_id_str, property_id_str) -> PropertyValueDto
    let mut values_map: HashMap<String, HashMap<String, PropertyValueDto>> = HashMap::new();
    for pv in all_values {
        let page_key = pv.page_id().to_string();
        let prop_key = pv.property_id().to_string();
        values_map
            .entry(page_key)
            .or_default()
            .insert(prop_key, PropertyValueDto::from(pv));
    }

    // 6. Assemble rows
    let mut rows: Vec<TableRowDto> = pages
        .into_iter()
        .map(|page| {
            let page_id_str = page.id().to_string();
            let page_values = values_map.remove(&page_id_str).unwrap_or_default();
            TableRowDto {
                page: PageDto::from(page),
                values: page_values,
            }
        })
        .collect();

    // Load or create default view
    let view_repo = SqlxViewRepository::new(state.db.clone());
    let view = match view_repo.find_by_database_id(&db_id).await? {
        Some(v) => v,
        None => {
            let v = View::new_default(db_id.clone());
            view_repo.save(&v).await?;
            v
        }
    };

    // 7. Build property info map and row value data for filter/sort/group
    let property_info: HashMap<String, SortPropertyInfo> = properties
        .iter()
        .map(|p| {
            (
                p.id().to_string(),
                SortPropertyInfo {
                    property_type: p.property_type(),
                    config: p.config().cloned(),
                },
            )
        })
        .collect();

    let property_types: HashMap<String, crate::domain::property::entity::PropertyType> = properties
        .iter()
        .map(|p| (p.id().to_string(), p.property_type()))
        .collect();

    let row_values: Vec<HashMap<String, RowPropertyValue>> = rows
        .iter()
        .map(|row| {
            row.values
                .iter()
                .map(|(prop_id, pv_dto)| {
                    (
                        prop_id.clone(),
                        RowPropertyValue {
                            text_value: pv_dto.text_value.clone(),
                            number_value: pv_dto.number_value,
                            date_value: pv_dto.date_value.as_ref().and_then(|d| d.parse().ok()),
                            boolean_value: pv_dto.boolean_value,
                        },
                    )
                })
                .collect()
        })
        .collect();

    // 8. Apply filter → sort pipeline
    // Filter first
    if !view.filter_conditions().is_empty() {
        let matching = crate::domain::view::filter::apply_filters(
            &row_values,
            view.filter_conditions(),
            &property_types,
        );
        let original = rows;
        rows = matching.into_iter().map(|i| original[i].clone()).collect();
    }

    // Sort (on filtered rows)
    if !view.sort_conditions().is_empty() {
        // Rebuild row values for filtered rows
        let filtered_row_values: Vec<HashMap<String, RowPropertyValue>> = rows
            .iter()
            .map(|row| {
                row.values
                    .iter()
                    .map(|(prop_id, pv_dto)| {
                        (
                            prop_id.clone(),
                            RowPropertyValue {
                                text_value: pv_dto.text_value.clone(),
                                number_value: pv_dto.number_value,
                                date_value: pv_dto.date_value.as_ref().and_then(|d| d.parse().ok()),
                                boolean_value: pv_dto.boolean_value,
                            },
                        )
                    })
                    .collect()
            })
            .collect();

        let sorted_indices = crate::domain::view::sort::compute_sort_order(
            &filtered_row_values,
            view.sort_conditions(),
            &property_info,
        );

        let original = rows;
        rows = sorted_indices
            .into_iter()
            .map(|i| original[i].clone())
            .collect();
    }

    // 9. Apply grouping if condition exists
    let groups = if let Some(gc) = view.group_condition() {
        let gc_prop_id_str = gc.property_id.to_string();
        let gc_prop_type = property_types
            .get(&gc_prop_id_str)
            .copied()
            .unwrap_or(crate::domain::property::entity::PropertyType::Text);
        let gc_config = properties
            .iter()
            .find(|p| p.id().to_string() == gc_prop_id_str)
            .and_then(|p| p.config());

        // Rebuild row values for grouped rows
        let grouped_row_values: Vec<HashMap<String, RowPropertyValue>> = rows
            .iter()
            .map(|row| {
                row.values
                    .iter()
                    .map(|(prop_id, pv_dto)| {
                        (
                            prop_id.clone(),
                            RowPropertyValue {
                                text_value: pv_dto.text_value.clone(),
                                number_value: pv_dto.number_value,
                                date_value: pv_dto.date_value.as_ref().and_then(|d| d.parse().ok()),
                                boolean_value: pv_dto.boolean_value,
                            },
                        )
                    })
                    .collect()
            })
            .collect();

        let (group_infos, grouped_indices) = crate::domain::view::group::compute_groups(
            &grouped_row_values,
            gc,
            gc_prop_type,
            gc_config,
            view.collapsed_groups(),
        );

        // Reorder rows by group order, excluding collapsed groups
        let original = rows;
        rows = Vec::new();
        for (gi, indices) in group_infos.iter().zip(grouped_indices.iter()) {
            if !gi.is_collapsed {
                for &idx in indices {
                    rows.push(original[idx].clone());
                }
            }
        }

        Some(
            group_infos
                .into_iter()
                .map(|gi| crate::ipc::dto::GroupInfoDto {
                    value: gi.value,
                    display_value: gi.display_value,
                    count: gi.count,
                    is_collapsed: gi.is_collapsed,
                })
                .collect(),
        )
    } else {
        None
    };

    let property_dtos: Vec<PropertyDto> = properties.into_iter().map(PropertyDto::from).collect();

    Ok(TableDataDto {
        database: DatabaseDto::from(database),
        properties: property_dtos,
        rows,
        view: ViewDto::from(&view),
        groups,
    })
}
