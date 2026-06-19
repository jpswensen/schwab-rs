use super::*;

impl OrderBuilder {
    /// Compose two orders into a one-cancels-other (`OCO`) strategy.
    ///
    /// # Arguments
    ///
    /// - `first_order` - First child order in the OCO group.
    /// - `second_order` - Second child order in the OCO group.
    ///
    /// # Defaults
    ///
    /// The parent strategy is [`OrderStrategyType::Oco`]. Child orders keep
    /// their own sessions, durations, order types, legs, and prices.
    ///
    /// # Payload
    ///
    /// Emits a parent with `orderStrategyType=OCO` and a
    /// `childOrderStrategies` array containing the two provided orders. The
    /// parent omits `orderType`, `session`, `duration`, `price`, `stopPrice`,
    /// and `orderLegCollection` so it does not invent simple-order fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Number, OrderBuilder};
    ///
    /// let quantity: Number = "1".parse().unwrap();
    /// let limit_price: Number = "155.00".parse().unwrap();
    /// let stop_price: Number = "145.00".parse().unwrap();
    /// let order = OrderBuilder::one_cancels_other(
    ///     OrderBuilder::limit_sell("AAPL", quantity, limit_price),
    ///     OrderBuilder::stop_sell("AAPL", quantity, stop_price),
    /// );
    /// ```
    pub fn one_cancels_other(first_order: Self, second_order: Self) -> Self {
        Self {
            order_type: None,
            session: None,
            duration: None,
            order_strategy_type: OrderStrategyType::Oco,
            complex_order_strategy_type: None,
            price: None,
            price_link_basis: None,
            price_link_type: None,
            stop_price: None,
            stop_price_link_basis: None,
            stop_price_link_type: None,
            stop_price_offset: None,
            stop_type: None,
            activation_price: None,
            special_instruction: None,
            order_leg_collection: Vec::new(),
            child_order_strategies: vec![first_order, second_order],
        }
    }

    /// Compose an order that triggers a second order after the first fills.
    ///
    /// # Arguments
    ///
    /// - `first_order` - Parent order that must fill first.
    /// - `second_order` - Child order sent by Schwab after the first fills.
    ///
    /// # Defaults
    ///
    /// Changes the first order strategy to [`OrderStrategyType::Trigger`]. The
    /// second order keeps its own default or overridden fields.
    ///
    /// # Payload
    ///
    /// Emits the first order as the parent with `orderStrategyType=TRIGGER`
    /// and appends the second order to `childOrderStrategies`. The parent keeps
    /// its original `orderType`, `session`, `duration`, price fields, and legs.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Number, OrderBuilder};
    ///
    /// let quantity: Number = "1".parse().unwrap();
    /// let stop_price: Number = "145.00".parse().unwrap();
    /// let order = OrderBuilder::first_triggers_second(
    ///     OrderBuilder::market_buy("AAPL", quantity),
    ///     OrderBuilder::stop_sell("AAPL", quantity, stop_price),
    /// );
    ///
    /// // A bracket order triggers an OCO exit after the entry order fills.
    /// let quantity: Number = "1".parse().unwrap();
    /// let limit_price: Number = "160.00".parse().unwrap();
    /// let stop_price: Number = "140.00".parse().unwrap();
    /// let bracket = OrderBuilder::first_triggers_second(
    ///     OrderBuilder::market_buy("AAPL", quantity),
    ///     OrderBuilder::one_cancels_other(
    ///         OrderBuilder::limit_sell("AAPL", quantity, limit_price),
    ///         OrderBuilder::stop_sell("AAPL", quantity, stop_price),
    ///     ),
    /// );
    /// ```
    pub fn first_triggers_second(mut first_order: Self, second_order: Self) -> Self {
        first_order.order_strategy_type = OrderStrategyType::Trigger;
        first_order.child_order_strategies.push(second_order);
        first_order
    }

    /// Build a `MARKET` order for a single equity leg.
    ///
    /// # Arguments
    ///
    /// - `symbol` - Equity ticker symbol copied exactly as provided.
    /// - `instruction` - Equity instruction to place on the leg.
    /// - `quantity` - Number of shares for the leg.
    ///
    /// # Defaults
    ///
    /// Sets [`Session::Normal`], [`Duration::Day`], and
    /// [`OrderStrategyType::Single`].
    ///
    /// # Payload
    ///
    /// Emits `orderType=MARKET`, the provided `instruction`, and
    /// `assetType=EQUITY`. No `price` or `stopPrice` field is included.
    ///
    /// # Caution
    ///
    /// This lower-level constructor trusts the provided instruction. Prefer
    /// [`Self::market_buy`] or [`Self::market_sell`] for common buy/sell flows.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Instruction, Number, OrderBuilder};
    ///
    /// let quantity: Number = "2".parse().unwrap();
    /// let order = OrderBuilder::equity_market("TSLA", Instruction::SellShort, quantity);
    /// ```
    pub fn equity_market(
        symbol: impl Into<String>,
        instruction: Instruction,
        quantity: Number,
    ) -> Self {
        Self::single_leg(
            OrderTypeRequest::Market,
            symbol,
            instruction,
            InstrumentAssetType::Equity,
            quantity,
            None,
            None,
        )
    }

    /// Build a `LIMIT` order for a single equity leg.
    ///
    /// # Arguments
    ///
    /// - `symbol` - Equity ticker symbol copied exactly as provided.
    /// - `instruction` - Equity instruction to place on the leg.
    /// - `quantity` - Number of shares for the leg.
    /// - `price` - Limit price for the order.
    ///
    /// # Defaults
    ///
    /// Sets [`Session::Normal`], [`Duration::Day`], and
    /// [`OrderStrategyType::Single`].
    ///
    /// # Payload
    ///
    /// Emits `orderType=LIMIT`, the provided `instruction`,
    /// `assetType=EQUITY`, and `price`. No `stopPrice` field is included.
    ///
    /// # Caution
    ///
    /// This lower-level constructor trusts the provided instruction. Prefer
    /// [`Self::limit_buy`] or [`Self::limit_sell`] for common buy/sell flows.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Instruction, Number, OrderBuilder};
    ///
    /// let quantity: Number = "2".parse().unwrap();
    /// let price: Number = "250.00".parse().unwrap();
    /// let order = OrderBuilder::equity_limit(
    ///     "TSLA",
    ///     Instruction::SellShort,
    ///     quantity,
    ///     price,
    /// );
    /// ```
    pub fn equity_limit(
        symbol: impl Into<String>,
        instruction: Instruction,
        quantity: Number,
        price: Number,
    ) -> Self {
        Self::single_leg(
            OrderTypeRequest::Limit,
            symbol,
            instruction,
            InstrumentAssetType::Equity,
            quantity,
            Some(price),
            None,
        )
    }

    /// Build a `STOP` order for a single equity leg.
    ///
    /// # Arguments
    ///
    /// - `symbol` - Equity ticker symbol copied exactly as provided.
    /// - `instruction` - Equity instruction to place on the leg.
    /// - `quantity` - Number of shares for the leg.
    /// - `stop_price` - Stop price that activates the market order.
    ///
    /// # Defaults
    ///
    /// Sets [`Session::Normal`], [`Duration::Day`], and
    /// [`OrderStrategyType::Single`].
    ///
    /// # Payload
    ///
    /// Emits `orderType=STOP`, the provided `instruction`,
    /// `assetType=EQUITY`, and `stopPrice`. No `price` field is included.
    ///
    /// # Caution
    ///
    /// This lower-level constructor trusts the provided instruction. Prefer
    /// [`Self::stop_buy`] or [`Self::stop_sell`] for common buy/sell flows.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Instruction, Number, OrderBuilder};
    ///
    /// let quantity: Number = "2".parse().unwrap();
    /// let stop_price: Number = "245.00".parse().unwrap();
    /// let order = OrderBuilder::equity_stop(
    ///     "TSLA",
    ///     Instruction::SellShort,
    ///     quantity,
    ///     stop_price,
    /// );
    /// ```
    pub fn equity_stop(
        symbol: impl Into<String>,
        instruction: Instruction,
        quantity: Number,
        stop_price: Number,
    ) -> Self {
        Self::single_leg(
            OrderTypeRequest::Stop,
            symbol,
            instruction,
            InstrumentAssetType::Equity,
            quantity,
            None,
            Some(stop_price),
        )
    }

    /// Build a `STOP_LIMIT` order for a single equity leg.
    ///
    /// # Arguments
    ///
    /// - `symbol` - Equity ticker symbol copied exactly as provided.
    /// - `instruction` - Equity instruction to place on the leg.
    /// - `quantity` - Number of shares for the leg.
    /// - `price` - Limit price used after the stop activates.
    /// - `stop_price` - Stop price that activates the limit order.
    ///
    /// # Defaults
    ///
    /// Sets [`Session::Normal`], [`Duration::Day`], and
    /// [`OrderStrategyType::Single`].
    ///
    /// # Payload
    ///
    /// Emits `orderType=STOP_LIMIT`, the provided `instruction`,
    /// `assetType=EQUITY`, `price`, and `stopPrice`.
    ///
    /// # Caution
    ///
    /// This lower-level constructor trusts the provided instruction. Prefer
    /// [`Self::stop_limit_buy`] or [`Self::stop_limit_sell`] for common
    /// buy/sell flows.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Instruction, Number, OrderBuilder};
    ///
    /// let quantity: Number = "2".parse().unwrap();
    /// let price: Number = "244.00".parse().unwrap();
    /// let stop_price: Number = "245.00".parse().unwrap();
    /// let order = OrderBuilder::equity_stop_limit(
    ///     "TSLA",
    ///     Instruction::SellShort,
    ///     quantity,
    ///     price,
    ///     stop_price,
    /// );
    /// ```
    pub fn equity_stop_limit(
        symbol: impl Into<String>,
        instruction: Instruction,
        quantity: Number,
        price: Number,
        stop_price: Number,
    ) -> Self {
        Self::single_leg(
            OrderTypeRequest::StopLimit,
            symbol,
            instruction,
            InstrumentAssetType::Equity,
            quantity,
            Some(price),
            Some(stop_price),
        )
    }

    /// Build a `MARKET` order for a single option leg.
    ///
    /// # Arguments
    ///
    /// - `symbol` - Schwab option symbol copied exactly as provided.
    /// - `instruction` - Option instruction to place on the leg.
    /// - `quantity` - Number of option contracts for the leg.
    ///
    /// # Defaults
    ///
    /// Sets [`Session::Normal`], [`Duration::Day`], and
    /// [`OrderStrategyType::Single`].
    ///
    /// # Payload
    ///
    /// Emits `orderType=MARKET`, the provided `instruction`, and
    /// `assetType=OPTION`. No `price` or `stopPrice` field is included. The
    /// option symbol is not parsed, formatted, trimmed, or normalized.
    ///
    /// # Caution
    ///
    /// This lower-level constructor trusts the provided instruction. Prefer
    /// the option open/close helpers for common flows.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Instruction, Number, OrderBuilder};
    ///
    /// let quantity: Number = "1".parse().unwrap();
    /// let order = OrderBuilder::option_market(
    ///     "AAPL  260116C00150000",
    ///     Instruction::BuyToOpen,
    ///     quantity,
    /// );
    /// ```
    pub fn option_market(
        symbol: impl Into<String>,
        instruction: Instruction,
        quantity: Number,
    ) -> Self {
        Self::single_leg(
            OrderTypeRequest::Market,
            symbol,
            instruction,
            InstrumentAssetType::Option,
            quantity,
            None,
            None,
        )
    }

    /// Build a `LIMIT` order for a single option leg.
    ///
    /// # Arguments
    ///
    /// - `symbol` - Schwab option symbol copied exactly as provided.
    /// - `instruction` - Option instruction to place on the leg.
    /// - `quantity` - Number of option contracts for the leg.
    /// - `price` - Limit price for the option order.
    ///
    /// # Defaults
    ///
    /// Sets [`Session::Normal`], [`Duration::Day`], and
    /// [`OrderStrategyType::Single`].
    ///
    /// # Payload
    ///
    /// Emits `orderType=LIMIT`, the provided `instruction`,
    /// `assetType=OPTION`, and `price`. No `stopPrice` field is included. The
    /// option symbol is not parsed, formatted, trimmed, or normalized.
    ///
    /// # Caution
    ///
    /// This lower-level constructor trusts the provided instruction. Prefer
    /// the option open/close helpers for common flows.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Instruction, Number, OrderBuilder};
    ///
    /// let quantity: Number = "1".parse().unwrap();
    /// let price: Number = "2.50".parse().unwrap();
    /// let order = OrderBuilder::option_limit(
    ///     "AAPL  260116C00150000",
    ///     Instruction::SellToClose,
    ///     quantity,
    ///     price,
    /// );
    /// ```
    pub fn option_limit(
        symbol: impl Into<String>,
        instruction: Instruction,
        quantity: Number,
        price: Number,
    ) -> Self {
        Self::single_leg(
            OrderTypeRequest::Limit,
            symbol,
            instruction,
            InstrumentAssetType::Option,
            quantity,
            Some(price),
            None,
        )
    }

    /// Override the session (default: [`Session::Normal`]).
    ///
    /// # Arguments
    ///
    /// - `session` - Session value to serialize on this order.
    ///
    /// # Payload
    ///
    /// Replaces the current `session` field. Single-leg constructors start
    /// with `NORMAL`; OCO parent orders intentionally omit `session` unless
    /// this setter is called on the composed parent.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Number, OrderBuilder, Session};
    ///
    /// let quantity: Number = "10".parse().unwrap();
    /// let order = OrderBuilder::market_buy("AAPL", quantity)
    ///     .session(Session::Am);
    /// ```
    pub fn session(mut self, session: Session) -> Self {
        self.session = Some(session);
        self
    }

    /// Override the duration (default: [`Duration::Day`]).
    ///
    /// # Arguments
    ///
    /// - `duration` - Duration value to serialize on this order.
    ///
    /// # Payload
    ///
    /// Replaces the current `duration` field. Single-leg constructors start
    /// with `DAY`; OCO parent orders intentionally omit `duration` unless this
    /// setter is called on the composed parent.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Duration, Number, OrderBuilder};
    ///
    /// let quantity: Number = "10".parse().unwrap();
    /// let order = OrderBuilder::market_buy("AAPL", quantity)
    ///     .duration(Duration::GoodTillCancel);
    /// ```
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Override the order strategy type (default: [`OrderStrategyType::Single`]).
    ///
    /// # Arguments
    ///
    /// - `strategy` - Strategy type to serialize on this order.
    ///
    /// # Payload
    ///
    /// Replaces the current `orderStrategyType` field. Prefer
    /// [`Self::one_cancels_other`] or [`Self::first_triggers_second`] for OCO
    /// and trigger strategies because they also set up child orders.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Number, OrderBuilder, OrderStrategyType};
    ///
    /// let quantity: Number = "10".parse().unwrap();
    /// let order = OrderBuilder::market_buy("AAPL", quantity)
    ///     .order_strategy_type(OrderStrategyType::Single);
    /// ```
    pub fn order_strategy_type(mut self, strategy: OrderStrategyType) -> Self {
        self.order_strategy_type = strategy;
        self
    }
    /// Shared constructor for single-leg orders.
    fn single_leg(
        order_type: OrderTypeRequest,
        symbol: impl Into<String>,
        instruction: Instruction,
        asset_type: InstrumentAssetType,
        quantity: Number,
        price: Option<Number>,
        stop_price: Option<Number>,
    ) -> Self {
        Self {
            order_type: Some(order_type),
            session: Some(Session::Normal),
            duration: Some(Duration::Day),
            order_strategy_type: OrderStrategyType::Single,
            complex_order_strategy_type: None,
            price,
            price_link_basis: None,
            price_link_type: None,
            stop_price,
            stop_price_link_basis: None,
            stop_price_link_type: None,
            stop_price_offset: None,
            stop_type: None,
            activation_price: None,
            special_instruction: None,
            order_leg_collection: vec![Leg {
                instruction,
                quantity,
                instrument: LegInstrument {
                    symbol: symbol.into(),
                    asset_type,
                },
            }],
            child_order_strategies: Vec::new(),
        }
    }
}
