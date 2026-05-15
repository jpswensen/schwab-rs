//! Equity screener streaming data types.

use super::super::Number;

/// Field selector for equity screener streaming subscriptions.
///
/// Each variant corresponds to a numeric field index in the Schwab streaming protocol.
///
/// # Examples
///
/// ```
/// use schwab::ScreenerEquityField;
///
/// assert_eq!(ScreenerEquityField::Symbol.index(), 0);
/// assert_eq!(ScreenerEquityField::Items.index(), 4);
/// ```
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ScreenerEquityField {
    Symbol = 0,
    Timestamp = 1,
    SortField = 2,
    Frequency = 3,
    Items = 4,
}

impl ScreenerEquityField {
    /// Return the numeric field index used in the Schwab streaming protocol.
    pub fn index(&self) -> u32 {
        *self as u32
    }

    /// Return all `ScreenerEquityField` variants in index order.
    pub fn all() -> &'static [ScreenerEquityField] {
        use ScreenerEquityField::*;
        &[Symbol, Timestamp, SortField, Frequency, Items]
    }
}

/// Single item from a screener streaming update.
///
/// Screener items use named camelCase keys in the Schwab streaming protocol
/// rather than numeric indices. Each item represents one security in the
/// screener results.
///
/// # Examples
///
/// ```
/// use schwab::ScreenerItem;
///
/// let item = ScreenerItem {
///     symbol: Some("AAPL".to_string()),
///     ..Default::default()
/// };
/// assert_eq!(item.symbol.as_deref(), Some("AAPL"));
/// ```
#[allow(missing_docs)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScreenerItem {
    pub description: Option<String>,
    pub last_price: Option<Number>,
    pub market_share: Option<Number>,
    pub net_change: Option<Number>,
    pub net_percent_change: Option<Number>,
    pub symbol: Option<String>,
    pub total_volume: Option<i64>,
    pub trades: Option<i64>,
    pub volume: Option<i64>,
}

/// Parse a [`Number`] from a [`serde_json::Value`].
///
/// Works for both `f64` (default) and `rust_decimal::Decimal` (`decimal` feature).
fn parse_num(v: &serde_json::Value) -> Option<Number> {
    serde_json::from_value::<Number>(v.clone()).ok()
}

impl ScreenerItem {
    /// Construct a [`ScreenerItem`] from a streaming data array element.
    ///
    /// Items use named camelCase keys (`lastPrice`, `symbol`, etc.).
    /// Returns `None` if `value` is not a JSON object.
    pub(crate) fn from_value(value: &serde_json::Value) -> Option<Self> {
        let map = value.as_object()?;
        Some(Self {
            description: map
                .get("description")
                .and_then(|v| v.as_str())
                .map(String::from),
            last_price: map.get("lastPrice").and_then(parse_num),
            market_share: map.get("marketShare").and_then(parse_num),
            net_change: map.get("netChange").and_then(parse_num),
            net_percent_change: map.get("netPercentChange").and_then(parse_num),
            symbol: map.get("symbol").and_then(|v| v.as_str()).map(String::from),
            total_volume: map.get("totalVolume").and_then(|v| v.as_i64()),
            trades: map.get("trades").and_then(|v| v.as_i64()),
            volume: map.get("volume").and_then(|v| v.as_i64()),
        })
    }
}

/// Equity screener streaming data for a single screener key.
///
/// All fields are `Option<T>` because the Schwab API sends only subscribed fields.
/// Named metadata fields use string keys; numeric data fields use numeric string keys.
///
/// Screener keys follow the format `(PREFIX)_(SORTFIELD)_(FREQUENCY)` where
/// PREFIX is an exchange or index identifier (e.g. `NASDAQ`, `$SPX`),
/// SORTFIELD is one of `VOLUME`, `TRADES`, `PERCENT_CHANGE_UP`,
/// `PERCENT_CHANGE_DOWN`, or `AVERAGE_PERCENT_VOLUME`, and FREQUENCY is
/// 0 (all day), 1, 5, 10, 30, or 60 minutes.
///
/// # Examples
///
/// ```
/// use schwab::ScreenerEquity;
///
/// let data = ScreenerEquity {
///     symbol: Some("NASDAQ_VOLUME_0".to_string()),
///     ..Default::default()
/// };
/// assert_eq!(data.symbol.as_deref(), Some("NASDAQ_VOLUME_0"));
/// ```
#[allow(missing_docs)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScreenerEquity {
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

impl ScreenerEquity {
    /// Construct a [`ScreenerEquity`] from a streaming data map entry.
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
        assert_eq!(ScreenerEquityField::Symbol.index(), 0);
    }

    #[test]
    fn field_index_last() {
        assert_eq!(ScreenerEquityField::Items.index(), 4);
    }

    #[test]
    fn all_fields_count() {
        assert_eq!(ScreenerEquityField::all().len(), 5);
    }

    #[test]
    fn all_fields_sequential_indices() {
        for (i, field) in ScreenerEquityField::all().iter().enumerate() {
            assert_eq!(
                field.index() as usize,
                i,
                "field at position {i} has wrong index"
            );
        }
    }

    #[test]
    fn item_from_value_parses_sample() {
        let input = json!({
            "description": "NVIDIA Corporation",
            "lastPrice": 120.5,
            "marketShare": 5.25,
            "netChange": 3.75,
            "netPercentChange": 3.2105,
            "symbol": "NVDA",
            "totalVolume": 85000000,
            "trades": 150000,
            "volume": 12000000
        });

        let item = ScreenerItem::from_value(&input).expect("should parse JSON object");

        assert_eq!(item.description.as_deref(), Some("NVIDIA Corporation"));
        assert_eq!(item.last_price, Some("120.5".parse().unwrap()));
        assert_eq!(item.market_share, Some("5.25".parse().unwrap()));
        assert_eq!(item.net_change, Some("3.75".parse().unwrap()));
        assert_eq!(item.net_percent_change, Some("3.2105".parse().unwrap()));
        assert_eq!(item.symbol.as_deref(), Some("NVDA"));
        assert_eq!(item.total_volume, Some(85000000));
        assert_eq!(item.trades, Some(150000));
        assert_eq!(item.volume, Some(12000000));
    }

    #[test]
    fn item_from_value_returns_none_for_non_object() {
        assert!(ScreenerItem::from_value(&json!(42)).is_none());
    }

    #[test]
    fn from_value_parses_sample() {
        let input = json!({
            "key": "NASDAQ_VOLUME_0",
            "0": "NASDAQ_VOLUME_0",
            "1": 1234567890000_i64,
            "2": "VOLUME",
            "3": 0,
            "4": [
                {
                    "description": "NVIDIA Corporation",
                    "lastPrice": 120.5,
                    "symbol": "NVDA",
                    "totalVolume": 85000000
                }
            ]
        });

        let screener = ScreenerEquity::from_value(&input).expect("should parse JSON object");

        assert_eq!(screener.key, Some("NASDAQ_VOLUME_0".to_string()));
        assert_eq!(screener.symbol, Some("NASDAQ_VOLUME_0".to_string()));
        assert_eq!(screener.timestamp, Some(1234567890000));
        assert_eq!(screener.sort_field, Some("VOLUME".to_string()));
        assert_eq!(screener.frequency, Some(0));
        let items = screener.items.expect("should have items");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].symbol.as_deref(), Some("NVDA"));
        assert_eq!(items[0].last_price, Some("120.5".parse().unwrap()));
    }

    #[test]
    fn from_value_returns_none_for_non_object() {
        assert!(ScreenerEquity::from_value(&json!(42)).is_none());
        assert!(ScreenerEquity::from_value(&json!("text")).is_none());
        assert!(ScreenerEquity::from_value(&json!(null)).is_none());
    }
}
