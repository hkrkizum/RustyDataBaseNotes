use super::entity::{PropertyId, PropertyType, PropertyValueId};
use crate::domain::database::entity::DatabaseId;
use crate::domain::page::entity::PageId;

/// Errors originating from the Property domain model.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum PropertyError {
    /// The property name was empty after trimming whitespace.
    #[error("property name must not be empty")]
    NameEmpty,

    /// The property name exceeded the maximum allowed length.
    #[error("property name too long: {len} characters (max {max})")]
    NameTooLong {
        /// Actual character count.
        len: usize,
        /// Maximum allowed character count.
        max: usize,
    },

    /// A property with the same name already exists in the database.
    #[error("duplicate property name '{name}' in database {database_id}")]
    DuplicateName {
        /// The duplicate name.
        name: String,
        /// The database where the duplicate was found.
        database_id: DatabaseId,
    },

    /// The property type value is invalid.
    #[error("invalid property type: {value}")]
    InvalidType {
        /// The invalid value.
        value: String,
    },

    /// Too many properties in a database.
    #[error("too many properties: {count} (max {max})")]
    TooManyProperties {
        /// Actual property count.
        count: usize,
        /// Maximum allowed count.
        max: usize,
    },

    /// No property was found with the given ID.
    #[error("property not found: {id}")]
    NotFound {
        /// The ID that was looked up.
        id: PropertyId,
    },

    /// The property configuration is invalid.
    #[error("invalid property config: {reason}")]
    InvalidConfig {
        /// Description of the config error.
        reason: String,
    },

    /// Too many select options.
    #[error("too many select options: {count} (max {max})")]
    TooManyOptions {
        /// Actual option count.
        count: usize,
        /// Maximum allowed count.
        max: usize,
    },

    /// A select option value was empty.
    #[error("select option value must not be empty")]
    OptionValueEmpty,

    /// A select option value is duplicated.
    #[error("duplicate select option value: {value}")]
    DuplicateOptionValue {
        /// The duplicated value.
        value: String,
    },
}

/// Errors originating from the PropertyValue domain model.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum PropertyValueError {
    /// The number value is invalid (NaN or Infinity).
    #[error("invalid number: {reason}")]
    InvalidNumber {
        /// Description of the number error.
        reason: String,
    },

    /// The date value is invalid.
    #[error("invalid date: {reason}")]
    InvalidDate {
        /// Description of the date error.
        reason: String,
    },

    /// The select option ID does not exist in the property's configuration.
    #[error("invalid select option '{option_id}' for property {property_id}")]
    InvalidSelectOption {
        /// The option ID that was not found.
        option_id: String,
        /// The property that was checked.
        property_id: PropertyId,
    },

    /// The value type does not match the property type.
    #[error("type mismatch: expected {expected} for property {property_id}")]
    TypeMismatch {
        /// The property type that was expected.
        expected: PropertyType,
        /// The property that was checked.
        property_id: PropertyId,
    },

    /// The page is not in the database that owns this property.
    #[error("page {page_id} not in database {database_id}")]
    PageNotInDatabase {
        /// The page ID.
        page_id: PageId,
        /// The database ID.
        database_id: DatabaseId,
    },

    /// No property value was found with the given ID.
    #[error("property value not found: {id}")]
    NotFound {
        /// The ID that was looked up.
        id: PropertyValueId,
    },
}
