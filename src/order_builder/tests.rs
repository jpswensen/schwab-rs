use std::assert_matches;

use super::*;
use crate::Error;
use crate::test_support::n;

cfg_select! {
    feature = "decimal" => {
        fn expected_number(value: f64) -> serde_json::Value {
            serde_json::json!(n(value).to_string())
        }
    }
    _ => {
        fn expected_number(value: f64) -> serde_json::Value {
            serde_json::json!(value)
        }
    }
}

/// Market order serializes with required fields and no price.
#[test]
fn market_order_json() {
    let order = OrderBuilder::equity_market("AAPL", Instruction::Buy, n(10.0));
    let json: serde_json::Value = serde_json::to_value(&order).unwrap();

    assert_eq!(json["orderType"], "MARKET");
    assert_eq!(json["session"], "NORMAL");
    assert_eq!(json["duration"], "DAY");
    assert_eq!(json["orderStrategyType"], "SINGLE");
    assert!(json.get("price").is_none());
    assert!(json.get("stopPrice").is_none());

    let legs = json["orderLegCollection"].as_array().unwrap();
    assert_eq!(legs.len(), 1);
    assert_eq!(legs[0]["instruction"], "BUY");
    assert_eq!(legs[0]["quantity"], expected_number(10.0));
    assert_eq!(legs[0]["instrument"]["symbol"], "AAPL");
    assert_eq!(legs[0]["instrument"]["assetType"], "EQUITY");
}

/// Limit order includes price and omits stopPrice.
#[test]
fn limit_order_json() {
    let order = OrderBuilder::equity_limit("MSFT", Instruction::Sell, n(5.0), n(400.50));
    let json: serde_json::Value = serde_json::to_value(&order).unwrap();

    assert_eq!(json["orderType"], "LIMIT");
    assert_eq!(json["price"], expected_number(400.50));
    assert!(json.get("stopPrice").is_none());
    assert_eq!(json["orderLegCollection"][0]["instruction"], "SELL");
    assert_eq!(
        json["orderLegCollection"][0]["quantity"],
        expected_number(5.0)
    );
}

/// Stop order includes stopPrice and omits price.
#[test]
fn stop_order_json() {
    let order = OrderBuilder::equity_stop("GOOG", Instruction::Sell, n(3.0), n(150.0));
    let json: serde_json::Value = serde_json::to_value(&order).unwrap();

    assert_eq!(json["orderType"], "STOP");
    assert_eq!(json["stopPrice"], expected_number(150.0));
    assert!(json.get("price").is_none());
}

/// Stop-limit order includes both price and stopPrice.
#[test]
fn stop_limit_order_json() {
    let order =
        OrderBuilder::equity_stop_limit("TSLA", Instruction::Buy, n(2.0), n(200.0), n(195.0));
    let json: serde_json::Value = serde_json::to_value(&order).unwrap();

    assert_eq!(json["orderType"], "STOP_LIMIT");
    assert_eq!(json["price"], expected_number(200.0));
    assert_eq!(json["stopPrice"], expected_number(195.0));
}

/// Fluent setters override defaults.
#[test]
fn fluent_setters() {
    let order = OrderBuilder::equity_market("SPY", Instruction::Buy, n(1.0))
        .session(Session::Am)
        .duration(Duration::GoodTillCancel)
        .order_strategy_type(OrderStrategyType::Trigger);

    let json: serde_json::Value = serde_json::to_value(&order).unwrap();
    assert_eq!(json["session"], "AM");
    assert_eq!(json["duration"], "GOOD_TILL_CANCEL");
    assert_eq!(json["orderStrategyType"], "TRIGGER");
}
/// Convenience constructors choose buy/sell instructions.
#[test]
fn buy_sell_convenience_constructors() {
    let cases = [
        (OrderBuilder::market_buy("AAPL", n(1.0)), "MARKET", "BUY"),
        (OrderBuilder::market_sell("AAPL", n(1.0)), "MARKET", "SELL"),
        (
            OrderBuilder::limit_buy("AAPL", n(1.0), n(100.0)),
            "LIMIT",
            "BUY",
        ),
        (
            OrderBuilder::limit_sell("AAPL", n(1.0), n(100.0)),
            "LIMIT",
            "SELL",
        ),
        (
            OrderBuilder::stop_buy("AAPL", n(1.0), n(90.0)),
            "STOP",
            "BUY",
        ),
        (
            OrderBuilder::stop_sell("AAPL", n(1.0), n(90.0)),
            "STOP",
            "SELL",
        ),
        (
            OrderBuilder::stop_limit_buy("AAPL", n(1.0), n(91.0), n(90.0)),
            "STOP_LIMIT",
            "BUY",
        ),
        (
            OrderBuilder::stop_limit_sell("AAPL", n(1.0), n(91.0), n(90.0)),
            "STOP_LIMIT",
            "SELL",
        ),
    ];

    for (order, expected_type, expected_instruction) in cases {
        let json: serde_json::Value = serde_json::to_value(&order).unwrap();
        assert_eq!(json["orderType"], expected_type);
        assert_eq!(
            json["orderLegCollection"][0]["instruction"],
            expected_instruction
        );
    }
}

/// Option convenience constructors choose option instructions and asset type.
#[test]
fn option_convenience_constructors() {
    let symbol = "AAPL  260116C00150000";
    let cases = [
        (
            OrderBuilder::option_buy_to_open_market(symbol, n(1.0)),
            "MARKET",
            "BUY_TO_OPEN",
            None,
        ),
        (
            OrderBuilder::option_buy_to_open_limit(symbol, n(1.0), n(2.5)),
            "LIMIT",
            "BUY_TO_OPEN",
            Some(expected_number(2.5)),
        ),
        (
            OrderBuilder::option_sell_to_open_market(symbol, n(1.0)),
            "MARKET",
            "SELL_TO_OPEN",
            None,
        ),
        (
            OrderBuilder::option_sell_to_open_limit(symbol, n(1.0), n(2.5)),
            "LIMIT",
            "SELL_TO_OPEN",
            Some(expected_number(2.5)),
        ),
        (
            OrderBuilder::option_buy_to_close_market(symbol, n(1.0)),
            "MARKET",
            "BUY_TO_CLOSE",
            None,
        ),
        (
            OrderBuilder::option_buy_to_close_limit(symbol, n(1.0), n(2.5)),
            "LIMIT",
            "BUY_TO_CLOSE",
            Some(expected_number(2.5)),
        ),
        (
            OrderBuilder::option_sell_to_close_market(symbol, n(1.0)),
            "MARKET",
            "SELL_TO_CLOSE",
            None,
        ),
        (
            OrderBuilder::option_sell_to_close_limit(symbol, n(1.0), n(2.5)),
            "LIMIT",
            "SELL_TO_CLOSE",
            Some(expected_number(2.5)),
        ),
    ];

    for (order, expected_type, expected_instruction, expected_price) in cases {
        let json: serde_json::Value = serde_json::to_value(&order).unwrap();
        assert_eq!(json["orderType"], expected_type);
        assert_eq!(json["session"], "NORMAL");
        assert_eq!(json["duration"], "DAY");
        assert_eq!(json["orderStrategyType"], "SINGLE");
        assert_eq!(
            json["orderLegCollection"][0]["instruction"],
            expected_instruction
        );
        assert_eq!(
            json["orderLegCollection"][0]["quantity"],
            expected_number(1.0)
        );
        assert_eq!(
            json["orderLegCollection"][0]["instrument"]["symbol"],
            symbol
        );
        assert_eq!(
            json["orderLegCollection"][0]["instrument"]["assetType"],
            "OPTION"
        );

        if let Some(price) = expected_price {
            assert_eq!(json["price"], price);
        } else {
            assert!(json.get("price").is_none());
        }
    }
}

/// Lower-level option constructors accept explicit option instructions.
#[test]
fn option_lower_level_constructors() {
    let order = OrderBuilder::option_limit(
        "MSFT  260116P00300000",
        Instruction::SellToClose,
        n(2.0),
        n(3.25),
    );
    let json: serde_json::Value = serde_json::to_value(&order).unwrap();

    assert_eq!(json["orderType"], "LIMIT");
    assert_eq!(json["price"], expected_number(3.25));
    assert_eq!(
        json["orderLegCollection"][0]["instruction"],
        "SELL_TO_CLOSE"
    );
    assert_eq!(
        json["orderLegCollection"][0]["quantity"],
        expected_number(2.0)
    );
    assert_eq!(
        json["orderLegCollection"][0]["instrument"]["assetType"],
        "OPTION"
    );
}

/// OCO composition nests two child orders without inventing parent order fields.
#[test]
fn one_cancels_other_json() {
    let order = OrderBuilder::one_cancels_other(
        OrderBuilder::limit_sell("AAPL", n(1.0), n(140.0)),
        OrderBuilder::stop_sell("AAPL", n(1.0), n(120.0)),
    );
    let json: serde_json::Value = serde_json::to_value(&order).unwrap();

    assert_eq!(json["orderStrategyType"], "OCO");
    assert!(json.get("orderType").is_none());
    assert!(json.get("session").is_none());
    assert!(json.get("duration").is_none());
    assert!(json.get("orderLegCollection").is_none());

    let children = json["childOrderStrategies"].as_array().unwrap();
    assert_eq!(children.len(), 2);
    assert_eq!(children[0]["orderType"], "LIMIT");
    assert_eq!(children[0]["orderLegCollection"][0]["instruction"], "SELL");
    assert_eq!(children[1]["orderType"], "STOP");
    assert_eq!(children[1]["orderLegCollection"][0]["instruction"], "SELL");
}

/// Trigger composition keeps the first order as the parent and nests the second order.
#[test]
fn first_triggers_second_json() {
    let order = OrderBuilder::first_triggers_second(
        OrderBuilder::market_buy("AAPL", n(1.0)),
        OrderBuilder::limit_sell("AAPL", n(1.0), n(140.0)),
    );
    let json: serde_json::Value = serde_json::to_value(&order).unwrap();

    assert_eq!(json["orderType"], "MARKET");
    assert_eq!(json["orderStrategyType"], "TRIGGER");
    assert_eq!(json["orderLegCollection"][0]["instruction"], "BUY");

    let children = json["childOrderStrategies"].as_array().unwrap();
    assert_eq!(children.len(), 1);
    assert_eq!(children[0]["orderType"], "LIMIT");
    assert_eq!(children[0]["orderStrategyType"], "SINGLE");
    assert_eq!(children[0]["orderLegCollection"][0]["instruction"], "SELL");
}

/// Bracket composition triggers an OCO exit after the entry order fills.
#[test]
fn bracket_order_json() {
    let order = OrderBuilder::first_triggers_second(
        OrderBuilder::market_buy("AAPL", n(1.0)),
        OrderBuilder::one_cancels_other(
            OrderBuilder::limit_sell("AAPL", n(1.0), n(160.0)),
            OrderBuilder::stop_sell("AAPL", n(1.0), n(140.0)),
        ),
    );
    let json: serde_json::Value = serde_json::to_value(&order).unwrap();

    assert_eq!(json["orderType"], "MARKET");
    assert_eq!(json["orderStrategyType"], "TRIGGER");
    assert_eq!(json["orderLegCollection"][0]["instruction"], "BUY");

    let trigger_children = json["childOrderStrategies"].as_array().unwrap();
    assert_eq!(trigger_children.len(), 1);
    assert_eq!(trigger_children[0]["orderStrategyType"], "OCO");
    assert!(trigger_children[0].get("orderType").is_none());
    assert!(trigger_children[0].get("orderLegCollection").is_none());

    let oco_children = trigger_children[0]["childOrderStrategies"]
        .as_array()
        .unwrap();
    assert_eq!(oco_children.len(), 2);
    assert_eq!(oco_children[0]["orderType"], "LIMIT");
    assert_eq!(oco_children[0]["price"], expected_number(160.0));
    assert_eq!(
        oco_children[0]["orderLegCollection"][0]["instruction"],
        "SELL"
    );
    assert_eq!(oco_children[1]["orderType"], "STOP");
    assert_eq!(oco_children[1]["stopPrice"], expected_number(140.0));
    assert_eq!(
        oco_children[1]["orderLegCollection"][0]["instruction"],
        "SELL"
    );
}
/// Historical single-leg limit orders convert into submit-ready payloads.
#[test]
fn converts_single_equity_order() {
    let order: Order = serde_json::from_value(serde_json::json!({
        "orderId": 123456,
        "status": "FILLED",
        "enteredTime": "2026-01-02T03:04:05+0000",
        "session": "NORMAL",
        "duration": "GOOD_TILL_CANCEL",
        "orderType": "LIMIT",
        "price": expected_number(400.50),
        "orderStrategyType": "SINGLE",
        "orderLegCollection": [{
            "orderLegType": "EQUITY",
            "instruction": "BUY",
            "quantity": expected_number(5.0),
            "instrument": {
                "assetType": "EQUITY",
                "symbol": "MSFT",
                "description": "Microsoft Corp"
            }
        }]
    }))
    .unwrap();

    let builder = OrderBuilder::try_from_order(&order).unwrap();
    let json = serde_json::to_value(&builder).unwrap();

    assert_eq!(json["session"], "NORMAL");
    assert_eq!(json["duration"], "GOOD_TILL_CANCEL");
    assert_eq!(json["orderType"], "LIMIT");
    assert_eq!(json["price"], expected_number(400.50));
    assert_eq!(json["orderStrategyType"], "SINGLE");
    assert_eq!(json["orderLegCollection"][0]["instruction"], "BUY");
    assert_eq!(
        json["orderLegCollection"][0]["quantity"],
        expected_number(5.0)
    );
    assert_eq!(
        json["orderLegCollection"][0]["instrument"]["symbol"],
        "MSFT"
    );
    assert_eq!(
        json["orderLegCollection"][0]["instrument"]["assetType"],
        "EQUITY"
    );
    assert!(json.get("orderId").is_none());
    assert!(json.get("status").is_none());
    assert!(json.get("enteredTime").is_none());
}

/// Historical option legs keep option asset type and symbol.
#[test]
fn converts_option_leg() {
    let order: Order = serde_json::from_value(serde_json::json!({
        "session": "NORMAL",
        "duration": "DAY",
        "orderType": "LIMIT",
        "price": expected_number(1.25),
        "orderStrategyType": "SINGLE",
        "orderLegCollection": [{
            "orderLegType": "OPTION",
            "instruction": "BUY_TO_OPEN",
            "quantity": expected_number(1.0),
            "instrument": {
                "assetType": "OPTION",
                "symbol": "MSFT  260116C00400000",
                "putCall": "CALL",
                "type": "VANILLA"
            }
        }]
    }))
    .unwrap();

    let builder = OrderBuilder::try_from_order(&order).unwrap();
    let json = serde_json::to_value(&builder).unwrap();

    assert_eq!(json["orderLegCollection"][0]["instruction"], "BUY_TO_OPEN");
    assert_eq!(
        json["orderLegCollection"][0]["instrument"]["assetType"],
        "OPTION"
    );
    assert_eq!(
        json["orderLegCollection"][0]["instrument"]["symbol"],
        "MSFT  260116C00400000"
    );
}

/// Trigger orders recursively preserve the first child strategy.
#[test]
fn converts_trigger_order_with_child() {
    let order: Order = serde_json::from_value(serde_json::json!({
        "session": "NORMAL",
        "duration": "DAY",
        "orderType": "LIMIT",
        "price": expected_number(400.0),
        "orderStrategyType": "TRIGGER",
        "orderLegCollection": [{
            "orderLegType": "EQUITY",
            "instruction": "BUY",
            "quantity": expected_number(1.0),
            "instrument": { "assetType": "EQUITY", "symbol": "MSFT" }
        }],
        "childOrderStrategies": [{
            "session": "NORMAL",
            "duration": "DAY",
            "orderType": "STOP",
            "stopPrice": expected_number(390.0),
            "orderStrategyType": "SINGLE",
            "orderLegCollection": [{
                "orderLegType": "EQUITY",
                "instruction": "SELL",
                "quantity": expected_number(1.0),
                "instrument": { "assetType": "EQUITY", "symbol": "MSFT" }
            }]
        }]
    }))
    .unwrap();

    let builder = OrderBuilder::try_from_order(&order).unwrap();
    let json = serde_json::to_value(&builder).unwrap();

    assert_eq!(json["orderStrategyType"], "TRIGGER");
    assert_eq!(json["childOrderStrategies"].as_array().unwrap().len(), 1);
    assert_eq!(json["childOrderStrategies"][0]["orderType"], "STOP");
    assert_eq!(
        json["childOrderStrategies"][0]["stopPrice"],
        expected_number(390.0)
    );
}

/// OCO wrapper orders can convert without top-level order fields or legs.
#[test]
fn converts_oco_order_with_two_children() {
    let child = |order_type: &str, instruction: &str, stop_price: Option<serde_json::Value>| {
        let mut value = serde_json::json!({
            "session": "NORMAL",
            "duration": "DAY",
            "orderType": order_type,
            "price": expected_number(400.0),
            "orderStrategyType": "SINGLE",
            "orderLegCollection": [{
                "orderLegType": "EQUITY",
                "instruction": instruction,
                "quantity": expected_number(1.0),
                "instrument": { "assetType": "EQUITY", "symbol": "MSFT" }
            }]
        });

        if let Some(stop_price) = stop_price {
            value["stopPrice"] = stop_price;
        }

        value
    };
    let order: Order = serde_json::from_value(serde_json::json!({
        "orderStrategyType": "OCO",
        "childOrderStrategies": [
            child("LIMIT", "SELL", None),
            child("STOP_LIMIT", "SELL", Some(expected_number(390.0)))
        ]
    }))
    .unwrap();

    let builder = OrderBuilder::try_from_order(&order).unwrap();
    let json = serde_json::to_value(&builder).unwrap();

    assert_eq!(json["orderStrategyType"], "OCO");
    assert!(json.get("orderType").is_none());
    assert!(json.get("orderLegCollection").is_none());
    assert_eq!(json["childOrderStrategies"].as_array().unwrap().len(), 2);
    assert_eq!(json["childOrderStrategies"][1]["orderType"], "STOP_LIMIT");
}

/// Missing order-type-specific prices fail instead of emitting invalid payloads.
#[test]
fn rejects_missing_type_specific_prices() {
    let cases = [
        ("LIMIT", None, None, "price"),
        ("STOP", None, None, "stopPrice"),
        ("STOP_LIMIT", Some(expected_number(10.0)), None, "stopPrice"),
        ("STOP_LIMIT", None, Some(expected_number(9.0)), "price"),
    ];

    for (order_type, price, stop_price, missing_field) in cases {
        let mut value = serde_json::json!({
            "session": "NORMAL",
            "duration": "DAY",
            "orderType": order_type,
            "orderStrategyType": "SINGLE",
            "orderLegCollection": [{
                "orderLegType": "EQUITY",
                "instruction": "BUY",
                "quantity": expected_number(1.0),
                "instrument": { "assetType": "EQUITY", "symbol": "MSFT" }
            }]
        });

        if let Some(price) = price {
            value["price"] = price;
        }
        if let Some(stop_price) = stop_price {
            value["stopPrice"] = stop_price;
        }

        let order: Order = serde_json::from_value(value).unwrap();
        let error = OrderBuilder::try_from_order(&order).unwrap_err();

        assert_matches!(error, Error::OrderConversion(message) if message.contains(missing_field));
    }
}

/// SINGLE orders with children fail because child strategies would be dropped.
#[test]
fn rejects_single_order_with_children() {
    let order: Order = serde_json::from_value(serde_json::json!({
        "session": "NORMAL",
        "duration": "DAY",
        "orderType": "MARKET",
        "orderStrategyType": "SINGLE",
        "orderLegCollection": [{
            "orderLegType": "EQUITY",
            "instruction": "BUY",
            "quantity": expected_number(1.0),
            "instrument": { "assetType": "EQUITY", "symbol": "MSFT" }
        }],
        "childOrderStrategies": [{ "orderStrategyType": "OCO" }]
    }))
    .unwrap();

    let error = OrderBuilder::try_from_order(&order).unwrap_err();

    assert_matches!(error, Error::OrderConversion(message) if message.contains("SINGLE"));
}

/// Common response-level quantity is allowed when it matches the single leg.
#[test]
fn accepts_matching_top_level_quantity() {
    let order: Order = serde_json::from_value(serde_json::json!({
        "session": "NORMAL",
        "duration": "DAY",
        "orderType": "MARKET",
        "quantity": expected_number(1.0),
        "orderStrategyType": "SINGLE",
        "orderLegCollection": [{
            "orderLegType": "EQUITY",
            "instruction": "BUY",
            "quantity": expected_number(1.0),
            "instrument": { "assetType": "EQUITY", "symbol": "MSFT" }
        }]
    }))
    .unwrap();

    let builder = OrderBuilder::try_from_order(&order).unwrap();
    let json = serde_json::to_value(&builder).unwrap();

    assert!(json.get("quantity").is_none());
    assert_eq!(
        json["orderLegCollection"][0]["quantity"],
        expected_number(1.0)
    );
}

/// Unsupported or inconsistent top-level request fields fail rather than being dropped.
#[test]
fn rejects_unsupported_top_level_request_fields() {
    let mut value = serde_json::json!({
        "session": "NORMAL",
        "duration": "DAY",
        "orderType": "MARKET",
        "quantity": expected_number(2.0),
        "orderStrategyType": "SINGLE",
        "orderLegCollection": [{
            "orderLegType": "EQUITY",
            "instruction": "BUY",
            "quantity": expected_number(1.0),
            "instrument": { "assetType": "EQUITY", "symbol": "MSFT" }
        }]
    });
    let order: Order = serde_json::from_value(value.clone()).unwrap();
    let error = OrderBuilder::try_from_order(&order).unwrap_err();
    assert_matches!(error, Error::OrderConversion(message) if message.contains("does not match"));

    value["quantity"] = expected_number(1.0);
    value["taxLotMethod"] = serde_json::json!("FIFO");
    let order: Order = serde_json::from_value(value).unwrap();
    let error = OrderBuilder::try_from_order(&order).unwrap_err();
    assert_matches!(error, Error::OrderConversion(message) if message.contains("taxLotMethod"));
}

/// Top-level quantity with multiple legs fails because this builder cannot model spreads yet.
#[test]
fn rejects_top_level_quantity_for_multi_leg_orders() {
    let order: Order = serde_json::from_value(serde_json::json!({
        "session": "NORMAL",
        "duration": "DAY",
        "orderType": "MARKET",
        "quantity": expected_number(2.0),
        "orderStrategyType": "SINGLE",
        "orderLegCollection": [
            {
                "orderLegType": "EQUITY",
                "instruction": "BUY",
                "quantity": expected_number(1.0),
                "instrument": { "assetType": "EQUITY", "symbol": "MSFT" }
            },
            {
                "orderLegType": "EQUITY",
                "instruction": "BUY",
                "quantity": expected_number(1.0),
                "instrument": { "assetType": "EQUITY", "symbol": "AAPL" }
            }
        ]
    }))
    .unwrap();

    let error = OrderBuilder::try_from_order(&order).unwrap_err();

    assert_matches!(error, Error::OrderConversion(message) if message.contains("without exactly one leg"));
}

/// Unsupported leg request fields fail rather than being dropped.
#[test]
fn rejects_unsupported_leg_request_fields() {
    for (field, expected) in [
        ("quantityType", "quantityType"),
        ("positionEffect", "positionEffect"),
        ("divCapGains", "divCapGains"),
        ("toSymbol", "toSymbol"),
    ] {
        let mut value = serde_json::json!({
            "session": "NORMAL",
            "duration": "DAY",
            "orderType": "MARKET",
            "orderStrategyType": "SINGLE",
            "orderLegCollection": [{
                "orderLegType": "EQUITY",
                "instruction": "BUY",
                "quantity": expected_number(1.0),
                "instrument": { "assetType": "EQUITY", "symbol": "MSFT" }
            }]
        });
        value["orderLegCollection"][0][field] = match field {
            "quantityType" => serde_json::json!("ALL_SHARES"),
            "positionEffect" => serde_json::json!("OPENING"),
            "divCapGains" => serde_json::json!("REINVEST"),
            _ => serde_json::json!("MSFT2"),
        };

        let order: Order = serde_json::from_value(value).unwrap();
        let error = OrderBuilder::try_from_order(&order).unwrap_err();

        assert_matches!(error, Error::OrderConversion(message) if message.contains(expected));
    }
}

/// Missing leg-field errors include the leg index for actionable diagnostics.
#[test]
fn missing_leg_field_errors_include_index() {
    let order: Order = serde_json::from_value(serde_json::json!({
        "session": "NORMAL",
        "duration": "DAY",
        "orderType": "MARKET",
        "orderStrategyType": "SINGLE",
        "orderLegCollection": [
            {
                "orderLegType": "EQUITY",
                "instruction": "BUY",
                "quantity": expected_number(1.0),
                "instrument": { "assetType": "EQUITY", "symbol": "MSFT" }
            },
            {
                "orderLegType": "EQUITY",
                "quantity": expected_number(1.0),
                "instrument": { "assetType": "EQUITY", "symbol": "AAPL" }
            }
        ]
    }))
    .unwrap();

    let error = OrderBuilder::try_from_order(&order).unwrap_err();

    assert_matches!(error, Error::OrderConversion(message) if message.contains("orderLegCollection[1].instruction"));
}

/// Unknown response-only order types fail instead of guessing a request type.
#[test]
fn rejects_unknown_order_type() {
    let order: Order = serde_json::from_value(serde_json::json!({
        "session": "NORMAL",
        "duration": "DAY",
        "orderType": "UNKNOWN",
        "orderStrategyType": "SINGLE",
        "orderLegCollection": [{
            "orderLegType": "EQUITY",
            "instruction": "BUY",
            "quantity": expected_number(1.0),
            "instrument": { "assetType": "EQUITY", "symbol": "MSFT" }
        }]
    }))
    .unwrap();

    let error = OrderBuilder::try_from_order(&order).unwrap_err();

    assert_matches!(error, Error::OrderConversion(message) if message.contains("UNKNOWN"));
}

/// Malformed OCO trees fail with a conversion error.
#[test]
fn rejects_oco_without_two_children() {
    let order: Order = serde_json::from_value(serde_json::json!({
        "orderStrategyType": "OCO",
        "childOrderStrategies": []
    }))
    .unwrap();

    let error = OrderBuilder::try_from_order(&order).unwrap_err();

    assert_matches!(error, Error::OrderConversion(message) if message.contains("requires 2"));
}
