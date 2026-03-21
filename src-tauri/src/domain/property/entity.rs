use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::database::entity::DatabaseId;
use crate::domain::page::entity::PageId;

use super::error::{PropertyError, PropertyValueError};

/// Maximum number of characters allowed in a property name.
pub const MAX_NAME_LENGTH: usize = 100;

/// Maximum number of properties a single database may contain.
///
/// Enforced at the service layer, not within the entity constructor.
pub const MAX_PROPERTIES: usize = 50;

/// Maximum number of select options a single property may contain.
pub const MAX_SELECT_OPTIONS: usize = 100;

/// Maximum number of characters allowed in a select-option value.
pub const MAX_OPTION_VALUE_LENGTH: usize = 100;

// ---------------------------------------------------------------------------
// PropertyId
// ---------------------------------------------------------------------------

/// A UUIDv7-based identifier for a [`Property`].
///
/// Wraps [`uuid::Uuid`] and is serialized transparently.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PropertyId(Uuid);

impl PropertyId {
    /// Generates a new time-ordered UUIDv7 identifier.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Returns the inner [`Uuid`] value.
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for PropertyId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PropertyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PropertyId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s).map(Self)
    }
}

// ---------------------------------------------------------------------------
// PropertyName
// ---------------------------------------------------------------------------

/// A validated property name (1-100 Unicode characters after trimming).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PropertyName(String);

impl PropertyName {
    /// Returns the inner string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for PropertyName {
    type Error = PropertyError;

    /// Creates a new [`PropertyName`] after trimming and validating the input.
    ///
    /// # Errors
    ///
    /// - [`PropertyError::NameEmpty`] if the trimmed string is empty.
    /// - [`PropertyError::NameTooLong`] if the trimmed string exceeds 100 characters.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim().to_owned();
        if trimmed.is_empty() {
            return Err(PropertyError::NameEmpty);
        }
        let len = trimmed.chars().count();
        if len > MAX_NAME_LENGTH {
            return Err(PropertyError::NameTooLong {
                len,
                max: MAX_NAME_LENGTH,
            });
        }
        Ok(Self(trimmed))
    }
}

impl fmt::Display for PropertyName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// PropertyType
// ---------------------------------------------------------------------------

/// The fundamental type of a property column in a database.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PropertyType {
    /// Free-form text.
    Text,
    /// Numeric value.
    Number,
    /// Date or date-time value.
    Date,
    /// Single-select from a predefined list.
    Select,
    /// Boolean toggle.
    Checkbox,
}

impl fmt::Display for PropertyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Text => "text",
            Self::Number => "number",
            Self::Date => "date",
            Self::Select => "select",
            Self::Checkbox => "checkbox",
        };
        write!(f, "{s}")
    }
}

// ---------------------------------------------------------------------------
// SelectOptionId
// ---------------------------------------------------------------------------

/// A UUIDv7-based identifier for a [`SelectOption`].
///
/// Wraps [`uuid::Uuid`] and is serialized transparently.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SelectOptionId(Uuid);

impl SelectOptionId {
    /// Generates a new time-ordered UUIDv7 identifier.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Returns the inner [`Uuid`] value.
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for SelectOptionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SelectOptionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SelectOptionId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s).map(Self)
    }
}

// ---------------------------------------------------------------------------
// SelectOption
// ---------------------------------------------------------------------------

/// A single option within a [`PropertyConfig::Select`] property.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectOption {
    /// The unique identifier for this option.
    pub id: SelectOptionId,
    /// The display value of this option.
    pub value: String,
}

// ---------------------------------------------------------------------------
// DateMode
// ---------------------------------------------------------------------------

/// Controls whether a date property stores a date only or a full date-time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DateMode {
    /// Date only (no time component).
    Date,
    /// Full date and time.
    DateTime,
}

// ---------------------------------------------------------------------------
// PropertyConfig
// ---------------------------------------------------------------------------

/// Type-specific configuration for a [`Property`].
///
/// Serialized as an internally tagged enum (`"type"` field).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PropertyConfig {
    /// Configuration for a text property (currently no extra fields).
    Text,
    /// Configuration for a number property (currently no extra fields).
    Number,
    /// Configuration for a date property.
    Date {
        /// Whether the property stores date-only or date-time.
        mode: DateMode,
    },
    /// Configuration for a single-select property.
    Select {
        /// The available options for selection.
        options: Vec<SelectOption>,
    },
    /// Configuration for a checkbox property (currently no extra fields).
    Checkbox,
}

// ---------------------------------------------------------------------------
// Property
// ---------------------------------------------------------------------------

/// A property (column) within a database, defining its type and configuration.
#[derive(Debug, Clone)]
pub struct Property {
    id: PropertyId,
    database_id: DatabaseId,
    name: PropertyName,
    property_type: PropertyType,
    config: Option<PropertyConfig>,
    position: i64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Property {
    /// Creates a new [`Property`] with a generated UUIDv7 ID and the current timestamp.
    ///
    /// Validates the following constraints:
    /// - `position` must be non-negative.
    /// - If `config` is provided it must be consistent with `property_type`.
    /// - For [`PropertyType::Select`], select-option values must be 1-100 characters,
    ///   unique, and at most 100 options.
    ///
    /// # Errors
    ///
    /// Returns [`PropertyError`] if any validation constraint is violated.
    pub fn new(
        database_id: DatabaseId,
        name: PropertyName,
        property_type: PropertyType,
        config: Option<PropertyConfig>,
        position: i64,
    ) -> Result<Self, PropertyError> {
        validate_position(position)?;
        validate_config_consistency(property_type, &config)?;

        let now = Utc::now();
        Ok(Self {
            id: PropertyId::new(),
            database_id,
            name,
            property_type,
            config,
            position,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstructs a [`Property`] from stored fields (e.g. database row).
    ///
    /// No validation is performed — the caller guarantees data integrity.
    #[allow(clippy::too_many_arguments)]
    pub fn from_stored(
        id: PropertyId,
        database_id: DatabaseId,
        name: PropertyName,
        property_type: PropertyType,
        config: Option<PropertyConfig>,
        position: i64,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            database_id,
            name,
            property_type,
            config,
            position,
            created_at,
            updated_at,
        }
    }

    /// Returns a reference to the property's ID.
    pub fn id(&self) -> &PropertyId {
        &self.id
    }

    /// Returns a reference to the owning database's ID.
    pub fn database_id(&self) -> &DatabaseId {
        &self.database_id
    }

    /// Returns a reference to the property's name.
    pub fn name(&self) -> &PropertyName {
        &self.name
    }

    /// Returns the property's type.
    pub fn property_type(&self) -> PropertyType {
        self.property_type
    }

    /// Returns a reference to the property's optional configuration.
    pub fn config(&self) -> Option<&PropertyConfig> {
        self.config.as_ref()
    }

    /// Returns the property's display-order position.
    pub fn position(&self) -> i64 {
        self.position
    }

    /// Returns the property's creation timestamp.
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Returns the property's last-updated timestamp.
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

/// Validates that a position value is non-negative.
fn validate_position(position: i64) -> Result<(), PropertyError> {
    if position < 0 {
        return Err(PropertyError::InvalidConfig {
            reason: format!("position must be non-negative, got {position}"),
        });
    }
    Ok(())
}

/// Validates that `config` is consistent with `property_type`.
fn validate_config_consistency(
    property_type: PropertyType,
    config: &Option<PropertyConfig>,
) -> Result<(), PropertyError> {
    let Some(cfg) = config else {
        return Ok(());
    };

    match (property_type, cfg) {
        (PropertyType::Text, PropertyConfig::Text) => Ok(()),
        (PropertyType::Number, PropertyConfig::Number) => Ok(()),
        (PropertyType::Date, PropertyConfig::Date { .. }) => Ok(()),
        (PropertyType::Select, PropertyConfig::Select { options }) => {
            validate_select_options(options)
        }
        (PropertyType::Checkbox, PropertyConfig::Checkbox) => Ok(()),
        _ => Err(PropertyError::InvalidConfig {
            reason: format!(
                "config type '{}' does not match property type '{property_type}'",
                config_type_label(cfg)
            ),
        }),
    }
}

/// Returns a human-readable label for a [`PropertyConfig`] variant.
fn config_type_label(config: &PropertyConfig) -> String {
    match config {
        PropertyConfig::Text => "Text".to_owned(),
        PropertyConfig::Number => "Number".to_owned(),
        PropertyConfig::Date { .. } => "Date".to_owned(),
        PropertyConfig::Select { .. } => "Select".to_owned(),
        PropertyConfig::Checkbox => "Checkbox".to_owned(),
    }
}

/// Validates select-option constraints:
/// - Each value must be 1-100 characters after trimming.
/// - Values must be unique (case-sensitive).
/// - At most 100 options.
fn validate_select_options(options: &[SelectOption]) -> Result<(), PropertyError> {
    if options.len() > MAX_SELECT_OPTIONS {
        return Err(PropertyError::TooManyOptions {
            count: options.len(),
            max: MAX_SELECT_OPTIONS,
        });
    }

    let mut seen = HashSet::with_capacity(options.len());

    for opt in options {
        let trimmed = opt.value.trim();
        if trimmed.is_empty() {
            return Err(PropertyError::OptionValueEmpty);
        }
        let char_count = trimmed.chars().count();
        if char_count > MAX_OPTION_VALUE_LENGTH {
            return Err(PropertyError::InvalidConfig {
                reason: format!(
                    "select option value too long: {char_count} characters (max {MAX_OPTION_VALUE_LENGTH})"
                ),
            });
        }
        if !seen.insert(trimmed.to_owned()) {
            return Err(PropertyError::DuplicateOptionValue {
                value: trimmed.to_owned(),
            });
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// PropertyValueId
// ---------------------------------------------------------------------------

/// A UUIDv7-based identifier for a [`PropertyValue`].
///
/// Wraps [`uuid::Uuid`] and is serialized transparently.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PropertyValueId(Uuid);

impl PropertyValueId {
    /// Generates a new time-ordered UUIDv7 identifier.
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Returns the inner [`Uuid`] value.
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for PropertyValueId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PropertyValueId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PropertyValueId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s).map(Self)
    }
}

// ---------------------------------------------------------------------------
// PropertyValueInput
// ---------------------------------------------------------------------------

/// Raw user input for a property value, dispatched by property type.
#[derive(Debug, Clone)]
pub enum PropertyValueInput {
    /// Free-form text input.
    Text(String),
    /// Numeric input.
    Number(f64),
    /// Date/date-time input.
    Date(DateTime<Utc>),
    /// Select option input (the option ID as a string).
    Select(String),
    /// Boolean input.
    Checkbox(bool),
}

// ---------------------------------------------------------------------------
// PropertyValue
// ---------------------------------------------------------------------------

/// A concrete value assigned to a [`Property`] for a specific page row.
#[derive(Debug, Clone)]
pub struct PropertyValue {
    id: PropertyValueId,
    page_id: PageId,
    property_id: PropertyId,
    text_value: Option<String>,
    number_value: Option<f64>,
    date_value: Option<DateTime<Utc>>,
    boolean_value: Option<bool>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl PropertyValue {
    /// Creates a new [`PropertyValue`] by validating `input` against the
    /// property's type and optional configuration.
    ///
    /// # Errors
    ///
    /// - [`PropertyValueError::TypeMismatch`] if the input variant does not
    ///   match the property type.
    /// - [`PropertyValueError::NumberNotFinite`] if a numeric value is NaN or
    ///   infinite.
    /// - [`PropertyValueError::InvalidSelectOption`] if the select option ID
    ///   is not present in the property's configuration.
    pub fn new_validated(
        page_id: PageId,
        property_id: PropertyId,
        property_type: PropertyType,
        config: Option<&PropertyConfig>,
        input: PropertyValueInput,
    ) -> Result<Self, PropertyValueError> {
        validate_input_type(property_type, &property_id, &input)?;

        let mut text_value: Option<String> = None;
        let mut number_value: Option<f64> = None;
        let mut date_value: Option<DateTime<Utc>> = None;
        let mut boolean_value: Option<bool> = None;

        match input {
            PropertyValueInput::Text(s) => {
                text_value = Some(s);
            }
            PropertyValueInput::Number(n) => {
                if !n.is_finite() {
                    return Err(PropertyValueError::InvalidNumber {
                        reason: "value must be finite (not NaN or Infinity)".to_owned(),
                    });
                }
                // Normalize -0.0 to 0.0
                let normalized = if n == 0.0 { 0.0 } else { n };
                number_value = Some(normalized);
            }
            PropertyValueInput::Date(dt) => {
                date_value = Some(dt);
            }
            PropertyValueInput::Select(option_id) => {
                validate_select_option_exists(config, &property_id, &option_id)?;
                text_value = Some(option_id);
            }
            PropertyValueInput::Checkbox(b) => {
                boolean_value = Some(b);
            }
        }

        let now = Utc::now();
        Ok(Self {
            id: PropertyValueId::new(),
            page_id,
            property_id,
            text_value,
            number_value,
            date_value,
            boolean_value,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstructs a [`PropertyValue`] from stored fields (e.g. database row).
    ///
    /// No validation is performed — the caller guarantees data integrity.
    #[allow(clippy::too_many_arguments)]
    pub fn from_stored(
        id: PropertyValueId,
        page_id: PageId,
        property_id: PropertyId,
        text_value: Option<String>,
        number_value: Option<f64>,
        date_value: Option<DateTime<Utc>>,
        boolean_value: Option<bool>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            page_id,
            property_id,
            text_value,
            number_value,
            date_value,
            boolean_value,
            created_at,
            updated_at,
        }
    }

    /// Returns a reference to the value's ID.
    pub fn id(&self) -> &PropertyValueId {
        &self.id
    }

    /// Returns a reference to the associated page's ID.
    pub fn page_id(&self) -> &PageId {
        &self.page_id
    }

    /// Returns a reference to the associated property's ID.
    pub fn property_id(&self) -> &PropertyId {
        &self.property_id
    }

    /// Returns the stored text value, if any.
    pub fn text_value(&self) -> Option<&str> {
        self.text_value.as_deref()
    }

    /// Returns the stored number value, if any.
    pub fn number_value(&self) -> Option<f64> {
        self.number_value
    }

    /// Returns the stored date value, if any.
    pub fn date_value(&self) -> Option<DateTime<Utc>> {
        self.date_value
    }

    /// Returns the stored boolean value, if any.
    pub fn boolean_value(&self) -> Option<bool> {
        self.boolean_value
    }

    /// Returns the value's creation timestamp.
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Returns the value's last-updated timestamp.
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

/// Validates that the [`PropertyValueInput`] variant matches the expected
/// [`PropertyType`].
fn validate_input_type(
    property_type: PropertyType,
    property_id: &PropertyId,
    input: &PropertyValueInput,
) -> Result<(), PropertyValueError> {
    let matches = matches!(
        (property_type, input),
        (PropertyType::Text, PropertyValueInput::Text(_))
            | (PropertyType::Number, PropertyValueInput::Number(_))
            | (PropertyType::Date, PropertyValueInput::Date(_))
            | (PropertyType::Select, PropertyValueInput::Select(_))
            | (PropertyType::Checkbox, PropertyValueInput::Checkbox(_))
    );

    if !matches {
        return Err(PropertyValueError::TypeMismatch {
            expected: property_type,
            property_id: property_id.clone(),
        });
    }
    Ok(())
}

/// Validates that the given `option_id` exists in the property's select
/// configuration.
fn validate_select_option_exists(
    config: Option<&PropertyConfig>,
    property_id: &PropertyId,
    option_id: &str,
) -> Result<(), PropertyValueError> {
    if let Some(PropertyConfig::Select { options }) = config {
        let found = options.iter().any(|o| o.id.to_string() == option_id);
        if !found {
            return Err(PropertyValueError::InvalidSelectOption {
                option_id: option_id.to_owned(),
                property_id: property_id.clone(),
            });
        }
    } else {
        return Err(PropertyValueError::InvalidSelectOption {
            option_id: option_id.to_owned(),
            property_id: property_id.clone(),
        });
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    // -- PropertyName -------------------------------------------------------

    #[test]
    fn property_name_valid() {
        let name = PropertyName::try_from("Status".to_owned());
        assert!(name.is_ok());
        assert_eq!(name.as_ref().map(|n| n.as_str()), Ok("Status"));
    }

    #[test]
    fn property_name_empty_is_rejected() {
        let result = PropertyName::try_from(String::new());
        assert!(matches!(result, Err(PropertyError::NameEmpty)));
    }

    #[test]
    fn property_name_whitespace_only_is_rejected() {
        let result = PropertyName::try_from("   \t\n  ".to_owned());
        assert!(matches!(result, Err(PropertyError::NameEmpty)));
    }

    #[test]
    fn property_name_over_100_chars_is_rejected() {
        let s = "a".repeat(101);
        let result = PropertyName::try_from(s);
        assert!(matches!(
            result,
            Err(PropertyError::NameTooLong { len: 101, max: 100 })
        ));
    }

    #[test]
    fn property_name_exactly_100_chars_is_accepted() {
        let s = "a".repeat(100);
        let result = PropertyName::try_from(s);
        assert!(result.is_ok());
    }

    #[test]
    fn property_name_trims_whitespace() {
        let name = PropertyName::try_from("  Priority  ".to_owned());
        assert!(name.is_ok());
        assert_eq!(name.as_ref().map(|n| n.as_str()), Ok("Priority"));
    }

    // -- PropertyType Display -----------------------------------------------

    #[test]
    fn property_type_display_lowercase() {
        assert_eq!(PropertyType::Text.to_string(), "text");
        assert_eq!(PropertyType::Number.to_string(), "number");
        assert_eq!(PropertyType::Date.to_string(), "date");
        assert_eq!(PropertyType::Select.to_string(), "select");
        assert_eq!(PropertyType::Checkbox.to_string(), "checkbox");
    }

    // -- PropertyConfig serde roundtrip -------------------------------------

    #[test]
    fn property_config_serde_text() {
        let cfg = PropertyConfig::Text;
        let json = serde_json::to_string(&cfg).expect("serialize");
        let deserialized: PropertyConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(cfg, deserialized);
    }

    #[test]
    fn property_config_serde_number() {
        let cfg = PropertyConfig::Number;
        let json = serde_json::to_string(&cfg).expect("serialize");
        let deserialized: PropertyConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(cfg, deserialized);
    }

    #[test]
    fn property_config_serde_date() {
        let cfg = PropertyConfig::Date {
            mode: DateMode::DateTime,
        };
        let json = serde_json::to_string(&cfg).expect("serialize");
        let deserialized: PropertyConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(cfg, deserialized);
    }

    #[test]
    fn property_config_serde_select() {
        let cfg = PropertyConfig::Select {
            options: vec![
                SelectOption {
                    id: SelectOptionId::new(),
                    value: "Alpha".to_owned(),
                },
                SelectOption {
                    id: SelectOptionId::new(),
                    value: "Beta".to_owned(),
                },
            ],
        };
        let json = serde_json::to_string(&cfg).expect("serialize");
        let deserialized: PropertyConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(cfg, deserialized);
    }

    #[test]
    fn property_config_serde_checkbox() {
        let cfg = PropertyConfig::Checkbox;
        let json = serde_json::to_string(&cfg).expect("serialize");
        let deserialized: PropertyConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(cfg, deserialized);
    }

    // -- Property::new ------------------------------------------------------

    #[test]
    fn property_new_valid_creation() {
        let db_id = DatabaseId::new();
        let name = PropertyName::try_from("Title".to_owned()).expect("valid name");
        let result = Property::new(db_id, name, PropertyType::Text, None, 0);
        assert!(result.is_ok());
        let prop = result.expect("valid property");
        assert_eq!(prop.property_type(), PropertyType::Text);
        assert!(prop.config().is_none());
        assert_eq!(prop.position(), 0);
        assert_eq!(prop.created_at(), prop.updated_at());
    }

    #[test]
    fn property_new_with_matching_config() {
        let db_id = DatabaseId::new();
        let name = PropertyName::try_from("Due Date".to_owned()).expect("valid name");
        let cfg = Some(PropertyConfig::Date {
            mode: DateMode::Date,
        });
        let result = Property::new(db_id, name, PropertyType::Date, cfg, 1);
        assert!(result.is_ok());
    }

    #[test]
    fn property_new_rejects_mismatched_config() {
        let db_id = DatabaseId::new();
        let name = PropertyName::try_from("Status".to_owned()).expect("valid name");
        // Provide a Text config for a Number type
        let cfg = Some(PropertyConfig::Text);
        let result = Property::new(db_id, name, PropertyType::Number, cfg, 0);
        assert!(matches!(
            result,
            Err(PropertyError::InvalidConfig { .. })
        ));
    }

    #[test]
    fn property_new_rejects_negative_position() {
        let db_id = DatabaseId::new();
        let name = PropertyName::try_from("Col".to_owned()).expect("valid name");
        let result = Property::new(db_id, name, PropertyType::Text, None, -1);
        assert!(matches!(
            result,
            Err(PropertyError::InvalidConfig { .. })
        ));
    }

    // -- SelectOption validation --------------------------------------------

    #[test]
    fn select_option_empty_value_rejected() {
        let db_id = DatabaseId::new();
        let name = PropertyName::try_from("Category".to_owned()).expect("valid name");
        let cfg = Some(PropertyConfig::Select {
            options: vec![SelectOption {
                id: SelectOptionId::new(),
                value: String::new(),
            }],
        });
        let result = Property::new(db_id, name, PropertyType::Select, cfg, 0);
        assert!(matches!(result, Err(PropertyError::OptionValueEmpty)));
    }

    #[test]
    fn select_option_duplicate_values_rejected() {
        let db_id = DatabaseId::new();
        let name = PropertyName::try_from("Tags".to_owned()).expect("valid name");
        let cfg = Some(PropertyConfig::Select {
            options: vec![
                SelectOption {
                    id: SelectOptionId::new(),
                    value: "Dup".to_owned(),
                },
                SelectOption {
                    id: SelectOptionId::new(),
                    value: "Dup".to_owned(),
                },
            ],
        });
        let result = Property::new(db_id, name, PropertyType::Select, cfg, 0);
        assert!(matches!(
            result,
            Err(PropertyError::DuplicateOptionValue { .. })
        ));
    }

    #[test]
    fn select_option_over_100_options_rejected() {
        let db_id = DatabaseId::new();
        let name = PropertyName::try_from("Big".to_owned()).expect("valid name");
        let options: Vec<SelectOption> = (0..101)
            .map(|i| SelectOption {
                id: SelectOptionId::new(),
                value: format!("opt-{i}"),
            })
            .collect();
        let cfg = Some(PropertyConfig::Select { options });
        let result = Property::new(db_id, name, PropertyType::Select, cfg, 0);
        assert!(matches!(
            result,
            Err(PropertyError::TooManyOptions {
                count: 101,
                max: 100
            })
        ));
    }

    // -- PropertyValue ------------------------------------------------------

    #[test]
    fn property_value_number_nan_rejected() {
        let result = PropertyValue::new_validated(
            PageId::new(),
            PropertyId::new(),
            PropertyType::Number,
            None,
            PropertyValueInput::Number(f64::NAN),
        );
        assert!(matches!(
            result,
            Err(PropertyValueError::InvalidNumber { .. })
        ));
    }

    #[test]
    fn property_value_number_infinity_rejected() {
        let result = PropertyValue::new_validated(
            PageId::new(),
            PropertyId::new(),
            PropertyType::Number,
            None,
            PropertyValueInput::Number(f64::INFINITY),
        );
        assert!(matches!(
            result,
            Err(PropertyValueError::InvalidNumber { .. })
        ));
    }

    #[test]
    fn property_value_number_neg_zero_normalized() {
        let result = PropertyValue::new_validated(
            PageId::new(),
            PropertyId::new(),
            PropertyType::Number,
            None,
            PropertyValueInput::Number(-0.0),
        );
        assert!(result.is_ok());
        let pv = result.expect("valid value");
        let n = pv.number_value().expect("has number");
        // Verify it is positive zero: 1/+0.0 = +inf, 1/-0.0 = -inf
        assert!(n.is_sign_positive(), "should be positive zero");
    }

    #[test]
    fn property_value_select_invalid_option_rejected() {
        let opt_id = SelectOptionId::new();
        let config = PropertyConfig::Select {
            options: vec![SelectOption {
                id: opt_id,
                value: "Valid".to_owned(),
            }],
        };
        let result = PropertyValue::new_validated(
            PageId::new(),
            PropertyId::new(),
            PropertyType::Select,
            Some(&config),
            PropertyValueInput::Select("nonexistent-id".to_owned()),
        );
        assert!(matches!(
            result,
            Err(PropertyValueError::InvalidSelectOption { .. })
        ));
    }

    #[test]
    fn property_value_select_valid_option_accepted() {
        let opt_id = SelectOptionId::new();
        let opt_id_str = opt_id.to_string();
        let config = PropertyConfig::Select {
            options: vec![SelectOption {
                id: opt_id,
                value: "Valid".to_owned(),
            }],
        };
        let result = PropertyValue::new_validated(
            PageId::new(),
            PropertyId::new(),
            PropertyType::Select,
            Some(&config),
            PropertyValueInput::Select(opt_id_str),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn property_value_type_mismatch_rejected() {
        // Send Text input for a Number property
        let result = PropertyValue::new_validated(
            PageId::new(),
            PropertyId::new(),
            PropertyType::Number,
            None,
            PropertyValueInput::Text("not a number".to_owned()),
        );
        assert!(matches!(
            result,
            Err(PropertyValueError::TypeMismatch { .. })
        ));
    }

    #[test]
    fn property_value_text_accepted() {
        let result = PropertyValue::new_validated(
            PageId::new(),
            PropertyId::new(),
            PropertyType::Text,
            None,
            PropertyValueInput::Text("hello".to_owned()),
        );
        assert!(result.is_ok());
        let pv = result.expect("valid value");
        assert_eq!(pv.text_value(), Some("hello"));
    }

    #[test]
    fn property_value_checkbox_accepted() {
        let result = PropertyValue::new_validated(
            PageId::new(),
            PropertyId::new(),
            PropertyType::Checkbox,
            None,
            PropertyValueInput::Checkbox(true),
        );
        assert!(result.is_ok());
        let pv = result.expect("valid value");
        assert_eq!(pv.boolean_value(), Some(true));
    }

    // -- ID roundtrips ------------------------------------------------------

    #[test]
    fn property_id_display_and_from_str_roundtrip() {
        let id = PropertyId::new();
        let s = id.to_string();
        let parsed: PropertyId = s.parse().expect("should parse");
        assert_eq!(id, parsed);
    }

    #[test]
    fn select_option_id_display_and_from_str_roundtrip() {
        let id = SelectOptionId::new();
        let s = id.to_string();
        let parsed: SelectOptionId = s.parse().expect("should parse");
        assert_eq!(id, parsed);
    }

    #[test]
    fn property_value_id_display_and_from_str_roundtrip() {
        let id = PropertyValueId::new();
        let s = id.to_string();
        let parsed: PropertyValueId = s.parse().expect("should parse");
        assert_eq!(id, parsed);
    }
}
