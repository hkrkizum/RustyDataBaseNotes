use crate::domain::block::entity::{Block, BlockContent, BlockId, BlockPosition};
use crate::domain::block::error::BlockError;
use crate::domain::page::entity::PageId;

/// An in-memory editing session for a single page's blocks.
///
/// `EditorSession` is a pure domain service with no database dependency.
/// It manages the block list, tracks dirty state, and provides all
/// block-manipulation operations (add, edit, move, remove).
///
/// The session is created by loading blocks from persistence via the IPC
/// layer and is stored in [`AppState`](crate::AppState).
pub struct EditorSession {
    page_id: PageId,
    blocks: Vec<Block>,
    is_dirty: bool,
}

impl EditorSession {
    /// Creates a new editor session for the given page with pre-loaded blocks.
    ///
    /// The blocks are assumed to be sorted by position. The session starts
    /// in a clean (not dirty) state.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustydatabasenotes_lib::domain::editor::session::EditorSession;
    /// # use rustydatabasenotes_lib::domain::page::entity::PageId;
    /// let session = EditorSession::new(PageId::new(), vec![]);
    /// assert!(!session.is_dirty());
    /// assert!(session.blocks().is_empty());
    /// ```
    pub fn new(page_id: PageId, blocks: Vec<Block>) -> Self {
        Self {
            page_id,
            blocks,
            is_dirty: false,
        }
    }

    /// Returns a reference to the page ID for this session.
    pub fn page_id(&self) -> &PageId {
        &self.page_id
    }

    /// Returns the current block list in position order.
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    /// Returns whether the session has unsaved changes.
    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    /// Marks the session as saved (not dirty).
    ///
    /// Called after successful persistence of all blocks.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustydatabasenotes_lib::domain::editor::session::EditorSession;
    /// # use rustydatabasenotes_lib::domain::page::entity::PageId;
    /// let mut session = EditorSession::new(PageId::new(), vec![]);
    /// session.add_block();
    /// assert!(session.is_dirty());
    /// session.mark_saved();
    /// assert!(!session.is_dirty());
    /// ```
    pub fn mark_saved(&mut self) {
        self.is_dirty = false;
    }

    /// Appends a new empty text block at the end of the block list.
    ///
    /// Generates a UUIDv7 identifier immediately and marks the session dirty.
    /// Returns a reference to the newly created block.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustydatabasenotes_lib::domain::editor::session::EditorSession;
    /// # use rustydatabasenotes_lib::domain::page::entity::PageId;
    /// let mut session = EditorSession::new(PageId::new(), vec![]);
    /// let block = session.add_block();
    /// assert_eq!(block.content().as_str(), "");
    /// assert!(session.is_dirty());
    /// ```
    pub fn add_block(&mut self) -> &Block {
        let position_value = self.blocks.len() as i64;
        // Safety: position is always non-negative (derived from Vec::len)
        let position = match BlockPosition::try_from(position_value) {
            Ok(p) => p,
            Err(_) => {
                // This branch is unreachable since len() >= 0, but we handle it
                // defensively per Principle VII.
                let p = BlockPosition::try_from(0_i64);
                match p {
                    Ok(pos) => pos,
                    // This is truly unreachable; 0 is always valid.
                    // We still avoid panic per constitution.
                    Err(_) => return &self.blocks[0],
                }
            }
        };

        let block = Block::new(self.page_id.clone(), position);
        self.blocks.push(block);
        self.is_dirty = true;
        // We just pushed, so last() is always Some
        &self.blocks[self.blocks.len() - 1]
    }

    /// Updates the content of the block identified by `id`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustydatabasenotes_lib::domain::editor::session::EditorSession;
    /// # use rustydatabasenotes_lib::domain::page::entity::PageId;
    /// let mut session = EditorSession::new(PageId::new(), vec![]);
    /// let block_id = session.add_block().id().clone();
    /// let result = session.edit_block_content(&block_id, "Hello".to_owned());
    /// assert!(result.is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// - [`BlockError::NotFound`] if no block has the given ID.
    /// - [`BlockError::ContentTooLong`] if the content exceeds 10,000 characters.
    pub fn edit_block_content(&mut self, id: &BlockId, content: String) -> Result<(), BlockError> {
        let block = self
            .blocks
            .iter_mut()
            .find(|b| b.id() == id)
            .ok_or_else(|| BlockError::NotFound { id: id.to_string() })?;

        let validated = BlockContent::try_from(content)?;
        block.set_content(validated);
        self.is_dirty = true;
        Ok(())
    }

    /// Moves the block identified by `id` one position up (toward index 0).
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustydatabasenotes_lib::domain::editor::session::EditorSession;
    /// # use rustydatabasenotes_lib::domain::page::entity::PageId;
    /// let mut session = EditorSession::new(PageId::new(), vec![]);
    /// let _ = session.add_block();
    /// let id = session.add_block().id().clone();
    /// let result = session.move_block_up(&id);
    /// assert!(result.is_ok());
    /// assert_eq!(session.blocks()[0].id(), &id);
    /// ```
    ///
    /// # Errors
    ///
    /// - [`BlockError::NotFound`] if no block has the given ID.
    /// - [`BlockError::CannotMoveUp`] if the block is already at position 0.
    pub fn move_block_up(&mut self, id: &BlockId) -> Result<(), BlockError> {
        let index = self
            .blocks
            .iter()
            .position(|b| b.id() == id)
            .ok_or_else(|| BlockError::NotFound { id: id.to_string() })?;

        if index == 0 {
            return Err(BlockError::CannotMoveUp { id: id.to_string() });
        }

        self.blocks.swap(index, index - 1);
        self.renumber_positions();
        self.is_dirty = true;
        Ok(())
    }

    /// Moves the block identified by `id` one position down (toward the end).
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustydatabasenotes_lib::domain::editor::session::EditorSession;
    /// # use rustydatabasenotes_lib::domain::page::entity::PageId;
    /// let mut session = EditorSession::new(PageId::new(), vec![]);
    /// let id = session.add_block().id().clone();
    /// let _ = session.add_block();
    /// let result = session.move_block_down(&id);
    /// assert!(result.is_ok());
    /// assert_eq!(session.blocks()[1].id(), &id);
    /// ```
    ///
    /// # Errors
    ///
    /// - [`BlockError::NotFound`] if no block has the given ID.
    /// - [`BlockError::CannotMoveDown`] if the block is already at the last position.
    pub fn move_block_down(&mut self, id: &BlockId) -> Result<(), BlockError> {
        let index = self
            .blocks
            .iter()
            .position(|b| b.id() == id)
            .ok_or_else(|| BlockError::NotFound { id: id.to_string() })?;

        if index >= self.blocks.len() - 1 {
            return Err(BlockError::CannotMoveDown { id: id.to_string() });
        }

        self.blocks.swap(index, index + 1);
        self.renumber_positions();
        self.is_dirty = true;
        Ok(())
    }

    /// Removes the block identified by `id` and renumbers remaining positions.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustydatabasenotes_lib::domain::editor::session::EditorSession;
    /// # use rustydatabasenotes_lib::domain::page::entity::PageId;
    /// let mut session = EditorSession::new(PageId::new(), vec![]);
    /// let id = session.add_block().id().clone();
    /// let result = session.remove_block(&id);
    /// assert!(result.is_ok());
    /// assert!(session.blocks().is_empty());
    /// ```
    ///
    /// # Errors
    ///
    /// - [`BlockError::NotFound`] if no block has the given ID.
    pub fn remove_block(&mut self, id: &BlockId) -> Result<(), BlockError> {
        let index = self
            .blocks
            .iter()
            .position(|b| b.id() == id)
            .ok_or_else(|| BlockError::NotFound { id: id.to_string() })?;

        self.blocks.remove(index);
        self.renumber_positions();
        self.is_dirty = true;
        Ok(())
    }

    /// Renumbers block positions from 0 to len-1.
    fn renumber_positions(&mut self) {
        for (i, block) in self.blocks.iter_mut().enumerate() {
            // i is always non-negative, so this conversion is safe
            if let Ok(pos) = BlockPosition::try_from(i as i64) {
                block.set_position(pos);
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

    fn make_session_with_blocks(n: usize) -> EditorSession {
        let page_id = PageId::new();
        let mut session = EditorSession::new(page_id, vec![]);
        for _ in 0..n {
            session.add_block();
        }
        session.mark_saved(); // Reset dirty to test mutations
        session
    }

    // T018: EditorSession::new() tests

    #[test]
    fn new_session_is_not_dirty() {
        let session = EditorSession::new(PageId::new(), vec![]);
        assert!(!session.is_dirty());
    }

    #[test]
    fn new_session_with_empty_blocks() {
        let session = EditorSession::new(PageId::new(), vec![]);
        assert!(session.blocks().is_empty());
    }

    #[test]
    fn new_session_with_preloaded_blocks() {
        let page_id = PageId::new();
        let pos0 = BlockPosition::try_from(0_i64).unwrap();
        let pos1 = BlockPosition::try_from(1_i64).unwrap();
        let b0 = Block::new(page_id.clone(), pos0);
        let b1 = Block::new(page_id.clone(), pos1);
        let session = EditorSession::new(page_id, vec![b0, b1]);
        assert_eq!(session.blocks().len(), 2);
        assert_eq!(session.blocks()[0].position().value(), 0);
        assert_eq!(session.blocks()[1].position().value(), 1);
        assert!(!session.is_dirty());
    }

    // T031: EditorSession::add_block() tests

    #[test]
    fn add_block_appends_at_end() {
        let mut session = EditorSession::new(PageId::new(), vec![]);
        let id1 = session.add_block().id().clone();
        let id2 = session.add_block().id().clone();
        assert_eq!(session.blocks().len(), 2);
        assert_eq!(session.blocks()[0].id(), &id1);
        assert_eq!(session.blocks()[1].id(), &id2);
    }

    #[test]
    fn add_block_sets_position_to_len() {
        let mut session = EditorSession::new(PageId::new(), vec![]);
        session.add_block();
        session.add_block();
        let block = session.add_block();
        assert_eq!(block.position().value(), 2);
    }

    #[test]
    fn add_block_marks_dirty() {
        let mut session = EditorSession::new(PageId::new(), vec![]);
        assert!(!session.is_dirty());
        session.add_block();
        assert!(session.is_dirty());
    }

    #[test]
    fn add_block_assigns_uuidv7() {
        let mut session = EditorSession::new(PageId::new(), vec![]);
        let block = session.add_block();
        let id_str = block.id().to_string();
        assert_eq!(id_str.len(), 36); // UUIDv7 format
    }

    #[test]
    fn add_block_has_empty_content() {
        let mut session = EditorSession::new(PageId::new(), vec![]);
        let block = session.add_block();
        assert_eq!(block.content().as_str(), "");
    }

    // T038: EditorSession::edit_block_content() tests

    #[test]
    fn edit_block_content_updates_content() {
        let mut session = make_session_with_blocks(1);
        let id = session.blocks()[0].id().clone();
        session.edit_block_content(&id, "Hello".to_owned()).unwrap();
        assert_eq!(session.blocks()[0].content().as_str(), "Hello");
    }

    #[test]
    fn edit_block_content_marks_dirty() {
        let mut session = make_session_with_blocks(1);
        let id = session.blocks()[0].id().clone();
        assert!(!session.is_dirty());
        session.edit_block_content(&id, "x".to_owned()).unwrap();
        assert!(session.is_dirty());
    }

    #[test]
    fn edit_block_content_too_long() {
        let mut session = make_session_with_blocks(1);
        let id = session.blocks()[0].id().clone();
        let long_content = "x".repeat(10_001);
        let result = session.edit_block_content(&id, long_content);
        assert!(matches!(
            result,
            Err(BlockError::ContentTooLong {
                len: 10_001,
                max: 10_000
            })
        ));
    }

    #[test]
    fn edit_block_content_not_found() {
        let mut session = make_session_with_blocks(1);
        let fake_id = BlockId::new();
        let result = session.edit_block_content(&fake_id, "x".to_owned());
        assert!(matches!(result, Err(BlockError::NotFound { .. })));
    }

    #[test]
    fn edit_block_content_empty_accepted() {
        let mut session = make_session_with_blocks(1);
        let id = session.blocks()[0].id().clone();
        session
            .edit_block_content(&id, "some text".to_owned())
            .unwrap();
        session.edit_block_content(&id, String::new()).unwrap();
        assert_eq!(session.blocks()[0].content().as_str(), "");
    }

    // T054: EditorSession::move_block_up() and move_block_down() tests

    #[test]
    fn move_block_up_swaps() {
        let mut session = make_session_with_blocks(3);
        let id1 = session.blocks()[1].id().clone();
        session.move_block_up(&id1).unwrap();
        assert_eq!(session.blocks()[0].id(), &id1);
    }

    #[test]
    fn move_block_up_at_top_errors() {
        let mut session = make_session_with_blocks(2);
        let id0 = session.blocks()[0].id().clone();
        let result = session.move_block_up(&id0);
        assert!(matches!(result, Err(BlockError::CannotMoveUp { .. })));
    }

    #[test]
    fn move_block_up_not_found() {
        let mut session = make_session_with_blocks(2);
        let fake_id = BlockId::new();
        let result = session.move_block_up(&fake_id);
        assert!(matches!(result, Err(BlockError::NotFound { .. })));
    }

    #[test]
    fn move_block_up_marks_dirty() {
        let mut session = make_session_with_blocks(2);
        let id1 = session.blocks()[1].id().clone();
        assert!(!session.is_dirty());
        session.move_block_up(&id1).unwrap();
        assert!(session.is_dirty());
    }

    #[test]
    fn move_block_down_swaps() {
        let mut session = make_session_with_blocks(3);
        let id0 = session.blocks()[0].id().clone();
        session.move_block_down(&id0).unwrap();
        assert_eq!(session.blocks()[1].id(), &id0);
    }

    #[test]
    fn move_block_down_at_bottom_errors() {
        let mut session = make_session_with_blocks(2);
        let id1 = session.blocks()[1].id().clone();
        let result = session.move_block_down(&id1);
        assert!(matches!(result, Err(BlockError::CannotMoveDown { .. })));
    }

    #[test]
    fn move_block_down_marks_dirty() {
        let mut session = make_session_with_blocks(2);
        let id0 = session.blocks()[0].id().clone();
        assert!(!session.is_dirty());
        session.move_block_down(&id0).unwrap();
        assert!(session.is_dirty());
    }

    // T061: EditorSession::remove_block() tests

    #[test]
    fn remove_block_removes() {
        let mut session = make_session_with_blocks(2);
        let id0 = session.blocks()[0].id().clone();
        session.remove_block(&id0).unwrap();
        assert_eq!(session.blocks().len(), 1);
    }

    #[test]
    fn remove_block_renumbers_positions() {
        let mut session = make_session_with_blocks(3);
        let id1 = session.blocks()[1].id().clone();
        session.remove_block(&id1).unwrap();
        assert_eq!(session.blocks()[0].position().value(), 0);
        assert_eq!(session.blocks()[1].position().value(), 1);
    }

    #[test]
    fn remove_block_not_found() {
        let mut session = make_session_with_blocks(1);
        let fake_id = BlockId::new();
        let result = session.remove_block(&fake_id);
        assert!(matches!(result, Err(BlockError::NotFound { .. })));
    }

    #[test]
    fn remove_last_block_returns_empty() {
        let mut session = make_session_with_blocks(1);
        let id = session.blocks()[0].id().clone();
        session.remove_block(&id).unwrap();
        assert!(session.blocks().is_empty());
    }

    #[test]
    fn remove_block_marks_dirty() {
        let mut session = make_session_with_blocks(1);
        let id = session.blocks()[0].id().clone();
        assert!(!session.is_dirty());
        session.remove_block(&id).unwrap();
        assert!(session.is_dirty());
    }

    // T068: EditorSession::is_dirty() state transitions

    #[test]
    fn dirty_starts_false() {
        let session = EditorSession::new(PageId::new(), vec![]);
        assert!(!session.is_dirty());
    }

    #[test]
    fn dirty_after_add() {
        let mut session = EditorSession::new(PageId::new(), vec![]);
        session.add_block();
        assert!(session.is_dirty());
    }

    #[test]
    fn dirty_after_edit() {
        let mut session = make_session_with_blocks(1);
        let id = session.blocks()[0].id().clone();
        session.edit_block_content(&id, "x".to_owned()).unwrap();
        assert!(session.is_dirty());
    }

    #[test]
    fn dirty_after_move() {
        let mut session = make_session_with_blocks(2);
        let id1 = session.blocks()[1].id().clone();
        session.move_block_up(&id1).unwrap();
        assert!(session.is_dirty());
    }

    #[test]
    fn dirty_after_remove() {
        let mut session = make_session_with_blocks(1);
        let id = session.blocks()[0].id().clone();
        session.remove_block(&id).unwrap();
        assert!(session.is_dirty());
    }

    #[test]
    fn dirty_false_after_mark_saved() {
        let mut session = make_session_with_blocks(1);
        let id = session.blocks()[0].id().clone();
        session
            .edit_block_content(&id, "changed".to_owned())
            .unwrap();
        assert!(session.is_dirty());
        session.mark_saved();
        assert!(!session.is_dirty());
    }

    // T074: Individual operation performance benchmarks (<100ms each)

    #[test]
    fn performance_add_block_under_100ms() {
        let mut session = make_session_with_blocks(1_000);
        let start = std::time::Instant::now();
        session.add_block();
        let elapsed = start.elapsed();
        assert!(
            elapsed < std::time::Duration::from_millis(100),
            "add_block took {elapsed:?}"
        );
    }

    #[test]
    fn performance_edit_block_content_under_100ms() {
        let mut session = make_session_with_blocks(1_000);
        let id = session.blocks()[500].id().clone();
        let start = std::time::Instant::now();
        session
            .edit_block_content(&id, "Updated content".to_owned())
            .unwrap();
        let elapsed = start.elapsed();
        assert!(
            elapsed < std::time::Duration::from_millis(100),
            "edit_block_content took {elapsed:?}"
        );
    }

    #[test]
    fn performance_move_block_up_under_100ms() {
        let mut session = make_session_with_blocks(1_000);
        let id = session.blocks()[500].id().clone();
        let start = std::time::Instant::now();
        session.move_block_up(&id).unwrap();
        let elapsed = start.elapsed();
        assert!(
            elapsed < std::time::Duration::from_millis(100),
            "move_block_up took {elapsed:?}"
        );
    }

    #[test]
    fn performance_remove_block_under_100ms() {
        let mut session = make_session_with_blocks(1_000);
        let id = session.blocks()[500].id().clone();
        let start = std::time::Instant::now();
        session.remove_block(&id).unwrap();
        let elapsed = start.elapsed();
        assert!(
            elapsed < std::time::Duration::from_millis(100),
            "remove_block took {elapsed:?}"
        );
    }
}
