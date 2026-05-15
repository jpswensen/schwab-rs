//! Option screener streaming data types.

use super::screener_equity::ScreenerItem;

/// Field selector for option screener streaming subscriptions.
///
/// Each variant corresponds to a numeric field index in the Schwab streaming protocol.
///
/// # Examples
///
/// ```
/// use schwab::ScreenerOptionField;
///
/// assert_eq!(ScreenerOptionField::Symbol.index(), 0);
/// assert_eq!(ScreenerOptionField::Items.index(), 4);
/// ```
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ScreenerOptionField {
    Symbol = 0,
    Timestamp = 1,
    SortField = 2,
    Frequency = 3,
    Items = 4,
}

impl ScreenerOptionField {
    /// Return the numeric field index used in the Schwab streaming protocol.
    pub fn index(&self) -> u32 {
        *self as u32
    }

    /// Return all `ScreenerOptionField` variants in index order.
    pub fn all() -> &'static [ScreenerOptionField] {
        use ScreenerOptionField::*;
        &[Symbol, Timestamp, SortField, Frequency, Items]
    }
}

/// Option screener streaming data for a single screener key.
///
/// All fields are `Option<T>` because the Schwab API sends only subscribed fields.
/// Named metadata fields use string keys; numeric data fields use numeric string keys.
///
/// Screener keys follow the format `(PREFIX)_(SORTFIELD)_(FREQUENCY)` where
/// PREFIX is an option category (`OPTION_PUT`, `OPTION_CALL`, `OPTION_ALL`),
/// SORTFIELD is one of `VOLUME`, `TRADES`, `PERCENT_CHANGE_UP`,
/// `PERCENT_CHANGE_DOWN`, or `AVERAGE_PERCENT_VOLUME`, and FREQUENCY is
/// 0 (all day), 1, 5, 10, 30, or 60 minutes.
///
/// # Examples
///
/// ```
/// use schwab::ScreenerOption;
///
/// let data = ScreenerOption {
///     symbol: Some("OPTION_CALL_VOLUME_0".to_string()),
///     ..Default::default()
/// };
/// assert_eq!(data.symbol.as_deref(), Some("OPTION_CALL_VOLUME_0"));
/// ```
#[allow(missing_docs)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScreenerOption {
    // Named metadata fields (string-keyed in the protocol)
    pub key: Option<String>,
    pub delayed: Option<bool>,
    pub asset_main_type: Option<String>,
    pub asset_sub_type: Option<String>,
    pub cusip: Option<String>,
    // Numeric data fields (index-keyed: "0", "1", ...)
    pub symbol: Option<String>,
    pub timestamp: Option<i64>,
    pub sort_field: Option<String>,
    pub frequency: Option<i64>,
    pub items: Option<Vec<ScreenerItem>>,
}

impl ScreenerOption {
    /// Construct a [`ScreenerOption`] from a streaming data map entry.
    ///
    /// The map uses named string keys for metadata (`"key"`, `"delayed"`) and
    /// numeric string keys (`"0"`, `"1"`, ...) for field data. Field `"4"`
    /// contains a nested array of [`ScreenerItem`] objects.
    /// Returns `None` if `value` is not a JSON object.
    pub(crate) fn from_value(value: &serde_json::Value) -> Option<Self> {
        let map = value.as_object()?;
        let mut s = Self {
            key: map.get("key").and_then(|v| v.as_str()).map(String::from),
            delayed: map.get("delayed").and_then(|v| v.as_bool()),
            asset_main_type: map
                .get("assetMainType")
                .and_then(|v| v.as_str())
                .map(String::from),
            asset_sub_type: map
                .get("assetSubType")
                .and_then(|v| v.as_str())
                .map(String::from),
            cusip: map.get("cusip").and_then(|v| v.as_str()).map(String::from),
            ..Self::default()
        };

        // Numeric-keyed data fields
        for (key, val) in map {
            match key.as_str() {
                "0" => s.symbol = val.as_str().map(String::from),
                "1" => s.timestamp = val.as_i64(),
                "2" => s.sort_field = val.as_str().map(String::from),
                "3" => s.frequency = val.as_i64(),
                "4" => {
                    s.items = val
                        .as_array()
                        .map(|arr| arr.iter().filter_map(ScreenerItem::from_value).collect());
                }
                _ => {}
            }
        }

        Some(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn field_index_first() {
        assert_eq!(ScreenerOptionField::Symbol.index(), 0);
    }

    #[test]
    fn field_index_last() {
        assert_eq!(ScreenerOptionField::Items.index(), 4);
    }

    #[test]
    fn all_fields_count() {
        assert_eq!(ScreenerOptionField::all().len(), 5);
    }

    #[test]
    fn all_fields_sequential_indices() {
        for (i, field) in ScreenerOptionField::all().iter().enumerate() {
            assert_eq!(
                field.index() as usize,
                i,
                "field at position {i} has wrong index"
            );
        }
    }

    #[test]
    fn from_value_parses_sample() {
        let input = json!({
            "key": "OPTION_CALL_VOLUME_0",
            "0": "OPTION_CALL_VOLUME_0",
            "1": 1234567890000_i64,
            "2": "VOLUME",
            "3": 0,
            "4": [
                {
                    "description": "AAPL Dec 2025 200 Call",
                    "lastPrice": 5.5,
                    "symbol": "AAPL  251219C00200000",
                    "totalVolume": 25000
                }
            ]
        });

        let screener = ScreenerOption::from_value(&input).expect("should parse JSON object");

        assert_eq!(screener.key, Some("OPTION_CALL_VOLUME_0".to_string()));
        assert_eq!(screener.symbol, Some("OPTION_CALL_VOLUME_0".to_string()));
        assert_eq!(screener.timestamp, Some(1234567890000));
        let items = screener.items.expect("should have items");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].symbol.as_deref(), Some("AAPL  251219C00200000"));
        assert_eq!(items[0].last_price, Some("5.5".parse().unwrap()));
    }

    #[test]
    fn from_value_returns_none_for_non_object() {
        assert!(ScreenerOption::from_value(&json!(42)).is_none());
        assert!(ScreenerOption::from_value(&json!("text")).is_none());
        assert!(ScreenerOption::from_value(&json!(null)).is_none());
    }
}
