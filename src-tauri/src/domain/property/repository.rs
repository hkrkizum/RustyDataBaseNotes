use super::entity::{Property, PropertyConfig, PropertyId, PropertyName, PropertyValue};
use super::error::{PropertyError, PropertyValueError};
use crate::domain::database::entity::DatabaseId;
use crate::domain::page::entity::PageId;

/// Trait defining persistence operations for [`Property`] entities.
#[allow(async_fn_in_trait)]
pub trait PropertyRepository {
    /// The error type returned by this repository, which must be convertible
    /// from [`PropertyError`].
    type Error: From<PropertyError>;

    /// Persists a new property.
    async fn create(&self, property: &Property) -> Result<(), Self::Error>;

    /// Returns all properties belonging to the given database, ordered by position.
    async fn find_by_database_id(
        &self,
        database_id: &DatabaseId,
    ) -> Result<Vec<Property>, Self::Error>;

    /// Returns the property with the given ID.
    async fn find_by_id(&self, id: &PropertyId) -> Result<Property, Self::Error>;

    /// Updates the name of the property identified by `id` and returns the
    /// updated entity.
    async fn update_name(
        &self,
        id: &PropertyId,
        name: &PropertyName,
    ) -> Result<Property, Self::Error>;

    /// Updates the configuration of the property identified by `id` and
    /// returns the updated entity.
    async fn update_config(
        &self,
        id: &PropertyId,
        config: &PropertyConfig,
    ) -> Result<Property, Self::Error>;

    /// Batch-updates the positions of multiple properties in a single
    /// transaction.
    async fn update_positions(&self, updates: &[(PropertyId, i64)]) -> Result<(), Self::Error>;

    /// Deletes the property with the given ID.
    async fn delete(&self, id: &PropertyId) -> Result<(), Self::Error>;

    /// Returns the number of properties in the given database.
    async fn count_by_database_id(&self, database_id: &DatabaseId) -> Result<usize, Self::Error>;

    /// Returns the next available position value for a new property in the
    /// given database.
    async fn next_position(&self, database_id: &DatabaseId) -> Result<i64, Self::Error>;
}

/// Trait defining persistence operations for [`PropertyValue`] entities.
#[allow(async_fn_in_trait)]
pub trait PropertyValueRepository {
    /// The error type returned by this repository, which must be convertible
    /// from [`PropertyValueError`].
    type Error: From<PropertyValueError>;

    /// Inserts or updates a property value.
    async fn upsert(&self, value: &PropertyValue) -> Result<(), Self::Error>;

    /// Returns the property value for the given page and property combination,
    /// or `None` if no value has been set.
    async fn find_by_page_and_property(
        &self,
        page_id: &PageId,
        property_id: &PropertyId,
    ) -> Result<Option<PropertyValue>, Self::Error>;

    /// Returns all property values attached to the given page.
    async fn find_by_page_id(&self, page_id: &PageId) -> Result<Vec<PropertyValue>, Self::Error>;

    /// Returns all property values for the given property across all pages.
    async fn find_by_property_id(
        &self,
        property_id: &PropertyId,
    ) -> Result<Vec<PropertyValue>, Self::Error>;

    /// Deletes the property value for the given page and property combination.
    async fn delete_by_page_and_property(
        &self,
        page_id: &PageId,
        property_id: &PropertyId,
    ) -> Result<(), Self::Error>;

    /// Deletes all property values for the given page within the specified
    /// database.
    async fn delete_by_page_and_database(
        &self,
        page_id: &PageId,
        database_id: &DatabaseId,
    ) -> Result<(), Self::Error>;

    /// Clears any property values that reference the given select option,
    /// resetting them to an empty state.
    async fn reset_select_option(
        &self,
        property_id: &PropertyId,
        option_id: &str,
    ) -> Result<(), Self::Error>;

    /// Returns all property values for pages belonging to the given database.
    async fn find_all_for_database(
        &self,
        database_id: &DatabaseId,
    ) -> Result<Vec<PropertyValue>, Self::Error>;
}
