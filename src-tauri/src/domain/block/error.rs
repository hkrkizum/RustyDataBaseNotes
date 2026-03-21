use std::fmt;

/// Errors originating from the Block domain model.
#[derive(Debug, PartialEq)]
pub enum BlockError {
    /// The block content exceeded the maximum allowed character count.
    ContentTooLong {
        /// Actual character count.
        len: usize,
        /// Maximum allowed character count.
        max: usize,
    },

    /// An invalid (negative) block position was provided.
    InvalidPosition {
        /// The invalid position value.
        value: i64,
    },

    /// No block was found with the given ID.
    NotFound {
        /// The ID that was looked up.
        id: String,
    },

    /// The block is already at the top and cannot move up.
    CannotMoveUp {
        /// The ID of the block.
        id: String,
    },

    /// The block is already at the bottom and cannot move down.
    CannotMoveDown {
        /// The ID of the block.
        id: String,
    },
}

impl fmt::Display for BlockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockError::ContentTooLong { len, max } => {
                write!(f, "block content too long: {len} characters (max: {max})")
            }
            BlockError::InvalidPosition { value } => {
                write!(f, "invalid block position: {value}")
            }
            BlockError::NotFound { id } => {
                write!(f, "block not found: {id}")
            }
            BlockError::CannotMoveUp { id } => {
                write!(f, "cannot move block up: {id} is at the top")
            }
            BlockError::CannotMoveDown { id } => {
                write!(f, "cannot move block down: {id} is at the bottom")
            }
        }
    }
}

impl std::error::Error for BlockError {}
