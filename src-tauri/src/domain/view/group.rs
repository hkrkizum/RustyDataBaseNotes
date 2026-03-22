use std::collections::{HashMap, HashSet};

use crate::domain::property::entity::{PropertyConfig, PropertyType};
use crate::domain::view::entity::GroupCondition;
use crate::domain::view::sort::RowPropertyValue;

/// Information about a single group for the frontend.
#[derive(Debug, Clone)]
pub struct GroupInfo {
    /// The group value (None = 未設定 group).
    pub value: Option<String>,
    /// Display label for the group.
    pub display_value: String,
    /// Number of rows in this group.
    pub count: usize,
    /// Whether this group is collapsed.
    pub is_collapsed: bool,
}

/// Computes group information and returns groups with row indices.
///
/// Returns `(groups, grouped_row_indices)` where grouped_row_indices
/// maps each group to its row indices (in the original row order).
///
/// # Arguments
///
/// * `row_values` - Per-row property values, keyed by property ID string.
/// * `condition` - The group condition (property to group by).
/// * `property_type` - The type of the grouping property.
/// * `config` - The config of the grouping property (for Select ordering).
/// * `collapsed_groups` - Set of collapsed group values (empty string = null group).
pub fn compute_groups(
    row_values: &[HashMap<String, RowPropertyValue>],
    condition: &GroupCondition,
    property_type: PropertyType,
    config: Option<&PropertyConfig>,
    collapsed_groups: &HashSet<String>,
) -> (Vec<GroupInfo>, Vec<Vec<usize>>) {
    let prop_id_str = condition.property_id.to_string();

    // Collect group keys and row indices
    let mut group_map: HashMap<String, Vec<usize>> = HashMap::new();
    let mut group_keys_order: Vec<String> = Vec::new();

    for (i, row) in row_values.iter().enumerate() {
        let key = extract_group_key(row.get(&prop_id_str), property_type);
        if !group_map.contains_key(&key) {
            group_keys_order.push(key.clone());
        }
        group_map.entry(key).or_default().push(i);
    }

    // Sort group keys by type-specific ordering
    sort_group_keys(&mut group_keys_order, property_type, config);

    // Move null group ("") to the end
    if let Some(pos) = group_keys_order.iter().position(|k| k.is_empty()) {
        let null_key = group_keys_order.remove(pos);
        group_keys_order.push(null_key);
    }

    // Build GroupInfo and row index lists
    let mut groups = Vec::with_capacity(group_keys_order.len());
    let mut grouped_rows = Vec::with_capacity(group_keys_order.len());

    for key in &group_keys_order {
        let indices = group_map.get(key).cloned().unwrap_or_default();
        if indices.is_empty() {
            continue;
        }

        let is_null = key.is_empty();
        let display_value = if is_null {
            "未設定".to_owned()
        } else {
            get_display_value(key, property_type, config)
        };

        let is_collapsed = collapsed_groups.contains(key);

        groups.push(GroupInfo {
            value: if is_null { None } else { Some(key.clone()) },
            display_value,
            count: indices.len(),
            is_collapsed,
        });

        grouped_rows.push(indices);
    }

    (groups, grouped_rows)
}

/// Extracts a group key string from a row property value.
///
/// Returns empty string for null values.
fn extract_group_key(val: Option<&RowPropertyValue>, property_type: PropertyType) -> String {
    let Some(val) = val else {
        return String::new();
    };

    match property_type {
        PropertyType::Text => val
            .text_value
            .as_ref()
            .map(|s| s.to_lowercase())
            .unwrap_or_default(),
        PropertyType::Number => val.number_value.map(|n| format!("{n}")).unwrap_or_default(),
        PropertyType::Date => val
            .date_value
            .map(|d| d.format("%Y-%m-%d").to_string())
            .unwrap_or_default(),
        PropertyType::Select => val.text_value.clone().unwrap_or_default(),
        PropertyType::Checkbox => val.boolean_value.map(|b| b.to_string()).unwrap_or_default(),
    }
}

/// Sorts group keys by type-specific ordering.
fn sort_group_keys(
    keys: &mut [String],
    property_type: PropertyType,
    config: Option<&PropertyConfig>,
) {
    match property_type {
        PropertyType::Select => {
            if let Some(PropertyConfig::Select { options }) = config {
                let position_map: HashMap<&str, usize> = options
                    .iter()
                    .enumerate()
                    .map(|(i, o)| (o.id.to_string().leak() as &str, i))
                    .collect();
                keys.sort_by_key(|k| position_map.get(k.as_str()).copied().unwrap_or(usize::MAX));
            }
        }
        PropertyType::Checkbox => {
            // true before false
            keys.sort_by(|a, b| {
                let a_bool = a == "true";
                let b_bool = b == "true";
                b_bool.cmp(&a_bool)
            });
        }
        PropertyType::Number => {
            keys.sort_by(|a, b| {
                let a_num: f64 = a.parse().unwrap_or(f64::MAX);
                let b_num: f64 = b.parse().unwrap_or(f64::MAX);
                a_num.total_cmp(&b_num)
            });
        }
        _ => {
            // Text, Date: ascending
            keys.sort();
        }
    }
}

/// Gets the display value for a group key.
fn get_display_value(
    key: &str,
    property_type: PropertyType,
    config: Option<&PropertyConfig>,
) -> String {
    match property_type {
        PropertyType::Select => {
            if let Some(PropertyConfig::Select { options }) = config
                && let Some(opt) = options.iter().find(|o| o.id.to_string() == key)
            {
                return opt.value.clone();
            }
            key.to_owned()
        }
        PropertyType::Checkbox => {
            if key == "true" {
                "チェック済み".to_owned()
            } else {
                "未チェック".to_owned()
            }
        }
        _ => key.to_owned(),
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use crate::domain::property::entity::{PropertyId, SelectOption, SelectOptionId};

    fn make_text(text: Option<&str>) -> RowPropertyValue {
        RowPropertyValue {
            text_value: text.map(|s| s.to_owned()),
            number_value: None,
            date_value: None,
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

    fn make_number(num: Option<f64>) -> RowPropertyValue {
        RowPropertyValue {
            text_value: None,
            number_value: num,
            date_value: None,
            boolean_value: None,
        }
    }

    #[test]
    fn group_text_case_insensitive() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let rows = vec![
            HashMap::from([(pids.clone(), make_text(Some("Alpha")))]),
            HashMap::from([(pids.clone(), make_text(Some("alpha")))]),
            HashMap::from([(pids.clone(), make_text(Some("Beta")))]),
        ];
        let cond = GroupCondition { property_id: pid };
        let (groups, indices) =
            compute_groups(&rows, &cond, PropertyType::Text, None, &HashSet::new());
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].display_value, "alpha");
        assert_eq!(groups[0].count, 2);
        assert_eq!(indices[0], vec![0, 1]);
        assert_eq!(groups[1].display_value, "beta");
        assert_eq!(groups[1].count, 1);
    }

    #[test]
    fn group_checkbox_two_groups() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let rows = vec![
            HashMap::from([(pids.clone(), make_checkbox(Some(true)))]),
            HashMap::from([(pids.clone(), make_checkbox(Some(false)))]),
            HashMap::from([(pids.clone(), make_checkbox(Some(true)))]),
        ];
        let cond = GroupCondition { property_id: pid };
        let (groups, _) =
            compute_groups(&rows, &cond, PropertyType::Checkbox, None, &HashSet::new());
        assert_eq!(groups.len(), 2);
        // Checked first, unchecked second
        assert_eq!(groups[0].display_value, "チェック済み");
        assert_eq!(groups[0].count, 2);
        assert_eq!(groups[1].display_value, "未チェック");
        assert_eq!(groups[1].count, 1);
    }

    #[test]
    fn group_null_at_end() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let rows = vec![
            HashMap::from([(pids.clone(), make_text(None))]),
            HashMap::from([(pids.clone(), make_text(Some("Hello")))]),
            HashMap::from([(pids.clone(), make_text(None))]),
        ];
        let cond = GroupCondition { property_id: pid };
        let (groups, _) = compute_groups(&rows, &cond, PropertyType::Text, None, &HashSet::new());
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].display_value, "hello");
        assert_eq!(groups[1].display_value, "未設定");
        assert_eq!(groups[1].count, 2);
        assert!(groups[1].value.is_none());
    }

    #[test]
    fn group_select_position_order() {
        let pid = PropertyId::new();
        let pids = pid.to_string();

        let opt_a = SelectOptionId::new();
        let opt_b = SelectOptionId::new();

        let config = PropertyConfig::Select {
            options: vec![
                SelectOption {
                    id: opt_a.clone(),
                    value: "Alpha".to_owned(),
                },
                SelectOption {
                    id: opt_b.clone(),
                    value: "Beta".to_owned(),
                },
            ],
        };

        let rows = vec![
            HashMap::from([(pids.clone(), make_select(Some(&opt_b.to_string())))]),
            HashMap::from([(pids.clone(), make_select(Some(&opt_a.to_string())))]),
        ];

        let cond = GroupCondition { property_id: pid };
        let (groups, _) = compute_groups(
            &rows,
            &cond,
            PropertyType::Select,
            Some(&config),
            &HashSet::new(),
        );
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].display_value, "Alpha");
        assert_eq!(groups[1].display_value, "Beta");
    }

    #[test]
    fn group_collapsed_state() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let rows = vec![
            HashMap::from([(pids.clone(), make_text(Some("A")))]),
            HashMap::from([(pids.clone(), make_text(Some("B")))]),
        ];
        let cond = GroupCondition { property_id: pid };
        let mut collapsed = HashSet::new();
        collapsed.insert("a".to_owned()); // key is lowercased

        let (groups, _) = compute_groups(&rows, &cond, PropertyType::Text, None, &collapsed);
        assert!(groups[0].is_collapsed);
        assert!(!groups[1].is_collapsed);
    }

    #[test]
    fn group_number_ascending_order() {
        let pid = PropertyId::new();
        let pids = pid.to_string();
        let rows = vec![
            HashMap::from([(pids.clone(), make_number(Some(30.0)))]),
            HashMap::from([(pids.clone(), make_number(Some(10.0)))]),
            HashMap::from([(pids.clone(), make_number(Some(20.0)))]),
        ];
        let cond = GroupCondition { property_id: pid };
        let (groups, _) = compute_groups(&rows, &cond, PropertyType::Number, None, &HashSet::new());
        assert_eq!(groups.len(), 3);
        assert_eq!(groups[0].display_value, "10");
        assert_eq!(groups[1].display_value, "20");
        assert_eq!(groups[2].display_value, "30");
    }
}
