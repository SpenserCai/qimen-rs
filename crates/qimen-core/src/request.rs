//! Optional calculation settings without coupling the calendar crate to Qimen.

use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::{Map, Value};
use std::fmt;

use crate::{Chart, ChartRequest, Error, ExtensionOptions};

/// A reusable deterministic calculator configured once for many civil inputs.
///
/// The default enables no extensions. Each selected annotation records its rule
/// in the result. Configuration does not change the base chart construction.
///
/// ```
/// let calculator = qimen_core::Calculator::new(qimen_core::ExtensionOptions::all());
/// let chart = calculator.calculate(&qimen_core::ChartRequest::new(2026, 9, 18, 18))?;
/// assert!(chart.extensions.is_some());
/// # Ok::<(), qimen_core::Error>(())
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Calculator {
    extensions: ExtensionOptions,
}

impl Calculator {
    /// Creates a calculator with explicitly selected extension rules.
    #[must_use]
    pub const fn new(extensions: ExtensionOptions) -> Self {
        Self { extensions }
    }

    /// Returns the immutable configuration used for every calculation.
    #[must_use]
    pub const fn extensions(&self) -> &ExtensionOptions {
        &self.extensions
    }

    /// Calculates one chart, validating the civil input before any annotations.
    pub fn calculate(&self, request: &ChartRequest) -> Result<Chart, Error> {
        calculate_with_options(request, &self.extensions)
    }
}

/// Calculates a chart with per-call extension settings.
///
/// [`crate::calculate`] remains the convenient entry point with all extensions
/// disabled. Both paths use exactly the same calendrical and plate calculations.
pub fn calculate_with_options(
    request: &ChartRequest,
    extensions: &ExtensionOptions,
) -> Result<Chart, Error> {
    let mut chart = crate::calculate(request)?;
    if !extensions.is_empty() {
        chart.extensions = Some(crate::extensions::calculate(&chart, extensions));
    }
    Ok(chart)
}

/// The shared JSON/MCP request: civil fields plus optional `extensions`.
///
/// Civil fields retain their original flat JSON layout. Rust stores them in the
/// independent calendar request, avoiding a second copy of its validation rules.
/// Unknown civil fields, extension names, and rule values are rejected.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct CalculationRequest {
    /// Gregorian civil input; serialized at the top level of the JSON object.
    #[serde(flatten)]
    pub calendar: ChartRequest,
    /// Optional annotations, all disabled when omitted or empty.
    #[serde(default, skip_serializing_if = "ExtensionOptions::is_empty")]
    pub extensions: ExtensionOptions,
}

impl CalculationRequest {
    /// Calculates with the supplied calendar input and extension configuration.
    pub fn calculate(&self) -> Result<Chart, Error> {
        calculate_with_options(&self.calendar, &self.extensions)
    }
}

impl From<ChartRequest> for CalculationRequest {
    fn from(calendar: ChartRequest) -> Self {
        Self {
            calendar,
            extensions: ExtensionOptions::default(),
        }
    }
}

impl<'de> Deserialize<'de> for CalculationRequest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // Serde's derived flatten + deny_unknown_fields combination cannot be
        // relied on for strict nested requests. Remove only our owned key, then
        // pass all remaining civil keys through CalendarRequest's strict parser.
        deserializer.deserialize_map(RequestVisitor)
    }
}

struct RequestVisitor;

impl<'de> de::Visitor<'de> for RequestVisitor {
    type Value = CalculationRequest;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("civil date fields and optional named extensions")
    }

    fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut civil = Map::new();
        let mut extensions = None;
        while let Some(key) = map.next_key::<String>()? {
            if key == "extensions" {
                if extensions.is_some() {
                    return Err(de::Error::duplicate_field("extensions"));
                }
                // Validate the object and rule shapes before the option derive.
                extensions = Some(map.next_value::<OptionsObject>()?.0);
            } else {
                if civil.contains_key(&key) {
                    return Err(de::Error::custom(format!("duplicate field `{key}`")));
                }
                civil.insert(key, map.next_value::<Value>()?);
            }
        }
        let calendar = serde_json::from_value(Value::Object(civil)).map_err(de::Error::custom)?;
        Ok(CalculationRequest {
            calendar,
            extensions: extensions.unwrap_or_default(),
        })
    }
}

// Struct derives can also accept positional sequences; the public request
// contract requires an object of string/null rules. Check duplicate keys and
// shapes before passing the values through ExtensionOptions' named-rule parser.
struct OptionsObject(ExtensionOptions);

impl<'de> Deserialize<'de> for OptionsObject {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct OptionsVisitor;

        impl<'de> de::Visitor<'de> for OptionsVisitor {
            type Value = OptionsObject;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an object of named extension rules")
            }

            fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut rules = Map::new();
                while let Some(key) = map.next_key::<String>()? {
                    if rules.contains_key(&key) {
                        return Err(de::Error::custom(format!("duplicate field `{key}`")));
                    }
                    let rule = map.next_value::<Option<String>>()?;
                    rules.insert(key, rule.map_or(Value::Null, Value::String));
                }
                serde_json::from_value(Value::Object(rules))
                    .map(OptionsObject)
                    .map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_map(OptionsVisitor)
    }
}
