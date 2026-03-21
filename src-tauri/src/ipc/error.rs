use serde::Serialize;

use crate::domain::block::error::BlockError;
use crate::domain::page::error::PageError;
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
            CommandError::Storage(_) => ("storage", self.to_string()),
        };

        let mut state = serializer.serialize_struct("CommandError", 2)?;
        state.serialize_field("kind", kind)?;
        state.serialize_field("message", &message)?;
        state.end()
    }
}
