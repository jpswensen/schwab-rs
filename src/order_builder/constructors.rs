use super::*;

impl OrderBuilder {
    /// Build a market buy order for a single equity leg.
    ///
    /// # Arguments
    ///
    /// - `symbol` - Equity ticker symbol copied exactly as provided.
    /// - `quantity` - Number of shares to buy.
    ///
    /// # Defaults
    ///
    /// Sets [`Session::Normal`], [`Duration::Day`], and
    /// [`OrderStrategyType::Single`].
    ///
    /// # Payload
    ///
    /// Emits `orderType=MARKET`, `instruction=BUY`, and `assetType=EQUITY`.
    /// No `price` or `stopPrice` field is included.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Number, OrderBuilder};
    ///
    /// let quantity: Number = "10".parse().unwrap();
    /// let order = OrderBuilder::market_buy("AAPL", quantity);
    /// ```
    pub fn market_buy(symbol: impl Into<String>, quantity: Number) -> Self {
        Self::equity_market(symbol, Instruction::Buy, quantity)
    }

    /// Build a market sell order for a single equity leg.
    ///
    /// # Arguments
    ///
    /// - `symbol` - Equity ticker symbol copied exactly as provided.
    /// - `quantity` - Number of shares to sell.
    ///
    /// # Defaults
    ///
    /// Sets [`Session::Normal`], [`Duration::Day`], and
    /// [`OrderStrategyType::Single`].
    ///
    /// # Payload
    ///
    /// Emits `orderType=MARKET`, `instruction=SELL`, and `assetType=EQUITY`.
    /// No `price` or `stopPrice` field is included.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Number, OrderBuilder};
    ///
    /// let quantity: Number = "10".parse().unwrap();
    /// let order = OrderBuilder::market_sell("AAPL", quantity);
    /// ```
    pub fn market_sell(symbol: impl Into<String>, quantity: Number) -> Self {
        Self::equity_market(symbol, Instruction::Sell, quantity)
    }

    /// Build a limit buy order for a single equity leg.
    ///
    /// # Arguments
    ///
    /// - `symbol` - Equity ticker symbol copied exactly as provided.
    /// - `quantity` - Number of shares to buy.
    /// - `price` - Limit price for the buy order.
    ///
    /// # Defaults
    ///
    /// Sets [`Session::Normal`], [`Duration::Day`], and
    /// [`OrderStrategyType::Single`].
    ///
    /// # Payload
    ///
    /// Emits `orderType=LIMIT`, `instruction=BUY`, `assetType=EQUITY`, and
    /// `price`. No `stopPrice` field is included.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Number, OrderBuilder};
    ///
    /// let quantity: Number = "5".parse().unwrap();
    /// let price: Number = "150.00".parse().unwrap();
    /// let order = OrderBuilder::limit_buy("AAPL", quantity, price);
    /// ```
    pub fn limit_buy(symbol: impl Into<String>, quantity: Number, price: Number) -> Self {
        Self::equity_limit(symbol, Instruction::Buy, quantity, price)
    }

    /// Build a limit sell order for a single equity leg.
    ///
    /// # Arguments
    ///
    /// - `symbol` - Equity ticker symbol copied exactly as provided.
    /// - `quantity` - Number of shares to sell.
    /// - `price` - Limit price for the sell order.
    ///
    /// # Defaults
    ///
    /// Sets [`Session::Normal`], [`Duration::Day`], and
    /// [`OrderStrategyType::Single`].
    ///
    /// # Payload
    ///
    /// Emits `orderType=LIMIT`, `instruction=SELL`, `assetType=EQUITY`, and
    /// `price`. No `stopPrice` field is included.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Number, OrderBuilder};
    ///
    /// let quantity: Number = "5".parse().unwrap();
    /// let price: Number = "150.00".parse().unwrap();
    /// let order = OrderBuilder::limit_sell("AAPL", quantity, price);
    /// ```
    pub fn limit_sell(symbol: impl Into<String>, quantity: Number, price: Number) -> Self {
        Self::equity_limit(symbol, Instruction::Sell, quantity, price)
    }

    /// Build a stop buy order for a single equity leg.
    ///
    /// # Arguments
    ///
    /// - `symbol` - Equity ticker symbol copied exactly as provided.
    /// - `quantity` - Number of shares to buy.
    /// - `stop_price` - Stop price that activates the market buy order.
    ///
    /// # Defaults
    ///
    /// Sets [`Session::Normal`], [`Duration::Day`], and
    /// [`OrderStrategyType::Single`].
    ///
    /// # Payload
    ///
    /// Emits `orderType=STOP`, `instruction=BUY`, `assetType=EQUITY`, and
    /// `stopPrice`. No `price` field is included.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Number, OrderBuilder};
    ///
    /// let quantity: Number = "5".parse().unwrap();
    /// let stop_price: Number = "155.00".parse().unwrap();
    /// let order = OrderBuilder::stop_buy("AAPL", quantity, stop_price);
    /// ```
    pub fn stop_buy(symbol: impl Into<String>, quantity: Number, stop_price: Number) -> Self {
        Self::equity_stop(symbol, Instruction::Buy, quantity, stop_price)
    }

    /// Build a stop sell order for a single equity leg.
    ///
    /// # Arguments
    ///
    /// - `symbol` - Equity ticker symbol copied exactly as provided.
    /// - `quantity` - Number of shares to sell.
    /// - `stop_price` - Stop price that activates the market sell order.
    ///
    /// # Defaults
    ///
    /// Sets [`Session::Normal`], [`Duration::Day`], and
    /// [`OrderStrategyType::Single`].
    ///
    /// # Payload
    ///
    /// Emits `orderType=STOP`, `instruction=SELL`, `assetType=EQUITY`, and
    /// `stopPrice`. No `price` field is included.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Number, OrderBuilder};
    ///
    /// let quantity: Number = "5".parse().unwrap();
    /// let stop_price: Number = "145.00".parse().unwrap();
    /// let order = OrderBuilder::stop_sell("AAPL", quantity, stop_price);
    /// ```
    pub fn stop_sell(symbol: impl Into<String>, quantity: Number, stop_price: Number) -> Self {
        Self::equity_stop(symbol, Instruction::Sell, quantity, stop_price)
    }

    /// Build a stop-limit buy order for a single equity leg.
    ///
    /// # Arguments
    ///
    /// - `symbol` - Equity ticker symbol copied exactly as provided.
    /// - `quantity` - Number of shares to buy.
    /// - `price` - Limit price used after the stop activates.
    /// - `stop_price` - Stop price that activates the limit buy order.
    ///
    /// # Defaults
    ///
    /// Sets [`Session::Normal`], [`Duration::Day`], and
    /// [`OrderStrategyType::Single`].
    ///
    /// # Payload
    ///
    /// Emits `orderType=STOP_LIMIT`, `instruction=BUY`, `assetType=EQUITY`,
    /// `price`, and `stopPrice`.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Number, OrderBuilder};
    ///
    /// let quantity: Number = "5".parse().unwrap();
    /// let price: Number = "156.00".parse().unwrap();
    /// let stop_price: Number = "155.00".parse().unwrap();
    /// let order = OrderBuilder::stop_limit_buy("AAPL", quantity, price, stop_price);
    /// ```
    pub fn stop_limit_buy(
        symbol: impl Into<String>,
        quantity: Number,
        price: Number,
        stop_price: Number,
    ) -> Self {
        Self::equity_stop_limit(symbol, Instruction::Buy, quantity, price, stop_price)
    }

    /// Build a stop-limit sell order for a single equity leg.
    ///
    /// # Arguments
    ///
    /// - `symbol` - Equity ticker symbol copied exactly as provided.
    /// - `quantity` - Number of shares to sell.
    /// - `price` - Limit price used after the stop activates.
    /// - `stop_price` - Stop price that activates the limit sell order.
    ///
    /// # Defaults
    ///
    /// Sets [`Session::Normal`], [`Duration::Day`], and
    /// [`OrderStrategyType::Single`].
    ///
    /// # Payload
    ///
    /// Emits `orderType=STOP_LIMIT`, `instruction=SELL`, `assetType=EQUITY`,
    /// `price`, and `stopPrice`.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Number, OrderBuilder};
    ///
    /// let quantity: Number = "5".parse().unwrap();
    /// let price: Number = "144.00".parse().unwrap();
    /// let stop_price: Number = "145.00".parse().unwrap();
    /// let order = OrderBuilder::stop_limit_sell("AAPL", quantity, price, stop_price);
    /// ```
    pub fn stop_limit_sell(
        symbol: impl Into<String>,
        quantity: Number,
        price: Number,
        stop_price: Number,
    ) -> Self {
        Self::equity_stop_limit(symbol, Instruction::Sell, quantity, price, stop_price)
    }

    /// Build a market buy-to-open order for a single option leg.
    ///
    /// # Arguments
    ///
    /// - `symbol` - Schwab option symbol copied exactly as provided.
    /// - `quantity` - Number of option contracts to buy to open.
    ///
    /// # Defaults
    ///
    /// Sets [`Session::Normal`], [`Duration::Day`], and
    /// [`OrderStrategyType::Single`].
    ///
    /// # Payload
    ///
    /// Emits `orderType=MARKET`, `instruction=BUY_TO_OPEN`, and
    /// `assetType=OPTION`. No `price` or `stopPrice` field is included.
    /// The option symbol is not parsed, formatted, trimmed, or normalized.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Number, OrderBuilder};
    ///
    /// let quantity: Number = "1".parse().unwrap();
    /// let order = OrderBuilder::option_buy_to_open_market(
    ///     "AAPL  260116C00150000",
    ///     quantity,
    /// );
    /// ```
    pub fn option_buy_to_open_market(symbol: impl Into<String>, quantity: Number) -> Self {
        Self::option_market(symbol, Instruction::BuyToOpen, quantity)
    }

    /// Build a limit buy-to-open order for a single option leg.
    ///
    /// # Arguments
    ///
    /// - `symbol` - Schwab option symbol copied exactly as provided.
    /// - `quantity` - Number of option contracts to buy to open.
    /// - `price` - Limit price for the option order.
    ///
    /// # Defaults
    ///
    /// Sets [`Session::Normal`], [`Duration::Day`], and
    /// [`OrderStrategyType::Single`].
    ///
    /// # Payload
    ///
    /// Emits `orderType=LIMIT`, `instruction=BUY_TO_OPEN`,
    /// `assetType=OPTION`, and `price`. No `stopPrice` field is included.
    /// The option symbol is not parsed, formatted, trimmed, or normalized.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Number, OrderBuilder};
    ///
    /// let quantity: Number = "1".parse().unwrap();
    /// let price: Number = "2.50".parse().unwrap();
    /// let order = OrderBuilder::option_buy_to_open_limit(
    ///     "AAPL  260116C00150000",
    ///     quantity,
    ///     price,
    /// );
    /// ```
    pub fn option_buy_to_open_limit(
        symbol: impl Into<String>,
        quantity: Number,
        price: Number,
    ) -> Self {
        Self::option_limit(symbol, Instruction::BuyToOpen, quantity, price)
    }

    /// Build a market sell-to-open order for a single option leg.
    ///
    /// # Arguments
    ///
    /// - `symbol` - Schwab option symbol copied exactly as provided.
    /// - `quantity` - Number of option contracts to sell to open.
    ///
    /// # Defaults
    ///
    /// Sets [`Session::Normal`], [`Duration::Day`], and
    /// [`OrderStrategyType::Single`].
    ///
    /// # Payload
    ///
    /// Emits `orderType=MARKET`, `instruction=SELL_TO_OPEN`, and
    /// `assetType=OPTION`. No `price` or `stopPrice` field is included.
    /// The option symbol is not parsed, formatted, trimmed, or normalized.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Number, OrderBuilder};
    ///
    /// let quantity: Number = "1".parse().unwrap();
    /// let order = OrderBuilder::option_sell_to_open_market(
    ///     "AAPL  260116C00150000",
    ///     quantity,
    /// );
    /// ```
    pub fn option_sell_to_open_market(symbol: impl Into<String>, quantity: Number) -> Self {
        Self::option_market(symbol, Instruction::SellToOpen, quantity)
    }

    /// Build a limit sell-to-open order for a single option leg.
    ///
    /// # Arguments
    ///
    /// - `symbol` - Schwab option symbol copied exactly as provided.
    /// - `quantity` - Number of option contracts to sell to open.
    /// - `price` - Limit price for the option order.
    ///
    /// # Defaults
    ///
    /// Sets [`Session::Normal`], [`Duration::Day`], and
    /// [`OrderStrategyType::Single`].
    ///
    /// # Payload
    ///
    /// Emits `orderType=LIMIT`, `instruction=SELL_TO_OPEN`,
    /// `assetType=OPTION`, and `price`. No `stopPrice` field is included.
    /// The option symbol is not parsed, formatted, trimmed, or normalized.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Number, OrderBuilder};
    ///
    /// let quantity: Number = "1".parse().unwrap();
    /// let price: Number = "2.50".parse().unwrap();
    /// let order = OrderBuilder::option_sell_to_open_limit(
    ///     "AAPL  260116C00150000",
    ///     quantity,
    ///     price,
    /// );
    /// ```
    pub fn option_sell_to_open_limit(
        symbol: impl Into<String>,
        quantity: Number,
        price: Number,
    ) -> Self {
        Self::option_limit(symbol, Instruction::SellToOpen, quantity, price)
    }

    /// Build a market buy-to-close order for a single option leg.
    ///
    /// # Arguments
    ///
    /// - `symbol` - Schwab option symbol copied exactly as provided.
    /// - `quantity` - Number of option contracts to buy to close.
    ///
    /// # Defaults
    ///
    /// Sets [`Session::Normal`], [`Duration::Day`], and
    /// [`OrderStrategyType::Single`].
    ///
    /// # Payload
    ///
    /// Emits `orderType=MARKET`, `instruction=BUY_TO_CLOSE`, and
    /// `assetType=OPTION`. No `price` or `stopPrice` field is included.
    /// The option symbol is not parsed, formatted, trimmed, or normalized.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Number, OrderBuilder};
    ///
    /// let quantity: Number = "1".parse().unwrap();
    /// let order = OrderBuilder::option_buy_to_close_market(
    ///     "AAPL  260116C00150000",
    ///     quantity,
    /// );
    /// ```
    pub fn option_buy_to_close_market(symbol: impl Into<String>, quantity: Number) -> Self {
        Self::option_market(symbol, Instruction::BuyToClose, quantity)
    }

    /// Build a limit buy-to-close order for a single option leg.
    ///
    /// # Arguments
    ///
    /// - `symbol` - Schwab option symbol copied exactly as provided.
    /// - `quantity` - Number of option contracts to buy to close.
    /// - `price` - Limit price for the option order.
    ///
    /// # Defaults
    ///
    /// Sets [`Session::Normal`], [`Duration::Day`], and
    /// [`OrderStrategyType::Single`].
    ///
    /// # Payload
    ///
    /// Emits `orderType=LIMIT`, `instruction=BUY_TO_CLOSE`,
    /// `assetType=OPTION`, and `price`. No `stopPrice` field is included.
    /// The option symbol is not parsed, formatted, trimmed, or normalized.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Number, OrderBuilder};
    ///
    /// let quantity: Number = "1".parse().unwrap();
    /// let price: Number = "2.50".parse().unwrap();
    /// let order = OrderBuilder::option_buy_to_close_limit(
    ///     "AAPL  260116C00150000",
    ///     quantity,
    ///     price,
    /// );
    /// ```
    pub fn option_buy_to_close_limit(
        symbol: impl Into<String>,
        quantity: Number,
        price: Number,
    ) -> Self {
        Self::option_limit(symbol, Instruction::BuyToClose, quantity, price)
    }

    /// Build a market sell-to-close order for a single option leg.
    ///
    /// # Arguments
    ///
    /// - `symbol` - Schwab option symbol copied exactly as provided.
    /// - `quantity` - Number of option contracts to sell to close.
    ///
    /// # Defaults
    ///
    /// Sets [`Session::Normal`], [`Duration::Day`], and
    /// [`OrderStrategyType::Single`].
    ///
    /// # Payload
    ///
    /// Emits `orderType=MARKET`, `instruction=SELL_TO_CLOSE`, and
    /// `assetType=OPTION`. No `price` or `stopPrice` field is included.
    /// The option symbol is not parsed, formatted, trimmed, or normalized.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Number, OrderBuilder};
    ///
    /// let quantity: Number = "1".parse().unwrap();
    /// let order = OrderBuilder::option_sell_to_close_market(
    ///     "AAPL  260116C00150000",
    ///     quantity,
    /// );
    /// ```
    pub fn option_sell_to_close_market(symbol: impl Into<String>, quantity: Number) -> Self {
        Self::option_market(symbol, Instruction::SellToClose, quantity)
    }

    /// Build a limit sell-to-close order for a single option leg.
    ///
    /// # Arguments
    ///
    /// - `symbol` - Schwab option symbol copied exactly as provided.
    /// - `quantity` - Number of option contracts to sell to close.
    /// - `price` - Limit price for the option order.
    ///
    /// # Defaults
    ///
    /// Sets [`Session::Normal`], [`Duration::Day`], and
    /// [`OrderStrategyType::Single`].
    ///
    /// # Payload
    ///
    /// Emits `orderType=LIMIT`, `instruction=SELL_TO_CLOSE`,
    /// `assetType=OPTION`, and `price`. No `stopPrice` field is included.
    /// The option symbol is not parsed, formatted, trimmed, or normalized.
    ///
    /// # Examples
    ///
    /// ```
    /// use schwab::{Number, OrderBuilder};
    ///
    /// let quantity: Number = "1".parse().unwrap();
    /// let price: Number = "2.50".parse().unwrap();
    /// let order = OrderBuilder::option_sell_to_close_limit(
    ///     "AAPL  260116C00150000",
    ///     quantity,
    ///     price,
    /// );
    /// ```
    pub fn option_sell_to_close_limit(
        symbol: impl Into<String>,
        quantity: Number,
        price: Number,
    ) -> Self {
        Self::option_limit(symbol, Instruction::SellToClose, quantity, price)
    }
}
