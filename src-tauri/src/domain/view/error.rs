use crate::domain::property::entity::PropertyId;

use super::entity::ViewId;

/// Errors originating from the View domain model.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ViewError {
    /// No view was found for the given database.
    #[error("view not found: {id}")]
    ViewNotFound {
        /// The view ID that was looked up.
        id: ViewId,
    },

    /// A sort condition failed validation.
    #[error("invalid sort condition: {reason}")]
    InvalidSortCondition {
        /// Description of the validation failure.
        reason: String,
    },

    /// Too many sort conditions (max 5).
    #[error("too many sort conditions: {count} (max {max})")]
    TooManySortConditions {
        /// Actual condition count.
        count: usize,
        /// Maximum allowed count.
        max: usize,
    },

    /// A filter operator is incompatible with the property type.
    #[error("invalid filter operator '{operator}' for property type '{property_type}'")]
    InvalidFilterOperator {
        /// The operator that was rejected.
        operator: String,
        /// The property type that was checked.
        property_type: String,
    },

    /// A filter value type does not match the expected type.
    #[error("invalid filter value: {reason}")]
    InvalidFilterValue {
        /// Description of the value type error.
        reason: String,
    },

    /// Too many filter conditions (max 20).
    #[error("too many filter conditions: {count} (max {max})")]
    TooManyFilterConditions {
        /// Actual condition count.
        count: usize,
        /// Maximum allowed count.
        max: usize,
    },

    /// A referenced property does not exist.
    #[error("property not found: {id}")]
    PropertyNotFound {
        /// The property ID that was not found.
        id: PropertyId,
    },

    /// A group collapse/expand was attempted without an active group condition.
    #[error("no group condition is set for the view")]
    NoGroupCondition,

    /// Duplicate property ID in sort conditions.
    #[error("duplicate sort property: {id}")]
    DuplicateSortProperty {
        /// The property ID that appeared more than once.
        id: PropertyId,
    },
}
