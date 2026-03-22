use super::entity::{Page, PageId};
use super::error::PageError;

/// Maximum nesting depth for the page hierarchy (root = depth 1).
pub const MAX_DEPTH: usize = 5;

/// Domain service for page hierarchy validation.
///
/// All methods are pure functions operating on in-memory page data.
/// The IPC command layer is responsible for loading the required pages
/// from the repository and passing them to this service.
///
/// # Examples
///
/// ```no_run
/// # use rustydatabasenotes_lib::domain::page::hierarchy::PageHierarchyService;
/// # use rustydatabasenotes_lib::domain::page::entity::{Page, PageId, PageTitle};
/// let parent_title = PageTitle::try_from("Parent".to_owned())?;
/// let parent = Page::new(parent_title);
/// let depth = PageHierarchyService::depth(parent.id(), &[parent.clone()]);
/// assert_eq!(depth, 1); // root page is depth 1
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct PageHierarchyService;

impl PageHierarchyService {
    /// Validates that moving a page under a new parent is safe.
    ///
    /// Checks self-reference first, then circular reference via the ancestor
    /// chain, database page restriction, and finally depth limits.
    ///
    /// # Errors
    ///
    /// - [`PageError::CircularReference`] if `new_parent_id` is a descendant
    ///   of the page, or if the page would become its own parent.
    /// - [`PageError::MaxDepthExceeded`] if the resulting depth exceeds
    ///   [`MAX_DEPTH`].
    /// - [`PageError::DatabasePageCannotNest`] if the page is a database page.
    pub fn validate_move(
        page: &Page,
        new_parent_id: Option<&PageId>,
        ancestors_of_target: &[PageId],
        max_descendant_depth: usize,
    ) -> Result<(), PageError> {
        // Database pages cannot participate in hierarchy.
        if page.is_database_page() {
            return Err(PageError::DatabasePageCannotNest {
                page_id: page.id().to_string(),
            });
        }

        // Moving to root is always valid (no parent).
        let target_parent_id = match new_parent_id {
            Some(id) => id,
            None => return Ok(()),
        };

        // Self-reference check.
        if page.id() == target_parent_id {
            return Err(PageError::CircularReference {
                page_id: page.id().to_string(),
                target_parent_id: target_parent_id.to_string(),
            });
        }

        // Circular reference check: the page must not appear in the
        // ancestor chain of the target parent.
        if ancestors_of_target.contains(page.id()) {
            return Err(PageError::CircularReference {
                page_id: page.id().to_string(),
                target_parent_id: target_parent_id.to_string(),
            });
        }

        // Depth check: target parent depth + max descendant depth of page
        // must not exceed MAX_DEPTH.
        // ancestors_of_target length = number of ancestors of the target
        // parent (not including the target itself), so target depth =
        // ancestors_of_target.len() + 1. The page would be placed one level
        // below, so new depth = ancestors_of_target.len() + 2.
        // Adding max_descendant_depth gives the deepest resulting node.
        let new_page_depth = ancestors_of_target.len() + 2; // target depth + 1
        let deepest = new_page_depth + max_descendant_depth;
        if deepest > MAX_DEPTH {
            return Err(PageError::MaxDepthExceeded {
                page_id: page.id().to_string(),
                current_depth: deepest,
                max_depth: MAX_DEPTH,
            });
        }

        Ok(())
    }

    /// Validates that creating a child page under a parent is safe.
    ///
    /// # Errors
    ///
    /// - [`PageError::MaxDepthExceeded`] if the parent is already at
    ///   [`MAX_DEPTH`].
    /// - [`PageError::DatabasePageCannotNest`] if the parent is a
    ///   database page.
    pub fn validate_create_child(parent: &Page, parent_depth: usize) -> Result<(), PageError> {
        if parent.is_database_page() {
            return Err(PageError::DatabasePageCannotNest {
                page_id: parent.id().to_string(),
            });
        }

        // Child would be at parent_depth + 1.
        let child_depth = parent_depth + 1;
        if child_depth > MAX_DEPTH {
            return Err(PageError::MaxDepthExceeded {
                page_id: parent.id().to_string(),
                current_depth: child_depth,
                max_depth: MAX_DEPTH,
            });
        }

        Ok(())
    }

    /// Builds a list of ancestor [`PageId`]s from the given page to the root.
    ///
    /// The returned list starts with the immediate parent and ends with
    /// the root-level ancestor. If the page has no parent, returns an
    /// empty vec.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustydatabasenotes_lib::domain::page::hierarchy::PageHierarchyService;
    /// # use rustydatabasenotes_lib::domain::page::entity::{Page, PageId, PageTitle};
    /// let root = Page::new(PageTitle::try_from("Root".to_owned())?);
    /// let child = Page::new_child(
    ///     PageTitle::try_from("Child".to_owned())?,
    ///     root.id().clone(),
    /// );
    /// let pages = vec![root.clone(), child.clone()];
    /// let chain = PageHierarchyService::ancestor_chain(child.id(), &pages);
    /// assert_eq!(chain, vec![root.id().clone()]);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn ancestor_chain(page_id: &PageId, pages: &[Page]) -> Vec<PageId> {
        let mut chain = Vec::new();
        let mut current_id = page_id.clone();

        // Safety limit to prevent infinite loops from corrupt data.
        for _ in 0..MAX_DEPTH + 5 {
            let page = pages.iter().find(|p| p.id() == &current_id);
            match page.and_then(|p| p.parent_id()) {
                Some(pid) => {
                    chain.push(pid.clone());
                    current_id = pid.clone();
                }
                None => break,
            }
        }

        chain
    }

    /// Calculates the depth of a page (root = 1).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustydatabasenotes_lib::domain::page::hierarchy::PageHierarchyService;
    /// # use rustydatabasenotes_lib::domain::page::entity::{Page, PageId, PageTitle};
    /// let root = Page::new(PageTitle::try_from("Root".to_owned())?);
    /// assert_eq!(PageHierarchyService::depth(root.id(), &[root.clone()]), 1);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn depth(page_id: &PageId, pages: &[Page]) -> usize {
        Self::ancestor_chain(page_id, pages).len() + 1
    }

    /// Calculates the maximum depth of descendants from a given page.
    ///
    /// Returns `0` if the page has no children (i.e., the page itself
    /// does not add extra depth beyond its own level).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustydatabasenotes_lib::domain::page::hierarchy::PageHierarchyService;
    /// # use rustydatabasenotes_lib::domain::page::entity::{Page, PageId, PageTitle};
    /// let root = Page::new(PageTitle::try_from("Root".to_owned())?);
    /// assert_eq!(
    ///     PageHierarchyService::max_descendant_depth(root.id(), &[root.clone()]),
    ///     0
    /// );
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn max_descendant_depth(page_id: &PageId, pages: &[Page]) -> usize {
        let children: Vec<&Page> = pages
            .iter()
            .filter(|p| p.parent_id() == Some(page_id))
            .collect();

        if children.is_empty() {
            return 0;
        }

        let mut max = 0;
        for child in children {
            let child_depth = 1 + Self::max_descendant_depth(child.id(), pages);
            if child_depth > max {
                max = child_depth;
            }
        }
        max
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::cloned_ref_to_slice_refs)]
mod tests {
    use super::*;
    use crate::domain::page::entity::PageTitle;

    fn make_page(title: &str) -> Page {
        Page::new(PageTitle::try_from(title.to_owned()).expect("valid title"))
    }

    fn make_child(title: &str, parent_id: PageId) -> Page {
        Page::new_child(
            PageTitle::try_from(title.to_owned()).expect("valid title"),
            parent_id,
        )
    }

    #[test]
    fn depth_root_is_1() {
        let root = make_page("Root");
        assert_eq!(PageHierarchyService::depth(root.id(), &[root.clone()]), 1);
    }

    #[test]
    fn depth_child_is_2() {
        let root = make_page("Root");
        let child = make_child("Child", root.id().clone());
        let pages = vec![root.clone(), child.clone()];
        assert_eq!(PageHierarchyService::depth(child.id(), &pages), 2);
    }

    #[test]
    fn ancestor_chain_root_is_empty() {
        let root = make_page("Root");
        let chain = PageHierarchyService::ancestor_chain(root.id(), &[root.clone()]);
        assert!(chain.is_empty());
    }

    #[test]
    fn ancestor_chain_three_levels() {
        let root = make_page("Root");
        let mid = make_child("Mid", root.id().clone());
        let leaf = make_child("Leaf", mid.id().clone());
        let pages = vec![root.clone(), mid.clone(), leaf.clone()];

        let chain = PageHierarchyService::ancestor_chain(leaf.id(), &pages);
        assert_eq!(chain, vec![mid.id().clone(), root.id().clone()]);
    }

    #[test]
    fn max_descendant_depth_no_children() {
        let root = make_page("Root");
        assert_eq!(
            PageHierarchyService::max_descendant_depth(root.id(), &[root.clone()]),
            0
        );
    }

    #[test]
    fn max_descendant_depth_with_children() {
        let root = make_page("Root");
        let c1 = make_child("C1", root.id().clone());
        let c2 = make_child("C2", root.id().clone());
        let c1_1 = make_child("C1.1", c1.id().clone());
        let pages = vec![root.clone(), c1.clone(), c2.clone(), c1_1.clone()];

        // root -> c1 -> c1_1 (depth 2), root -> c2 (depth 1)
        assert_eq!(
            PageHierarchyService::max_descendant_depth(root.id(), &pages),
            2
        );
    }

    #[test]
    fn validate_move_rejects_self_reference() {
        let page = make_page("Page");
        let result = PageHierarchyService::validate_move(&page, Some(page.id()), &[], 0);
        assert!(matches!(result, Err(PageError::CircularReference { .. })));
    }

    #[test]
    fn validate_move_rejects_circular_reference() {
        let parent = make_page("Parent");
        let child = make_child("Child", parent.id().clone());

        // Trying to move parent under child — child's ancestors include parent
        let ancestors_of_child = vec![parent.id().clone()]; // not useful here
        // But we want to check: moving `parent` under `child`
        // ancestors_of_target (child) = [] (child has no ancestors from
        // child's perspective... wait, we need the ancestors of `child`
        // which is [parent_id])
        // Actually, the ancestors_of_target param is the ancestors of the
        // new_parent_id. So if we're moving parent under child,
        // new_parent_id=child.id(), and ancestors_of_target should be the
        // ancestor chain of child = [parent.id()].
        let result =
            PageHierarchyService::validate_move(&parent, Some(child.id()), &ancestors_of_child, 0);
        assert!(matches!(result, Err(PageError::CircularReference { .. })));
    }

    #[test]
    fn validate_move_to_root_always_succeeds() {
        let page = make_page("Page");
        let result = PageHierarchyService::validate_move(&page, None, &[], 0);
        assert!(result.is_ok());
    }

    #[test]
    fn validate_move_rejects_depth_exceeded() {
        let page = make_page("Page");
        let target = make_page("Target");
        // Target is at depth 4 (3 ancestors + itself), page has 1
        // descendant level. New depth = 3 + 2 = 5 (page at 5),
        // deepest = 5 + 1 = 6 > MAX_DEPTH(5).
        let ancestors = vec![PageId::new(), PageId::new(), PageId::new()];
        let result = PageHierarchyService::validate_move(&page, Some(target.id()), &ancestors, 1);
        assert!(matches!(result, Err(PageError::MaxDepthExceeded { .. })));
    }

    #[test]
    fn validate_move_accepts_at_max_depth() {
        let page = make_page("Page");
        let target = make_page("Target");
        // Target at depth 4 (3 ancestors). Page at depth 5 = 3 + 2.
        // No descendants (0). Deepest = 5 <= MAX_DEPTH(5). OK.
        let ancestors = vec![PageId::new(), PageId::new(), PageId::new()];
        let result = PageHierarchyService::validate_move(&page, Some(target.id()), &ancestors, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn validate_move_rejects_database_page() {
        let db_page = Page::from_stored(
            PageId::new(),
            PageTitle::try_from("DB Page".to_owned()).expect("valid"),
            Some(crate::domain::database::entity::DatabaseId::new()),
            None,
            0,
            chrono::Utc::now(),
            chrono::Utc::now(),
        );
        let result = PageHierarchyService::validate_move(&db_page, Some(&PageId::new()), &[], 0);
        assert!(matches!(
            result,
            Err(PageError::DatabasePageCannotNest { .. })
        ));
    }

    #[test]
    fn validate_create_child_success() {
        let parent = make_page("Parent");
        let result = PageHierarchyService::validate_create_child(&parent, 1);
        assert!(result.is_ok());
    }

    #[test]
    fn validate_create_child_rejects_at_max_depth() {
        let parent = make_page("Parent");
        let result = PageHierarchyService::validate_create_child(&parent, MAX_DEPTH);
        assert!(matches!(result, Err(PageError::MaxDepthExceeded { .. })));
    }

    #[test]
    fn validate_create_child_rejects_database_page() {
        let db_page = Page::from_stored(
            PageId::new(),
            PageTitle::try_from("DB Page".to_owned()).expect("valid"),
            Some(crate::domain::database::entity::DatabaseId::new()),
            None,
            0,
            chrono::Utc::now(),
            chrono::Utc::now(),
        );
        let result = PageHierarchyService::validate_create_child(&db_page, 1);
        assert!(matches!(
            result,
            Err(PageError::DatabasePageCannotNest { .. })
        ));
    }
}
