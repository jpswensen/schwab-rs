use serde::Serialize;

use crate::models::Number;
use crate::models::enums::{
    ComplexOrderStrategyType, Duration, Instruction, InstrumentAssetType, OrderStrategyType,
    OrderType, OrderTypeRequest, PriceLinkBasis, PriceLinkType, Session, SpecialInstruction,
    StopPriceLinkBasis, StopPriceLinkType, StopType,
};
use crate::models::{AccountsInstrument, Order, OrderLegCollection};
use crate::{Error, Result};

/// Instrument description for order submission.
///
/// Contains only the fields the Schwab API requires when placing orders.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LegInstrument {
    symbol: String,
    asset_type: InstrumentAssetType,
}

/// A single leg in an order.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Leg {
    instruction: Instruction,
    quantity: Number,
    instrument: LegInstrument,
}

/// Convenience builder for constructing Schwab order payloads.
///
/// Produces a [`Serialize`]-able value that [`crate::Client::place_order`],
/// [`crate::Client::replace_order`], and [`crate::Client::preview_order`] accept
/// directly.
///
/// The buy/sell constructors cover common equity orders, and the option
/// constructors cover common buy-to-open, sell-to-open, buy-to-close, and
/// sell-to-close orders without requiring callers to choose raw instruction
/// values. The `equity_*` constructors stay available for advanced equity
/// instructions such as short sales. Single orders set sensible defaults
/// (`NORMAL` session, `DAY` duration, `SINGLE` strategy). Override them with
/// the fluent setters, or compose builders with [`Self::one_cancels_other`]
/// and [`Self::first_triggers_second`].
///
/// Public constructor docs use consistent `Arguments`, `Defaults`, and
/// `Payload` sections so downstream tools can generate command help without
/// reverse-engineering the serialized JSON shape.
///
/// # Examples
///
/// ```
/// use schwab::{Instruction, Number, OrderBuilder};
///
/// // Market buy 10 shares of AAPL
/// let quantity: Number = "10".parse().unwrap();
/// let order = OrderBuilder::market_buy("AAPL", quantity);
///
/// // Limit buy 5 shares of MSFT at $400, good-til-cancel
/// let quantity: Number = "5".parse().unwrap();
/// let price: Number = "400".parse().unwrap();
/// let order = OrderBuilder::limit_buy("MSFT", quantity, price)
///     .duration(schwab::Duration::GoodTillCancel);
///
/// // Advanced instructions are still available when needed.
/// let quantity: Number = "2".parse().unwrap();
/// let order = OrderBuilder::equity_market("TSLA", Instruction::SellShort, quantity);
///
/// // Buy to open one option contract at market.
/// let quantity: Number = "1".parse().unwrap();
/// let order = OrderBuilder::option_buy_to_open_market("AAPL  260116C00150000", quantity);
///
/// // Compose two already-built orders into an OCO order.
/// let quantity: Number = "1".parse().unwrap();
/// let limit_price: Number = "140".parse().unwrap();
/// let stop_price: Number = "120".parse().unwrap();
/// let order = OrderBuilder::one_cancels_other(
///     OrderBuilder::limit_sell("AAPL", quantity, limit_price),
///     OrderBuilder::stop_sell("AAPL", quantity, stop_price),
/// );
///
/// // Buy shares, then place a bracket exit with profit target and stop loss.
/// let quantity: Number = "1".parse().unwrap();
/// let limit_price: Number = "160".parse().unwrap();
/// let stop_price: Number = "140".parse().unwrap();
/// let order = OrderBuilder::first_triggers_second(
///     OrderBuilder::market_buy("AAPL", quantity),
///     OrderBuilder::one_cancels_other(
///         OrderBuilder::limit_sell("AAPL", quantity, limit_price),
///         OrderBuilder::stop_sell("AAPL", quantity, stop_price),
///     ),
/// );
/// ```
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    order_type: Option<OrderTypeRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session: Option<Session>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Duration>,
    order_strategy_type: OrderStrategyType,
    #[serde(skip_serializing_if = "Option::is_none")]
    complex_order_strategy_type: Option<ComplexOrderStrategyType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_link_basis: Option<PriceLinkBasis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_link_type: Option<PriceLinkType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_price: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_price_link_basis: Option<StopPriceLinkBasis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_price_link_type: Option<StopPriceLinkType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_price_offset: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_type: Option<StopType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activation_price: Option<Number>,
    #[serde(skip_serializing_if = "Option::is_none")]
    special_instruction: Option<SpecialInstruction>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    order_leg_collection: Vec<Leg>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    child_order_strategies: Vec<OrderBuilder>,
}

mod constructors;
mod convert;
mod core;

#[cfg(test)]
mod tests;
