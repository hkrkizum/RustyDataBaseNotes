use std::cmp::Ordering;
use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::domain::property::entity::{PropertyConfig, PropertyType, SelectOption};
use crate::domain::view::entity::{SortCondition, SortDirection};

/// Row data used by the sort engine.
///
/// Each row has a page ID and a map of property values keyed by property ID string.
pub struct SortableRow<'a> {
    /// Index into the original row vector, used to maintain stable ordering.
    pub index: usize,
    /// Property values: text_value, number_value, date_value, boolean_value.
    pub values: &'a HashMap<String, RowPropertyValue>,
}

/// Extracted property value for sorting purposes.
#[derive(Debug, Clone)]
pub struct RowPropertyValue {
    /// The text value, if any.
    pub text_value: Option<String>,
    /// The number value, if any.
    pub number_value: Option<f64>,
    /// The date value, if any.
    pub date_value: Option<DateTime<Utc>>,
    /// The boolean value, if any.
    pub boolean_value: Option<bool>,
}

/// Property metadata needed for type-aware sorting.
pub struct SortPropertyInfo {
    /// The property type.
    pub property_type: PropertyType,
    /// The property config (needed for Select position ordering).
    pub config: Option<PropertyConfig>,
}

/// Applies sort conditions to produce a sorted index order.
///
/// Returns a vector of indices into the original row array, sorted according
/// to the given conditions. Uses stable sort to preserve insertion order for
/// equal elements.
///
/// # Arguments
///
/// * `row_values` - Per-row property values, keyed by property ID string.
/// * `conditions` - Sort conditions in priority order (index 0 = primary key).
/// * `property_info` - Property metadata keyed by property ID string.
///
/// # Examples
///
/// ```ignore
/// let sorted_indices = compute_sort_order(&row_values, &conditions, &property_info);
/// let sorted_rows: Vec<_> = sorted_indices.iter().map(|&i| &rows[i]).collect();
/// ```
pub fn compute_sort_order(
    row_values: &[HashMap<String, RowPropertyValue>],
    conditions: &[SortCondition],
    property_info: &HashMap<String, SortPropertyInfo>,
) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..row_values.len()).collect();

    if conditions.is_empty() {
        return indices;
    }

    indices.sort_by(|&a, &b| {
        for cond in conditions {
            let prop_id_str = cond.property_id.to_string();
            let info = property_info.get(&prop_id_str);

            let val_a = row_values[a].get(&prop_id_str);
            let val_b = row_values[b].get(&prop_id_str);

            let ordering = if let Some(info) = info {
                compare_values(val_a, val_b, info, cond.direction)
            } else {
                Ordering::Equal
            };

            if ordering != Ordering::Equal {
                return ordering;
            }
        }
        // Stable sort: preserve original order when all keys are equal
        a.cmp(&b)
    });

    indices
}

/// Compares two property values based on property type and sort direction.
///
/// Null handling: ascending = nulls last, descending = nulls first.
fn compare_values(
    a: Option<&RowPropertyValue>,
    b: Option<&RowPropertyValue>,
    info: &SortPropertyInfo,
    direction: SortDirection,
) -> Ordering {
    let raw_ordering = match info.property_type {
        PropertyType::Text => compare_text(a, b),
        PropertyType::Number => compare_number(a, b),
        PropertyType::Date => compare_date(a, b),
        PropertyType::Select => compare_select(a, b, &info.config),
        PropertyType::Checkbox => compare_checkbox(a, b),
    };

    match direction {
        SortDirection::Ascending => raw_ordering,
        SortDirection::Descending => raw_ordering.reverse(),
    }
}

/// Compares text values using Unicode codepoint order.
///
/// Empty string is treated as null. Nulls are ordered last (in ascending).
fn compare_text(a: Option<&RowPropertyValue>, b: Option<&RowPropertyValue>) -> Ordering {
    let a_val = a
        .and_then(|v| v.text_value.as_deref())
        .filter(|s| !s.is_empty());
    let b_val = b
        .and_then(|v| v.text_value.as_deref())
        .filter(|s| !s.is_empty());

    match (a_val, b_val) {
        (Some(a), Some(b)) => a.cmp(b),
        (Some(_), None) => Ordering::Less, // non-null before null
        (None, Some(_)) => Ordering::Greater, // null after non-null
        (None, None) => Ordering::Equal,
    }
}

/// Compares number values using `f64::total_cmp`.
///
/// Nulls are ordered last (in ascending).
fn compare_number(a: Option<&RowPropertyValue>, b: Option<&RowPropertyValue>) -> Ordering {
    let a_val = a.and_then(|v| v.number_value);
    let b_val = b.and_then(|v| v.number_value);

    match (a_val, b_val) {
        (Some(a), Some(b)) => a.total_cmp(&b),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

/// Compares date values using `DateTime<Utc>` comparison.
///
/// Nulls are ordered last (in ascending).
fn compare_date(a: Option<&RowPropertyValue>, b: Option<&RowPropertyValue>) -> Ordering {
    let a_val = a.and_then(|v| v.date_value);
    let b_val = b.and_then(|v| v.date_value);

    match (a_val, b_val) {
        (Some(a), Some(b)) => a.cmp(&b),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

/// Compares select values by their position in the options list.
///
/// Nulls are ordered last (in ascending).
fn compare_select(
    a: Option<&RowPropertyValue>,
    b: Option<&RowPropertyValue>,
    config: &Option<PropertyConfig>,
) -> Ordering {
    let options = match config {
        Some(PropertyConfig::Select { options }) => options,
        _ => return Ordering::Equal,
    };

    let a_val = a
        .and_then(|v| v.text_value.as_deref())
        .filter(|s| !s.is_empty());
    let b_val = b
        .and_then(|v| v.text_value.as_deref())
        .filter(|s| !s.is_empty());

    let a_pos = a_val.and_then(|id| find_option_position(options, id));
    let b_pos = b_val.and_then(|id| find_option_position(options, id));

    match (a_pos, b_pos) {
        (Some(a), Some(b)) => a.cmp(&b),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

/// Finds the position of a select option by its ID string.
fn find_option_position(options: &[SelectOption], option_id: &str) -> Option<usize> {
    options.iter().position(|o| o.id.to_string() == option_id)
}

/// Compares checkbox values: `false < true`.
///
/// Nulls are ordered last (in ascending).
fn compare_checkbox(a: Option<&RowPropertyValue>, b: Option<&RowPropertyValue>) -> Ordering {
    let a_val = a.and_then(|v| v.boolean_value);
    let b_val = b.and_then(|v| v.boolean_value);

    match (a_val, b_val) {
        (Some(a), Some(b)) => a.cmp(&b),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use crate::domain::property::entity::{DateMode, PropertyId, SelectOptionId};

    fn make_text_value(text: Option<&str>) -> RowPropertyValue {
        RowPropertyValue {
            text_value: text.map(|s| s.to_owned()),
            number_value: None,
            date_value: None,
            boolean_value: None,
        }
    }

    fn make_number_value(num: Option<f64>) -> RowPropertyValue {
        RowPropertyValue {
            text_value: None,
            number_value: num,
            date_value: None,
            boolean_value: None,
        }
    }

    fn make_date_value(date: Option<DateTime<Utc>>) -> RowPropertyValue {
        RowPropertyValue {
            text_value: None,
            number_value: None,
            date_value: date,
            boolean_value: None,
        }
    }

    fn make_checkbox_value(val: Option<bool>) -> RowPropertyValue {
        RowPropertyValue {
            text_value: None,
            number_value: None,
            date_value: None,
            boolean_value: val,
        }
    }

    fn make_select_value(option_id: Option<&str>) -> RowPropertyValue {
        RowPropertyValue {
            text_value: option_id.map(|s| s.to_owned()),
            number_value: None,
            date_value: None,
            boolean_value: None,
        }
    }

    // ---- Text sort tests ----

    #[test]
    fn sort_text_ascending_unicode_order() {
        let prop_id = PropertyId::new();
        let prop_id_str = prop_id.to_string();

        let rows = vec![
            HashMap::from([(prop_id_str.clone(), make_text_value(Some("Charlie")))]),
            HashMap::from([(prop_id_str.clone(), make_text_value(Some("Alpha")))]),
            HashMap::from([(prop_id_str.clone(), make_text_value(Some("Bravo")))]),
        ];

        let conditions = vec![SortCondition {
            property_id: prop_id.clone(),
            direction: SortDirection::Ascending,
        }];
        let info = HashMap::from([(
            prop_id_str,
            SortPropertyInfo {
                property_type: PropertyType::Text,
                config: None,
            },
        )]);

        let order = compute_sort_order(&rows, &conditions, &info);
        assert_eq!(order, vec![1, 2, 0]); // Alpha, Bravo, Charlie
    }

    #[test]
    fn sort_text_descending() {
        let prop_id = PropertyId::new();
        let prop_id_str = prop_id.to_string();

        let rows = vec![
            HashMap::from([(prop_id_str.clone(), make_text_value(Some("Alpha")))]),
            HashMap::from([(prop_id_str.clone(), make_text_value(Some("Charlie")))]),
            HashMap::from([(prop_id_str.clone(), make_text_value(Some("Bravo")))]),
        ];

        let conditions = vec![SortCondition {
            property_id: prop_id.clone(),
            direction: SortDirection::Descending,
        }];
        let info = HashMap::from([(
            prop_id_str,
            SortPropertyInfo {
                property_type: PropertyType::Text,
                config: None,
            },
        )]);

        let order = compute_sort_order(&rows, &conditions, &info);
        assert_eq!(order, vec![1, 2, 0]); // Charlie, Bravo, Alpha
    }

    // ---- Number sort tests ----

    #[test]
    fn sort_number_ascending_f64_total_cmp() {
        let prop_id = PropertyId::new();
        let prop_id_str = prop_id.to_string();

        let rows = vec![
            HashMap::from([(prop_id_str.clone(), make_number_value(Some(30.0)))]),
            HashMap::from([(prop_id_str.clone(), make_number_value(Some(1.0)))]),
            HashMap::from([(prop_id_str.clone(), make_number_value(Some(20.0)))]),
        ];

        let conditions = vec![SortCondition {
            property_id: prop_id.clone(),
            direction: SortDirection::Ascending,
        }];
        let info = HashMap::from([(
            prop_id_str,
            SortPropertyInfo {
                property_type: PropertyType::Number,
                config: None,
            },
        )]);

        let order = compute_sort_order(&rows, &conditions, &info);
        assert_eq!(order, vec![1, 2, 0]); // 1.0, 20.0, 30.0
    }

    // ---- Date sort tests ----

    #[test]
    fn sort_date_ascending() {
        let prop_id = PropertyId::new();
        let prop_id_str = prop_id.to_string();

        let d1: DateTime<Utc> = "2024-01-01T00:00:00Z".parse().expect("valid");
        let d2: DateTime<Utc> = "2024-06-15T12:00:00Z".parse().expect("valid");
        let d3: DateTime<Utc> = "2024-03-10T08:30:00Z".parse().expect("valid");

        let rows = vec![
            HashMap::from([(prop_id_str.clone(), make_date_value(Some(d2)))]),
            HashMap::from([(prop_id_str.clone(), make_date_value(Some(d1)))]),
            HashMap::from([(prop_id_str.clone(), make_date_value(Some(d3)))]),
        ];

        let conditions = vec![SortCondition {
            property_id: prop_id.clone(),
            direction: SortDirection::Ascending,
        }];
        let info = HashMap::from([(
            prop_id_str,
            SortPropertyInfo {
                property_type: PropertyType::Date,
                config: Some(PropertyConfig::Date {
                    mode: DateMode::DateTime,
                }),
            },
        )]);

        let order = compute_sort_order(&rows, &conditions, &info);
        assert_eq!(order, vec![1, 2, 0]); // d1, d3, d2
    }

    // ---- Select sort tests ----

    #[test]
    fn sort_select_by_position_order() {
        let prop_id = PropertyId::new();
        let prop_id_str = prop_id.to_string();

        let opt_a = SelectOptionId::new();
        let opt_b = SelectOptionId::new();
        let opt_c = SelectOptionId::new();

        let config = PropertyConfig::Select {
            options: vec![
                SelectOption {
                    id: opt_a.clone(),
                    value: "Alpha".to_owned(),
                },
                SelectOption {
                    id: opt_b.clone(),
                    value: "Bravo".to_owned(),
                },
                SelectOption {
                    id: opt_c.clone(),
                    value: "Charlie".to_owned(),
                },
            ],
        };

        let rows = vec![
            HashMap::from([(
                prop_id_str.clone(),
                make_select_value(Some(&opt_c.to_string())),
            )]),
            HashMap::from([(
                prop_id_str.clone(),
                make_select_value(Some(&opt_a.to_string())),
            )]),
            HashMap::from([(
                prop_id_str.clone(),
                make_select_value(Some(&opt_b.to_string())),
            )]),
        ];

        let conditions = vec![SortCondition {
            property_id: prop_id.clone(),
            direction: SortDirection::Ascending,
        }];
        let info = HashMap::from([(
            prop_id_str,
            SortPropertyInfo {
                property_type: PropertyType::Select,
                config: Some(config),
            },
        )]);

        let order = compute_sort_order(&rows, &conditions, &info);
        assert_eq!(order, vec![1, 2, 0]); // Alpha, Bravo, Charlie (by position)
    }

    // ---- Checkbox sort tests ----

    #[test]
    fn sort_checkbox_false_before_true() {
        let prop_id = PropertyId::new();
        let prop_id_str = prop_id.to_string();

        let rows = vec![
            HashMap::from([(prop_id_str.clone(), make_checkbox_value(Some(true)))]),
            HashMap::from([(prop_id_str.clone(), make_checkbox_value(Some(false)))]),
            HashMap::from([(prop_id_str.clone(), make_checkbox_value(Some(true)))]),
        ];

        let conditions = vec![SortCondition {
            property_id: prop_id.clone(),
            direction: SortDirection::Ascending,
        }];
        let info = HashMap::from([(
            prop_id_str,
            SortPropertyInfo {
                property_type: PropertyType::Checkbox,
                config: None,
            },
        )]);

        let order = compute_sort_order(&rows, &conditions, &info);
        assert_eq!(order[0], 1); // false first
    }

    // ---- Null handling tests ----

    #[test]
    fn sort_nulls_last_ascending() {
        let prop_id = PropertyId::new();
        let prop_id_str = prop_id.to_string();

        let rows = vec![
            HashMap::from([(prop_id_str.clone(), make_text_value(None))]),
            HashMap::from([(prop_id_str.clone(), make_text_value(Some("Alpha")))]),
            HashMap::from([(prop_id_str.clone(), make_text_value(Some("Bravo")))]),
        ];

        let conditions = vec![SortCondition {
            property_id: prop_id.clone(),
            direction: SortDirection::Ascending,
        }];
        let info = HashMap::from([(
            prop_id_str,
            SortPropertyInfo {
                property_type: PropertyType::Text,
                config: None,
            },
        )]);

        let order = compute_sort_order(&rows, &conditions, &info);
        // Alpha, Bravo, null
        assert_eq!(order, vec![1, 2, 0]);
    }

    #[test]
    fn sort_nulls_first_descending() {
        let prop_id = PropertyId::new();
        let prop_id_str = prop_id.to_string();

        let rows = vec![
            HashMap::from([(prop_id_str.clone(), make_text_value(Some("Alpha")))]),
            HashMap::from([(prop_id_str.clone(), make_text_value(None))]),
            HashMap::from([(prop_id_str.clone(), make_text_value(Some("Bravo")))]),
        ];

        let conditions = vec![SortCondition {
            property_id: prop_id.clone(),
            direction: SortDirection::Descending,
        }];
        let info = HashMap::from([(
            prop_id_str,
            SortPropertyInfo {
                property_type: PropertyType::Text,
                config: None,
            },
        )]);

        let order = compute_sort_order(&rows, &conditions, &info);
        // null, Bravo, Alpha (descending reverses: null=last→first)
        assert_eq!(order, vec![1, 2, 0]);
    }

    #[test]
    fn sort_empty_string_treated_as_null() {
        let prop_id = PropertyId::new();
        let prop_id_str = prop_id.to_string();

        let rows = vec![
            HashMap::from([(prop_id_str.clone(), make_text_value(Some("")))]),
            HashMap::from([(prop_id_str.clone(), make_text_value(Some("Alpha")))]),
        ];

        let conditions = vec![SortCondition {
            property_id: prop_id.clone(),
            direction: SortDirection::Ascending,
        }];
        let info = HashMap::from([(
            prop_id_str,
            SortPropertyInfo {
                property_type: PropertyType::Text,
                config: None,
            },
        )]);

        let order = compute_sort_order(&rows, &conditions, &info);
        assert_eq!(order, vec![1, 0]); // Alpha, empty (treated as null, last)
    }

    // ---- Stable sort test ----

    #[test]
    fn sort_preserves_insertion_order_for_equal_values() {
        let prop_id = PropertyId::new();
        let prop_id_str = prop_id.to_string();

        let rows = vec![
            HashMap::from([(prop_id_str.clone(), make_text_value(Some("Same")))]),
            HashMap::from([(prop_id_str.clone(), make_text_value(Some("Same")))]),
            HashMap::from([(prop_id_str.clone(), make_text_value(Some("Same")))]),
        ];

        let conditions = vec![SortCondition {
            property_id: prop_id.clone(),
            direction: SortDirection::Ascending,
        }];
        let info = HashMap::from([(
            prop_id_str,
            SortPropertyInfo {
                property_type: PropertyType::Text,
                config: None,
            },
        )]);

        let order = compute_sort_order(&rows, &conditions, &info);
        assert_eq!(order, vec![0, 1, 2]); // Stable: original order preserved
    }

    // ---- Multi-column sort test ----

    #[test]
    fn sort_multi_key_primary_tie_broken_by_secondary() {
        let prop1 = PropertyId::new();
        let prop1_str = prop1.to_string();
        let prop2 = PropertyId::new();
        let prop2_str = prop2.to_string();

        let rows = vec![
            HashMap::from([
                (prop1_str.clone(), make_text_value(Some("A"))),
                (prop2_str.clone(), make_number_value(Some(3.0))),
            ]),
            HashMap::from([
                (prop1_str.clone(), make_text_value(Some("A"))),
                (prop2_str.clone(), make_number_value(Some(1.0))),
            ]),
            HashMap::from([
                (prop1_str.clone(), make_text_value(Some("B"))),
                (prop2_str.clone(), make_number_value(Some(2.0))),
            ]),
        ];

        let conditions = vec![
            SortCondition {
                property_id: prop1.clone(),
                direction: SortDirection::Ascending,
            },
            SortCondition {
                property_id: prop2.clone(),
                direction: SortDirection::Ascending,
            },
        ];
        let info = HashMap::from([
            (
                prop1_str,
                SortPropertyInfo {
                    property_type: PropertyType::Text,
                    config: None,
                },
            ),
            (
                prop2_str,
                SortPropertyInfo {
                    property_type: PropertyType::Number,
                    config: None,
                },
            ),
        ]);

        let order = compute_sort_order(&rows, &conditions, &info);
        // A(1.0), A(3.0), B(2.0) — primary key ties broken by secondary
        assert_eq!(order, vec![1, 0, 2]);
    }

    #[test]
    fn sort_empty_conditions_preserves_order() {
        let rows = vec![HashMap::new(), HashMap::new(), HashMap::new()];
        let order = compute_sort_order(&rows, &[], &HashMap::new());
        assert_eq!(order, vec![0, 1, 2]);
    }

    #[test]
    fn sort_three_plus_conditions_chaining() {
        let p1 = PropertyId::new();
        let p2 = PropertyId::new();
        let p3 = PropertyId::new();
        let p1s = p1.to_string();
        let p2s = p2.to_string();
        let p3s = p3.to_string();

        let rows = vec![
            HashMap::from([
                (p1s.clone(), make_text_value(Some("A"))),
                (p2s.clone(), make_number_value(Some(1.0))),
                (p3s.clone(), make_checkbox_value(Some(true))),
            ]),
            HashMap::from([
                (p1s.clone(), make_text_value(Some("A"))),
                (p2s.clone(), make_number_value(Some(1.0))),
                (p3s.clone(), make_checkbox_value(Some(false))),
            ]),
            HashMap::from([
                (p1s.clone(), make_text_value(Some("A"))),
                (p2s.clone(), make_number_value(Some(2.0))),
                (p3s.clone(), make_checkbox_value(Some(true))),
            ]),
        ];

        let conditions = vec![
            SortCondition {
                property_id: p1.clone(),
                direction: SortDirection::Ascending,
            },
            SortCondition {
                property_id: p2.clone(),
                direction: SortDirection::Ascending,
            },
            SortCondition {
                property_id: p3.clone(),
                direction: SortDirection::Ascending,
            },
        ];
        let info = HashMap::from([
            (
                p1s,
                SortPropertyInfo {
                    property_type: PropertyType::Text,
                    config: None,
                },
            ),
            (
                p2s,
                SortPropertyInfo {
                    property_type: PropertyType::Number,
                    config: None,
                },
            ),
            (
                p3s,
                SortPropertyInfo {
                    property_type: PropertyType::Checkbox,
                    config: None,
                },
            ),
        ]);

        let order = compute_sort_order(&rows, &conditions, &info);
        // All same text "A"; then sorted by number 1.0, 1.0, 2.0;
        // For 1.0 tie, sorted by checkbox false < true
        assert_eq!(order, vec![1, 0, 2]);
    }
}
