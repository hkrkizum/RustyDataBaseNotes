use serde::Serialize;

use crate::domain::block::error::BlockError;
use crate::domain::database::error::DatabaseError;
use crate::domain::page::error::PageError;
use crate::domain::property::error::{PropertyError, PropertyValueError};
use crate::domain::view::error::ViewError;
use crate::infrastructure::persistence::error::StorageError;

/// Unified error type for IPC command handlers.
///
/// Wraps domain and storage errors and serializes them as
/// `{ "kind": "<errorKind>", "message": "<msg>" }` for the frontend.
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    /// A domain-level page error.
    #[error(transparent)]
    Page(#[from] PageError),

    /// A domain-level block error.
    #[error(transparent)]
    Block(#[from] BlockError),

    /// A domain-level database error.
    #[error(transparent)]
    Database(#[from] DatabaseError),

    /// A domain-level property error.
    #[error(transparent)]
    Property(#[from] PropertyError),

    /// A domain-level property value error.
    #[error(transparent)]
    PropertyValue(#[from] PropertyValueError),

    /// A domain-level view error.
    #[error(transparent)]
    View(#[from] ViewError),

    /// A storage-level error.
    #[error(transparent)]
    Storage(#[from] StorageError),
}

impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let (kind, message) = match self {
            CommandError::Page(PageError::TitleEmpty) => ("titleEmpty", self.to_string()),
            CommandError::Page(PageError::TitleTooLong { .. }) => {
                ("titleTooLong", self.to_string())
            }
            CommandError::Page(PageError::NotFound { .. }) => ("notFound", self.to_string()),
            CommandError::Page(PageError::AlreadyInDatabase { .. }) => {
                ("pageAlreadyInDatabase", self.to_string())
            }
            CommandError::Block(BlockError::ContentTooLong { .. }) => {
                ("contentTooLong", self.to_string())
            }
            CommandError::Block(BlockError::InvalidPosition { .. }) => {
                ("invalidPosition", self.to_string())
            }
            CommandError::Block(BlockError::NotFound { .. }) => ("blockNotFound", self.to_string()),
            CommandError::Block(BlockError::CannotMoveUp { .. }) => {
                ("cannotMoveUp", self.to_string())
            }
            CommandError::Block(BlockError::CannotMoveDown { .. }) => {
                ("cannotMoveDown", self.to_string())
            }
            // Database errors
            CommandError::Database(DatabaseError::TitleEmpty) => ("titleEmpty", self.to_string()),
            CommandError::Database(DatabaseError::TitleTooLong { .. }) => {
                ("titleTooLong", self.to_string())
            }
            CommandError::Database(DatabaseError::NotFound { .. }) => {
                ("databaseNotFound", self.to_string())
            }

            // Property errors
            CommandError::Property(PropertyError::NameEmpty) => {
                ("propertyNameEmpty", self.to_string())
            }
            CommandError::Property(PropertyError::NameTooLong { .. }) => {
                ("propertyNameTooLong", self.to_string())
            }
            CommandError::Property(PropertyError::DuplicateName { .. }) => {
                ("duplicatePropertyName", self.to_string())
            }
            CommandError::Property(PropertyError::InvalidType { .. }) => {
                ("invalidConfig", self.to_string())
            }
            CommandError::Property(PropertyError::TooManyProperties { .. }) => {
                ("tooManyProperties", self.to_string())
            }
            CommandError::Property(PropertyError::NotFound { .. }) => {
                ("propertyNotFound", self.to_string())
            }
            CommandError::Property(PropertyError::InvalidConfig { .. }) => {
                ("invalidConfig", self.to_string())
            }
            CommandError::Property(PropertyError::TooManyOptions { .. }) => {
                ("tooManyOptions", self.to_string())
            }
            CommandError::Property(PropertyError::OptionValueEmpty) => {
                ("optionValueEmpty", self.to_string())
            }
            CommandError::Property(PropertyError::DuplicateOptionValue { .. }) => {
                ("duplicateOptionValue", self.to_string())
            }

            // PropertyValue errors
            CommandError::PropertyValue(PropertyValueError::InvalidNumber { .. }) => {
                ("invalidNumber", self.to_string())
            }
            CommandError::PropertyValue(PropertyValueError::InvalidDate { .. }) => {
                ("invalidDate", self.to_string())
            }
            CommandError::PropertyValue(PropertyValueError::InvalidSelectOption { .. }) => {
                ("invalidSelectOption", self.to_string())
            }
            CommandError::PropertyValue(PropertyValueError::TypeMismatch { .. }) => {
                ("typeMismatch", self.to_string())
            }
            CommandError::PropertyValue(PropertyValueError::PageNotInDatabase { .. }) => {
                ("pageNotInDatabase", self.to_string())
            }
            CommandError::PropertyValue(PropertyValueError::NotFound { .. }) => {
                ("propertyValueNotFound", self.to_string())
            }

            // View errors
            CommandError::View(ViewError::ViewNotFound { .. }) => {
                ("viewNotFound", self.to_string())
            }
            CommandError::View(ViewError::InvalidSortCondition { .. }) => {
                ("invalidSortCondition", self.to_string())
            }
            CommandError::View(ViewError::TooManySortConditions { .. }) => {
                ("tooManySortConditions", self.to_string())
            }
            CommandError::View(ViewError::InvalidFilterOperator { .. }) => {
                ("invalidFilterOperator", self.to_string())
            }
            CommandError::View(ViewError::InvalidFilterValue { .. }) => {
                ("invalidFilterValue", self.to_string())
            }
            CommandError::View(ViewError::TooManyFilterConditions { .. }) => {
                ("tooManyFilterConditions", self.to_string())
            }
            CommandError::View(ViewError::PropertyNotFound { .. }) => {
                ("propertyNotFound", self.to_string())
            }
            CommandError::View(ViewError::NoGroupCondition) => {
                ("noGroupCondition", self.to_string())
            }
            CommandError::View(ViewError::DuplicateSortProperty { .. }) => {
                ("duplicateSortProperty", self.to_string())
            }

            CommandError::Storage(_) => ("storage", self.to_string()),
        };

        let mut state = serializer.serialize_struct("CommandError", 2)?;
        state.serialize_field("kind", kind)?;
        state.serialize_field("message", &message)?;
        state.end()
    }
}
