use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::database::entity::DatabaseId;
use crate::domain::property::entity::PropertyId;

use super::error::ViewError;

/// Maximum number of sort conditions a view may contain.
pub const MAX_SORT_CONDITIONS: usize = 5;

/// Maximum number of filter conditions a view may contain.
pub const MAX_FILTER_CONDITIONS: usize = 20;

/// Maximum number of characters allowed in a view name.
pub const MAX_VIEW_NAME_LENGTH: usize = 100;

// ---------------------------------------------------------------------------
// ViewId
// ---------------------------------------------------------------------------

/// A UUIDv7-based identifier for a [`View`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ViewId(Uuid);

impl ViewId {
    /// Generates a new time-ordered UUIDv7 identifier.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Returns the inner [`Uuid`] value.
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for ViewId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ViewId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ViewId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s).map(Self)
    }
}

// ---------------------------------------------------------------------------
// ViewName
// ---------------------------------------------------------------------------

/// A validated view name (1–100 Unicode characters after trimming).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ViewName(String);

impl ViewName {
    /// Returns the inner string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ViewName {
    type Error = ViewError;

    /// Creates a new [`ViewName`] after trimming and validating the input.
    ///
    /// # Errors
    ///
    /// Returns [`ViewError::InvalidSortCondition`] if the name is empty or too long.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim().to_owned();
        if trimmed.is_empty() {
            return Err(ViewError::InvalidSortCondition {
                reason: "view name must not be empty".to_owned(),
            });
        }
        let len = trimmed.chars().count();
        if len > MAX_VIEW_NAME_LENGTH {
            return Err(ViewError::InvalidSortCondition {
                reason: format!(
                    "view name too long: {len} characters (max {MAX_VIEW_NAME_LENGTH})"
                ),
            });
        }
        Ok(Self(trimmed))
    }
}

impl fmt::Display for ViewName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// ViewType
// ---------------------------------------------------------------------------

/// The type of view layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ViewType {
    /// Standard table layout.
    Table,
}

impl fmt::Display for ViewType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Table => write!(f, "table"),
        }
    }
}

// ---------------------------------------------------------------------------
// SortDirection
// ---------------------------------------------------------------------------

/// The direction for sorting rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SortDirection {
    /// Sort in ascending order.
    Ascending,
    /// Sort in descending order.
    Descending,
}

// ---------------------------------------------------------------------------
// SortCondition
// ---------------------------------------------------------------------------

/// A single sort condition referencing a property and direction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortCondition {
    /// The property to sort by.
    pub property_id: PropertyId,
    /// The sort direction.
    pub direction: SortDirection,
}

// ---------------------------------------------------------------------------
// FilterOperator
// ---------------------------------------------------------------------------

/// Filter comparison operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FilterOperator {
    // Text + common
    /// Exact equality (case-insensitive for text).
    Equals,
    /// Not equal (case-insensitive for text).
    NotEquals,
    /// Contains substring (case-insensitive for text).
    Contains,
    /// Does not contain substring (case-insensitive for text).
    NotContains,

    // Number
    /// Greater than.
    GreaterThan,
    /// Less than.
    LessThan,
    /// Greater than or equal.
    GreaterOrEqual,
    /// Less than or equal.
    LessOrEqual,

    // Date
    /// Before a date (exclusive).
    Before,
    /// After a date (inclusive).
    After,

    // Select
    /// Is a specific option.
    Is,
    /// Is not a specific option.
    IsNot,

    // Checkbox
    /// Checkbox is checked.
    IsChecked,
    /// Checkbox is unchecked.
    IsUnchecked,

    // Common
    /// Value is empty (null).
    IsEmpty,
    /// Value is not empty (not null).
    IsNotEmpty,
}

impl fmt::Display for FilterOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Equals => "equals",
            Self::NotEquals => "notEquals",
            Self::Contains => "contains",
            Self::NotContains => "notContains",
            Self::GreaterThan => "greaterThan",
            Self::LessThan => "lessThan",
            Self::GreaterOrEqual => "greaterOrEqual",
            Self::LessOrEqual => "lessOrEqual",
            Self::Before => "before",
            Self::After => "after",
            Self::Is => "is",
            Self::IsNot => "isNot",
            Self::IsChecked => "isChecked",
            Self::IsUnchecked => "isUnchecked",
            Self::IsEmpty => "isEmpty",
            Self::IsNotEmpty => "isNotEmpty",
        };
        write!(f, "{s}")
    }
}

// ---------------------------------------------------------------------------
// FilterValue
// ---------------------------------------------------------------------------

/// A type-safe filter comparison value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum FilterValue {
    /// Text comparison value.
    Text(String),
    /// Numeric comparison value.
    Number(f64),
    /// Date comparison value (ISO 8601 string).
    Date(String),
    /// Select option comparison value.
    SelectOption(String),
}

// ---------------------------------------------------------------------------
// FilterCondition
// ---------------------------------------------------------------------------

/// A single filter condition referencing a property, operator, and optional value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterCondition {
    /// The property to filter by.
    pub property_id: PropertyId,
    /// The filter operator.
    pub operator: FilterOperator,
    /// The comparison value (None for IsEmpty/IsNotEmpty/IsChecked/IsUnchecked).
    pub value: Option<FilterValue>,
}

// ---------------------------------------------------------------------------
// GroupCondition
// ---------------------------------------------------------------------------

/// A grouping condition referencing a single property.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupCondition {
    /// The property to group by.
    pub property_id: PropertyId,
}

// ---------------------------------------------------------------------------
// View
// ---------------------------------------------------------------------------

/// A view entity — holds display settings (sort, filter, group) for a database.
#[derive(Debug, Clone)]
pub struct View {
    id: ViewId,
    database_id: DatabaseId,
    name: ViewName,
    view_type: ViewType,
    sort_conditions: Vec<SortCondition>,
    filter_conditions: Vec<FilterCondition>,
    group_condition: Option<GroupCondition>,
    collapsed_groups: HashSet<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl View {
    /// Creates a new default [`View`] for the given database.
    ///
    /// The view is initialized with:
    /// - Name: "Table"
    /// - Type: Table
    /// - Empty sort/filter/group conditions
    pub fn new_default(database_id: DatabaseId) -> Self {
        let now = Utc::now();
        Self {
            id: ViewId::new(),
            database_id,
            name: ViewName("Table".to_owned()),
            view_type: ViewType::Table,
            sort_conditions: Vec::new(),
            filter_conditions: Vec::new(),
            group_condition: None,
            collapsed_groups: HashSet::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Reconstructs a [`View`] from stored fields (e.g. database row).
    ///
    /// No validation is performed — the caller guarantees data integrity.
    #[allow(clippy::too_many_arguments)]
    pub fn from_stored(
        id: ViewId,
        database_id: DatabaseId,
        name: ViewName,
        view_type: ViewType,
        sort_conditions: Vec<SortCondition>,
        filter_conditions: Vec<FilterCondition>,
        group_condition: Option<GroupCondition>,
        collapsed_groups: HashSet<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            database_id,
            name,
            view_type,
            sort_conditions,
            filter_conditions,
            group_condition,
            collapsed_groups,
            created_at,
            updated_at,
        }
    }

    /// Returns a reference to the view's ID.
    pub fn id(&self) -> &ViewId {
        &self.id
    }

    /// Returns a reference to the owning database's ID.
    pub fn database_id(&self) -> &DatabaseId {
        &self.database_id
    }

    /// Returns a reference to the view's name.
    pub fn name(&self) -> &ViewName {
        &self.name
    }

    /// Returns the view's type.
    pub fn view_type(&self) -> ViewType {
        self.view_type
    }

    /// Returns a reference to the sort conditions.
    pub fn sort_conditions(&self) -> &[SortCondition] {
        &self.sort_conditions
    }

    /// Returns a reference to the filter conditions.
    pub fn filter_conditions(&self) -> &[FilterCondition] {
        &self.filter_conditions
    }

    /// Returns a reference to the group condition.
    pub fn group_condition(&self) -> Option<&GroupCondition> {
        self.group_condition.as_ref()
    }

    /// Returns a reference to the collapsed groups set.
    pub fn collapsed_groups(&self) -> &HashSet<String> {
        &self.collapsed_groups
    }

    /// Returns the view's creation timestamp.
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Returns the view's last-updated timestamp.
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// Validates and sets new sort conditions.
    ///
    /// # Errors
    ///
    /// - [`ViewError::TooManySortConditions`] if more than 5 conditions.
    /// - [`ViewError::DuplicateSortProperty`] if a property_id appears more than once.
    pub fn set_sort_conditions(&mut self, conditions: Vec<SortCondition>) -> Result<(), ViewError> {
        if conditions.len() > MAX_SORT_CONDITIONS {
            return Err(ViewError::TooManySortConditions {
                count: conditions.len(),
                max: MAX_SORT_CONDITIONS,
            });
        }

        let mut seen = HashSet::with_capacity(conditions.len());
        for cond in &conditions {
            if !seen.insert(&cond.property_id) {
                return Err(ViewError::DuplicateSortProperty {
                    id: cond.property_id.clone(),
                });
            }
        }

        self.sort_conditions = conditions;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Validates and sets new filter conditions.
    ///
    /// # Errors
    ///
    /// - [`ViewError::TooManyFilterConditions`] if more than 20 conditions.
    pub fn set_filter_conditions(
        &mut self,
        conditions: Vec<FilterCondition>,
    ) -> Result<(), ViewError> {
        if conditions.len() > MAX_FILTER_CONDITIONS {
            return Err(ViewError::TooManyFilterConditions {
                count: conditions.len(),
                max: MAX_FILTER_CONDITIONS,
            });
        }

        self.filter_conditions = conditions;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Sets or clears the group condition.
    ///
    /// When the group condition changes, collapsed_groups is cleared.
    pub fn set_group_condition(&mut self, condition: Option<GroupCondition>) {
        let changed = self.group_condition != condition;
        self.group_condition = condition;
        if changed {
            self.collapsed_groups.clear();
        }
        self.updated_at = Utc::now();
    }

    /// Toggles a group value's collapsed state.
    ///
    /// # Errors
    ///
    /// - [`ViewError::NoGroupCondition`] if no group condition is set.
    pub fn toggle_collapsed_group(&mut self, group_value: Option<String>) -> Result<(), ViewError> {
        if self.group_condition.is_none() {
            return Err(ViewError::NoGroupCondition);
        }

        // Use empty string as sentinel for null (未設定) group
        let key = group_value.unwrap_or_default();
        if self.collapsed_groups.contains(&key) {
            self.collapsed_groups.remove(&key);
        } else {
            self.collapsed_groups.insert(key);
        }

        self.updated_at = Utc::now();
        Ok(())
    }

    /// Resets all view settings to defaults.
    pub fn reset(&mut self) {
        self.sort_conditions.clear();
        self.filter_conditions.clear();
        self.group_condition = None;
        self.collapsed_groups.clear();
        self.updated_at = Utc::now();
    }

    /// Removes all references to a property ID from sort, filter, and group conditions.
    ///
    /// Returns `true` if any conditions were removed.
    pub fn remove_property_references(&mut self, property_id: &PropertyId) -> bool {
        let orig_sort = self.sort_conditions.len();
        let orig_filter = self.filter_conditions.len();

        self.sort_conditions
            .retain(|c| c.property_id != *property_id);
        self.filter_conditions
            .retain(|c| c.property_id != *property_id);

        let group_removed = if let Some(gc) = &self.group_condition {
            if gc.property_id == *property_id {
                self.group_condition = None;
                self.collapsed_groups.clear();
                true
            } else {
                false
            }
        } else {
            false
        };

        let changed = self.sort_conditions.len() != orig_sort
            || self.filter_conditions.len() != orig_filter
            || group_removed;

        if changed {
            self.updated_at = Utc::now();
        }

        changed
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn view_name_valid() {
        let name = ViewName::try_from("Table".to_owned());
        assert!(name.is_ok());
        assert_eq!(name.as_ref().map(|n| n.as_str()), Ok("Table"));
    }

    #[test]
    fn view_name_empty_is_rejected() {
        let result = ViewName::try_from(String::new());
        assert!(result.is_err());
    }

    #[test]
    fn view_name_over_100_chars_is_rejected() {
        let s = "a".repeat(101);
        let result = ViewName::try_from(s);
        assert!(result.is_err());
    }

    #[test]
    fn view_name_trims_whitespace() {
        let name = ViewName::try_from("  My View  ".to_owned());
        assert!(name.is_ok());
        assert_eq!(name.as_ref().map(|n| n.as_str()), Ok("My View"));
    }

    #[test]
    fn new_default_view_has_empty_conditions() {
        let view = View::new_default(DatabaseId::new());
        assert_eq!(view.name().as_str(), "Table");
        assert_eq!(view.view_type(), ViewType::Table);
        assert!(view.sort_conditions().is_empty());
        assert!(view.filter_conditions().is_empty());
        assert!(view.group_condition().is_none());
        assert!(view.collapsed_groups().is_empty());
    }

    #[test]
    fn set_sort_conditions_rejects_over_5() {
        let mut view = View::new_default(DatabaseId::new());
        let conditions: Vec<SortCondition> = (0..6)
            .map(|_| SortCondition {
                property_id: PropertyId::new(),
                direction: SortDirection::Ascending,
            })
            .collect();
        let result = view.set_sort_conditions(conditions);
        assert!(matches!(
            result,
            Err(ViewError::TooManySortConditions { count: 6, max: 5 })
        ));
    }

    #[test]
    fn set_sort_conditions_rejects_duplicate_property() {
        let mut view = View::new_default(DatabaseId::new());
        let pid = PropertyId::new();
        let conditions = vec![
            SortCondition {
                property_id: pid.clone(),
                direction: SortDirection::Ascending,
            },
            SortCondition {
                property_id: pid,
                direction: SortDirection::Descending,
            },
        ];
        let result = view.set_sort_conditions(conditions);
        assert!(matches!(
            result,
            Err(ViewError::DuplicateSortProperty { .. })
        ));
    }

    #[test]
    fn set_sort_conditions_accepts_valid() {
        let mut view = View::new_default(DatabaseId::new());
        let conditions = vec![SortCondition {
            property_id: PropertyId::new(),
            direction: SortDirection::Ascending,
        }];
        assert!(view.set_sort_conditions(conditions).is_ok());
        assert_eq!(view.sort_conditions().len(), 1);
    }

    #[test]
    fn set_filter_conditions_rejects_over_20() {
        let mut view = View::new_default(DatabaseId::new());
        let conditions: Vec<FilterCondition> = (0..21)
            .map(|_| FilterCondition {
                property_id: PropertyId::new(),
                operator: FilterOperator::IsEmpty,
                value: None,
            })
            .collect();
        let result = view.set_filter_conditions(conditions);
        assert!(matches!(
            result,
            Err(ViewError::TooManyFilterConditions { count: 21, max: 20 })
        ));
    }

    #[test]
    fn set_group_condition_clears_collapsed_groups() {
        let mut view = View::new_default(DatabaseId::new());
        let pid = PropertyId::new();
        view.set_group_condition(Some(GroupCondition {
            property_id: pid.clone(),
        }));
        view.toggle_collapsed_group(Some("value1".to_owned()))
            .expect("ok");
        assert!(!view.collapsed_groups().is_empty());

        // Change group property → collapsed should be cleared
        view.set_group_condition(Some(GroupCondition {
            property_id: PropertyId::new(),
        }));
        assert!(view.collapsed_groups().is_empty());
    }

    #[test]
    fn toggle_collapsed_group_without_condition_errors() {
        let mut view = View::new_default(DatabaseId::new());
        let result = view.toggle_collapsed_group(Some("value".to_owned()));
        assert!(matches!(result, Err(ViewError::NoGroupCondition)));
    }

    #[test]
    fn toggle_collapsed_group_adds_and_removes() {
        let mut view = View::new_default(DatabaseId::new());
        view.set_group_condition(Some(GroupCondition {
            property_id: PropertyId::new(),
        }));

        // First toggle → add
        view.toggle_collapsed_group(Some("grp".to_owned()))
            .expect("ok");
        assert!(view.collapsed_groups().contains("grp"));

        // Second toggle → remove
        view.toggle_collapsed_group(Some("grp".to_owned()))
            .expect("ok");
        assert!(!view.collapsed_groups().contains("grp"));
    }

    #[test]
    fn toggle_collapsed_group_null_uses_empty_string() {
        let mut view = View::new_default(DatabaseId::new());
        view.set_group_condition(Some(GroupCondition {
            property_id: PropertyId::new(),
        }));
        view.toggle_collapsed_group(None).expect("ok");
        assert!(view.collapsed_groups().contains(""));
    }

    #[test]
    fn reset_clears_all_conditions() {
        let mut view = View::new_default(DatabaseId::new());
        view.set_sort_conditions(vec![SortCondition {
            property_id: PropertyId::new(),
            direction: SortDirection::Ascending,
        }])
        .expect("ok");
        view.set_filter_conditions(vec![FilterCondition {
            property_id: PropertyId::new(),
            operator: FilterOperator::IsEmpty,
            value: None,
        }])
        .expect("ok");
        view.set_group_condition(Some(GroupCondition {
            property_id: PropertyId::new(),
        }));

        view.reset();
        assert!(view.sort_conditions().is_empty());
        assert!(view.filter_conditions().is_empty());
        assert!(view.group_condition().is_none());
        assert!(view.collapsed_groups().is_empty());
    }

    #[test]
    fn remove_property_references_removes_matching() {
        let mut view = View::new_default(DatabaseId::new());
        let target = PropertyId::new();
        let other = PropertyId::new();

        view.set_sort_conditions(vec![
            SortCondition {
                property_id: target.clone(),
                direction: SortDirection::Ascending,
            },
            SortCondition {
                property_id: other.clone(),
                direction: SortDirection::Descending,
            },
        ])
        .expect("ok");
        view.set_filter_conditions(vec![FilterCondition {
            property_id: target.clone(),
            operator: FilterOperator::IsEmpty,
            value: None,
        }])
        .expect("ok");
        view.set_group_condition(Some(GroupCondition {
            property_id: target.clone(),
        }));

        let changed = view.remove_property_references(&target);
        assert!(changed);
        assert_eq!(view.sort_conditions().len(), 1);
        assert_eq!(view.sort_conditions()[0].property_id, other);
        assert!(view.filter_conditions().is_empty());
        assert!(view.group_condition().is_none());
    }

    #[test]
    fn view_id_display_and_from_str_roundtrip() {
        let id = ViewId::new();
        let s = id.to_string();
        let parsed: ViewId = s.parse().expect("should parse");
        assert_eq!(id, parsed);
    }
}
