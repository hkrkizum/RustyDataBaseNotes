use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, Utc};

use crate::domain::property::entity::PropertyType;
use crate::domain::view::entity::{FilterCondition, FilterOperator, FilterValue};
use crate::domain::view::sort::RowPropertyValue;

/// Applies filter conditions to rows and returns indices of matching rows.
///
/// All conditions are combined with AND logic — a row must match every
/// condition to be included in the result.
///
/// # Arguments
///
/// * `row_values` - Per-row property values, keyed by property ID string.
/// * `conditions` - Filter conditions to apply (AND combination).
/// * `property_types` - Property types keyed by property ID string.
///
/// # Examples
///
/// ```ignore
/// let matching = apply_filters(&row_values, &conditions, &property_types);
/// let filtered_rows: Vec<_> = matching.iter().map(|&i| &rows[i]).collect();
/// ```
pub fn apply_filters(
    row_values: &[HashMap<String, RowPropertyValue>],
    conditions: &[FilterCondition],
    property_types: &HashMap<String, PropertyType>,
) -> Vec<usize> {
    if conditions.is_empty() {
        return (0..row_values.len()).collect();
    }

    (0..row_values.len())
        .filter(|&i| {
            conditions.iter().all(|cond| {
                let prop_id_str = cond.property_id.to_string();
                let val = row_values[i].get(&prop_id_str);
                let prop_type = property_types.get(&prop_id_str).copied();
                matches_condition(val, cond, prop_type)
            })
        })
        .collect()
}

/// Checks if a single row value matches a filter condition.
fn matches_condition(
    row_val: Option<&RowPropertyValue>,
    condition: &FilterCondition,
    prop_type: Option<PropertyType>,
) -> bool {
    match condition.operator {
        FilterOperator::IsEmpty => is_empty(row_val, prop_type),
        FilterOperator::IsNotEmpty => !is_empty(row_val, prop_type),
        FilterOperator::IsChecked => row_val.and_then(|v| v.boolean_value).unwrap_or(false),
        FilterOperator::IsUnchecked => !row_val.and_then(|v| v.boolean_value).unwrap_or(false),
        _ => match prop_type {
            Some(PropertyType::Text) => match_text(row_val, condition),
            Some(PropertyType::Number) => match_number(row_val, condition),
            Some(PropertyType::Date) => match_date(row_val, condition),
            Some(PropertyType::Select) => match_select(row_val, condition),
            _ => true, // Unknown type: don't filter
        },
    }
}

/// Checks if a row value is "empty" (null).
///
/// Only null values are considered empty — zero, empty string, and false
/// are NOT considered empty per spec.
fn is_empty(row_val: Option<&RowPropertyValue>, prop_type: Option<PropertyType>) -> bool {
    let Some(val) = row_val else {
        return true;
    };

    match prop_type {
        Some(PropertyType::Text | PropertyType::Select) => val.text_value.is_none(),
        Some(PropertyType::Number) => val.number_value.is_none(),
        Some(PropertyType::Date) => val.date_value.is_none(),
        Some(PropertyType::Checkbox) => val.boolean_value.is_none(),
        None => true,
    }
}

/// Text filter matching (case-insensitive).
fn match_text(row_val: Option<&RowPropertyValue>, condition: &FilterCondition) -> bool {
    let text = row_val.and_then(|v| v.text_value.as_deref());

    let filter_text = match &condition.value {
        Some(FilterValue::Text(s)) => s.as_str(),
        _ => return false,
    };

    let Some(text) = text else {
        return false; // null doesn't match text conditions
    };

    let text_lower = text.to_lowercase();
    let filter_lower = filter_text.to_lowercase();

    match condition.operator {
        FilterOperator::Equals => text_lower == filter_lower,
        FilterOperator::NotEquals => text_lower != filter_lower,
        FilterOperator::Contains => text_lower.contains(&filter_lower),
        FilterOperator::NotContains => !text_lower.contains(&filter_lower),
        _ => true,
    }
}

/// Number filter matching with f64 equality.
fn match_number(row_val: Option<&RowPropertyValue>, condition: &FilterCondition) -> bool {
    let num = row_val.and_then(|v| v.number_value);

    let filter_num = match &condition.value {
        Some(FilterValue::Number(n)) => *n,
        _ => return false,
    };

    let Some(num) = num else {
        return false;
    };

    match condition.operator {
        FilterOperator::Equals => num == filter_num,
        FilterOperator::NotEquals => num != filter_num,
        FilterOperator::GreaterThan => num > filter_num,
        FilterOperator::LessThan => num < filter_num,
        FilterOperator::GreaterOrEqual => num >= filter_num,
        FilterOperator::LessOrEqual => num <= filter_num,
        _ => true,
    }
}

/// Date filter matching.
///
/// - Equals: minute-granularity comparison.
/// - Before: exclusive (row_date < filter_date).
/// - After: inclusive (row_date >= filter_date).
fn match_date(row_val: Option<&RowPropertyValue>, condition: &FilterCondition) -> bool {
    let date = row_val.and_then(|v| v.date_value);

    let filter_date_str = match &condition.value {
        Some(FilterValue::Date(d)) => d.as_str(),
        _ => return false,
    };

    let Some(date) = date else {
        return false;
    };

    // Parse filter date — try full datetime first, then date-only
    let filter_date: DateTime<Utc> = if let Ok(dt) = filter_date_str.parse::<DateTime<Utc>>() {
        dt
    } else if let Ok(nd) = filter_date_str.parse::<NaiveDate>() {
        nd.and_hms_opt(0, 0, 0)
            .map(|ndt| ndt.and_utc())
            .unwrap_or(date) // fallback: use row date (will equal itself)
    } else {
        return false;
    };

    match condition.operator {
        FilterOperator::Equals => {
            // Minute granularity
            date.format("%Y-%m-%d %H:%M").to_string()
                == filter_date.format("%Y-%m-%d %H:%M").to_string()
        }
        FilterOperator::Before => date < filter_date,
        FilterOperator::After => date >= filter_date,
        _ => true,
    }
}

/// Select filter matching.
fn match_select(row_val: Option<&RowPropertyValue>, condition: &FilterCondition) -> bool {
    let option_id = row_val.and_then(|v| v.text_value.as_deref());

    let filter_option = match &condition.value {
        Some(FilterValue::SelectOption(s)) => s.as_str(),
        _ => return false,
    };

    match condition.operator {
        FilterOperator::Is => option_id == Some(filter_option),
        FilterOperator::IsNot => option_id != Some(filter_option),
        _ => true,
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use crate::domain::property::entity::PropertyId;

    fn make_text(text: Option<&str>) -> RowPropertyValue {
        RowPropertyValue {
            text_value: text.map(|s| s.to_owned()),
            number_value: None,
            date_value: None,
            boolean_value: None,
        }
    }

    fn make_number(num: Option<f64>) -> RowPropertyValue {
        RowPropertyValue {
            text_value: None,
            number_value: num,
            date_value: None,
            boolean_value: None,
        }
    }

    fn make_date(date: Option<DateTime<Utc>>) -> RowPropertyValue {
        RowPropertyValue {
            text_value: None,
            number_value: None,
            date_value: date,
            boolean_value: None,
        }
    }

    fn make_checkbox(val: Option<bool>) -> RowPropertyValue {
        RowPropertyValue {
            text_value: None,
            number_value: None,
            date_value: None,
            boolean_value: val,
        }
    }

    fn make_select(option_id: Option<&str>) -> RowPropertyValue {
        RowPropertyValue {
            text_value: option_id.map(|s| s.to_owned()),
            number_value: None,
            date_value: None,
            boolean_value: None,
        }
    }

    // ---- Text filter tests ----

    #[test]
    fn filter_text_equals_case_insensitive() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let rows = vec![
            HashMap::from([(pids.clone(), make_text(Some("Hello")))]),
            HashMap::from([(pids.clone(), make_text(Some("hello")))]),
            HashMap::from([(pids.clone(), make_text(Some("World")))]),
        ];
        let conditions = vec![FilterCondition {
            property_id: pid,
            operator: FilterOperator::Equals,
            value: Some(FilterValue::Text("HELLO".to_owned())),
        }];
        let types = HashMap::from([(pids, PropertyType::Text)]);
        let result = apply_filters(&rows, &conditions, &types);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn filter_text_not_equals() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let rows = vec![
            HashMap::from([(pids.clone(), make_text(Some("Hello")))]),
            HashMap::from([(pids.clone(), make_text(Some("World")))]),
        ];
        let conditions = vec![FilterCondition {
            property_id: pid,
            operator: FilterOperator::NotEquals,
            value: Some(FilterValue::Text("Hello".to_owned())),
        }];
        let types = HashMap::from([(pids, PropertyType::Text)]);
        let result = apply_filters(&rows, &conditions, &types);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn filter_text_contains_case_insensitive() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let rows = vec![
            HashMap::from([(pids.clone(), make_text(Some("Hello World")))]),
            HashMap::from([(pids.clone(), make_text(Some("Goodbye")))]),
        ];
        let conditions = vec![FilterCondition {
            property_id: pid,
            operator: FilterOperator::Contains,
            value: Some(FilterValue::Text("hello".to_owned())),
        }];
        let types = HashMap::from([(pids, PropertyType::Text)]);
        let result = apply_filters(&rows, &conditions, &types);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn filter_text_not_contains() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let rows = vec![
            HashMap::from([(pids.clone(), make_text(Some("Hello World")))]),
            HashMap::from([(pids.clone(), make_text(Some("Goodbye")))]),
        ];
        let conditions = vec![FilterCondition {
            property_id: pid,
            operator: FilterOperator::NotContains,
            value: Some(FilterValue::Text("hello".to_owned())),
        }];
        let types = HashMap::from([(pids, PropertyType::Text)]);
        let result = apply_filters(&rows, &conditions, &types);
        assert_eq!(result, vec![1]);
    }

    // ---- Number filter tests ----

    #[test]
    fn filter_number_equals() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let rows = vec![
            HashMap::from([(pids.clone(), make_number(Some(42.0)))]),
            HashMap::from([(pids.clone(), make_number(Some(10.0)))]),
        ];
        let conditions = vec![FilterCondition {
            property_id: pid,
            operator: FilterOperator::Equals,
            value: Some(FilterValue::Number(42.0)),
        }];
        let types = HashMap::from([(pids, PropertyType::Number)]);
        let result = apply_filters(&rows, &conditions, &types);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn filter_number_greater_than() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let rows = vec![
            HashMap::from([(pids.clone(), make_number(Some(10.0)))]),
            HashMap::from([(pids.clone(), make_number(Some(20.0)))]),
            HashMap::from([(pids.clone(), make_number(Some(30.0)))]),
        ];
        let conditions = vec![FilterCondition {
            property_id: pid,
            operator: FilterOperator::GreaterThan,
            value: Some(FilterValue::Number(15.0)),
        }];
        let types = HashMap::from([(pids, PropertyType::Number)]);
        let result = apply_filters(&rows, &conditions, &types);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn filter_number_less_or_equal() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let rows = vec![
            HashMap::from([(pids.clone(), make_number(Some(10.0)))]),
            HashMap::from([(pids.clone(), make_number(Some(20.0)))]),
            HashMap::from([(pids.clone(), make_number(Some(30.0)))]),
        ];
        let conditions = vec![FilterCondition {
            property_id: pid,
            operator: FilterOperator::LessOrEqual,
            value: Some(FilterValue::Number(20.0)),
        }];
        let types = HashMap::from([(pids, PropertyType::Number)]);
        let result = apply_filters(&rows, &conditions, &types);
        assert_eq!(result, vec![0, 1]);
    }

    // ---- Date filter tests ----

    #[test]
    fn filter_date_before_exclusive() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let d1: DateTime<Utc> = "2024-01-15T12:00:00Z".parse().expect("valid");
        let d2: DateTime<Utc> = "2024-06-15T12:00:00Z".parse().expect("valid");
        let rows = vec![
            HashMap::from([(pids.clone(), make_date(Some(d1)))]),
            HashMap::from([(pids.clone(), make_date(Some(d2)))]),
        ];
        let conditions = vec![FilterCondition {
            property_id: pid,
            operator: FilterOperator::Before,
            value: Some(FilterValue::Date("2024-03-01T00:00:00Z".to_owned())),
        }];
        let types = HashMap::from([(pids, PropertyType::Date)]);
        let result = apply_filters(&rows, &conditions, &types);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn filter_date_after_inclusive() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let d1: DateTime<Utc> = "2024-03-01T00:00:00Z".parse().expect("valid");
        let d2: DateTime<Utc> = "2024-06-15T12:00:00Z".parse().expect("valid");
        let rows = vec![
            HashMap::from([(pids.clone(), make_date(Some(d1)))]),
            HashMap::from([(pids.clone(), make_date(Some(d2)))]),
        ];
        let conditions = vec![FilterCondition {
            property_id: pid,
            operator: FilterOperator::After,
            value: Some(FilterValue::Date("2024-03-01T00:00:00Z".to_owned())),
        }];
        let types = HashMap::from([(pids, PropertyType::Date)]);
        let result = apply_filters(&rows, &conditions, &types);
        // Both d1 (==) and d2 (>) match (After is inclusive)
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn filter_date_equals_minute_granularity() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let d1: DateTime<Utc> = "2024-03-01T12:30:00Z".parse().expect("valid");
        let d2: DateTime<Utc> = "2024-03-01T12:30:45Z".parse().expect("valid");
        let d3: DateTime<Utc> = "2024-03-01T12:31:00Z".parse().expect("valid");
        let rows = vec![
            HashMap::from([(pids.clone(), make_date(Some(d1)))]),
            HashMap::from([(pids.clone(), make_date(Some(d2)))]),
            HashMap::from([(pids.clone(), make_date(Some(d3)))]),
        ];
        let conditions = vec![FilterCondition {
            property_id: pid,
            operator: FilterOperator::Equals,
            value: Some(FilterValue::Date("2024-03-01T12:30:00Z".to_owned())),
        }];
        let types = HashMap::from([(pids, PropertyType::Date)]);
        let result = apply_filters(&rows, &conditions, &types);
        // d1 and d2 match (same minute), d3 doesn't
        assert_eq!(result, vec![0, 1]);
    }

    // ---- Select filter tests ----

    #[test]
    fn filter_select_is() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let rows = vec![
            HashMap::from([(pids.clone(), make_select(Some("opt-a")))]),
            HashMap::from([(pids.clone(), make_select(Some("opt-b")))]),
            HashMap::from([(pids.clone(), make_select(None))]),
        ];
        let conditions = vec![FilterCondition {
            property_id: pid,
            operator: FilterOperator::Is,
            value: Some(FilterValue::SelectOption("opt-a".to_owned())),
        }];
        let types = HashMap::from([(pids, PropertyType::Select)]);
        let result = apply_filters(&rows, &conditions, &types);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn filter_select_is_not_includes_null() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let rows = vec![
            HashMap::from([(pids.clone(), make_select(Some("opt-a")))]),
            HashMap::from([(pids.clone(), make_select(Some("opt-b")))]),
            HashMap::from([(pids.clone(), make_select(None))]),
        ];
        let conditions = vec![FilterCondition {
            property_id: pid,
            operator: FilterOperator::IsNot,
            value: Some(FilterValue::SelectOption("opt-a".to_owned())),
        }];
        let types = HashMap::from([(pids, PropertyType::Select)]);
        let result = apply_filters(&rows, &conditions, &types);
        // opt-b and null match IsNot
        assert_eq!(result, vec![1, 2]);
    }

    // ---- Checkbox filter tests ----

    #[test]
    fn filter_checkbox_is_checked() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let rows = vec![
            HashMap::from([(pids.clone(), make_checkbox(Some(true)))]),
            HashMap::from([(pids.clone(), make_checkbox(Some(false)))]),
            HashMap::from([(pids.clone(), make_checkbox(None))]),
        ];
        let conditions = vec![FilterCondition {
            property_id: pid,
            operator: FilterOperator::IsChecked,
            value: None,
        }];
        let types = HashMap::from([(pids, PropertyType::Checkbox)]);
        let result = apply_filters(&rows, &conditions, &types);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn filter_checkbox_is_unchecked() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let rows = vec![
            HashMap::from([(pids.clone(), make_checkbox(Some(true)))]),
            HashMap::from([(pids.clone(), make_checkbox(Some(false)))]),
            HashMap::from([(pids.clone(), make_checkbox(None))]),
        ];
        let conditions = vec![FilterCondition {
            property_id: pid,
            operator: FilterOperator::IsUnchecked,
            value: None,
        }];
        let types = HashMap::from([(pids, PropertyType::Checkbox)]);
        let result = apply_filters(&rows, &conditions, &types);
        assert_eq!(result, vec![1, 2]);
    }

    // ---- IsEmpty / IsNotEmpty tests ----

    #[test]
    fn filter_is_empty_null_only() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let rows = vec![
            HashMap::from([(pids.clone(), make_number(Some(0.0)))]),
            HashMap::from([(pids.clone(), make_number(None))]),
            HashMap::from([(pids.clone(), make_number(Some(42.0)))]),
        ];
        let conditions = vec![FilterCondition {
            property_id: pid,
            operator: FilterOperator::IsEmpty,
            value: None,
        }];
        let types = HashMap::from([(pids, PropertyType::Number)]);
        let result = apply_filters(&rows, &conditions, &types);
        // Only null is empty, not zero
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn filter_is_not_empty() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let rows = vec![
            HashMap::from([(pids.clone(), make_text(Some("hello")))]),
            HashMap::from([(pids.clone(), make_text(None))]),
        ];
        let conditions = vec![FilterCondition {
            property_id: pid,
            operator: FilterOperator::IsNotEmpty,
            value: None,
        }];
        let types = HashMap::from([(pids, PropertyType::Text)]);
        let result = apply_filters(&rows, &conditions, &types);
        assert_eq!(result, vec![0]);
    }

    // ---- Multiple conditions AND tests ----

    #[test]
    fn filter_multiple_conditions_and() {
        let p1 = PropertyId::new();
        let p1s = p1.to_string();
        let p2 = PropertyId::new();
        let p2s = p2.to_string();

        let rows = vec![
            HashMap::from([
                (p1s.clone(), make_text(Some("Alice"))),
                (p2s.clone(), make_number(Some(25.0))),
            ]),
            HashMap::from([
                (p1s.clone(), make_text(Some("Bob"))),
                (p2s.clone(), make_number(Some(30.0))),
            ]),
            HashMap::from([
                (p1s.clone(), make_text(Some("Alice"))),
                (p2s.clone(), make_number(Some(35.0))),
            ]),
        ];

        let conditions = vec![
            FilterCondition {
                property_id: p1,
                operator: FilterOperator::Equals,
                value: Some(FilterValue::Text("Alice".to_owned())),
            },
            FilterCondition {
                property_id: p2,
                operator: FilterOperator::GreaterThan,
                value: Some(FilterValue::Number(30.0)),
            },
        ];

        let types = HashMap::from([(p1s, PropertyType::Text), (p2s, PropertyType::Number)]);

        let result = apply_filters(&rows, &conditions, &types);
        // Only Alice with age > 30
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn filter_empty_conditions_returns_all() {
        let rows = vec![HashMap::new(), HashMap::new()];
        let result = apply_filters(&rows, &[], &HashMap::new());
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn filter_null_row_doesnt_match_text_condition() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let rows = vec![
            HashMap::from([(pids.clone(), make_text(None))]),
            HashMap::from([(pids.clone(), make_text(Some("hello")))]),
        ];
        let conditions = vec![FilterCondition {
            property_id: pid,
            operator: FilterOperator::Contains,
            value: Some(FilterValue::Text("hel".to_owned())),
        }];
        let types = HashMap::from([(pids, PropertyType::Text)]);
        let result = apply_filters(&rows, &conditions, &types);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn filter_select_is_matches_by_option_id_not_display_value() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let option_uuid = "550e8400-e29b-41d4-a716-446655440000";
        let rows = vec![HashMap::from([(
            pids.clone(),
            make_select(Some(option_uuid)),
        )])];
        let types = HashMap::from([(pids.clone(), PropertyType::Select)]);

        // option ID で検索 → マッチ
        let cond_by_id = vec![FilterCondition {
            property_id: pid.clone(),
            operator: FilterOperator::Is,
            value: Some(FilterValue::SelectOption(option_uuid.to_owned())),
        }];
        assert_eq!(apply_filters(&rows, &cond_by_id, &types), vec![0]);

        // 表示値で検索 → 不一致（フロントは UUID を送る必要がある）
        let cond_by_display = vec![FilterCondition {
            property_id: pid,
            operator: FilterOperator::Is,
            value: Some(FilterValue::SelectOption("1".to_owned())),
        }];
        let empty: Vec<usize> = vec![];
        assert_eq!(apply_filters(&rows, &cond_by_display, &types), empty);
    }
}
