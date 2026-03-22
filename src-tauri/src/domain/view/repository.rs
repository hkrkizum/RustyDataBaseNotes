use super::entity::{FilterCondition, GroupCondition, SortCondition, View};
use super::error::ViewError;
use crate::domain::database::entity::DatabaseId;
use crate::domain::property::entity::PropertyId;

/// Trait defining persistence operations for [`View`] entities.
#[allow(async_fn_in_trait)]
pub trait ViewRepository {
    /// The error type returned by this repository, which must be convertible
    /// from [`ViewError`].
    type Error: From<ViewError>;

    /// Returns the view for the given database, or `None` if no view exists.
    async fn find_by_database_id(
        &self,
        database_id: &DatabaseId,
    ) -> Result<Option<View>, Self::Error>;

    /// Persists a new view.
    async fn save(&self, view: &View) -> Result<(), Self::Error>;

    /// Updates the sort conditions for the view belonging to the given database.
    async fn update_sort_conditions(
        &self,
        database_id: &DatabaseId,
        conditions: &[SortCondition],
    ) -> Result<View, Self::Error>;

    /// Updates the filter conditions for the view belonging to the given database.
    async fn update_filter_conditions(
        &self,
        database_id: &DatabaseId,
        conditions: &[FilterCondition],
    ) -> Result<View, Self::Error>;

    /// Updates the group condition for the view belonging to the given database.
    async fn update_group_condition(
        &self,
        database_id: &DatabaseId,
        condition: Option<&GroupCondition>,
        collapsed_groups: &[String],
    ) -> Result<View, Self::Error>;

    /// Updates the collapsed groups for the view belonging to the given database.
    async fn update_collapsed_groups(
        &self,
        database_id: &DatabaseId,
        collapsed_groups: &[String],
    ) -> Result<View, Self::Error>;

    /// Resets the view for the given database to default settings.
    async fn reset(&self, database_id: &DatabaseId) -> Result<View, Self::Error>;

    /// Removes all references to the given property ID from conditions in all views.
    async fn remove_property_references(&self, property_id: &PropertyId)
    -> Result<(), Self::Error>;
}
