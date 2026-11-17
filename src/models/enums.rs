use serde::{Deserialize, Serialize};

// --- Market Data Enums ---

/// Primary asset classification.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum AssetMainType {
    Bond,
    Equity,
    Forex,
    Future,
    FutureOption,
    Index,
    MutualFund,
    Option,
}

/// Market data quote type classification.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum MarketType {
    Bond,
    Equity,
    Etf,
    Extended,
    Forex,
    Future,
    FutureOption,
    Fundamental,
    Index,
    Indicator,
    MutualFund,
    Option,
    Unknown,
}

/// Option contract type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ContractType {
    #[serde(rename = "P")]
    Put,
    #[serde(rename = "C")]
    Call,
}

/// Equity asset subtype classification.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum EquityAssetSubType {
    #[serde(rename = "COE")]
    CommonEquity,
    #[serde(rename = "PRF")]
    Preferred,
    #[serde(rename = "ADR")]
    Adr,
    #[serde(rename = "GDR")]
    Gdr,
    #[serde(rename = "CEF")]
    ClosedEndFund,
    #[serde(rename = "ETF")]
    Etf,
    #[serde(rename = "ETN")]
    Etn,
    #[serde(rename = "UIT")]
    Uit,
    #[serde(rename = "WAR")]
    Warrant,
    #[serde(rename = "RGT")]
    Right,
}

/// Market data error status.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum MarketErrorStatus {
    #[serde(rename = "400")]
    BadRequest,
    #[serde(rename = "401")]
    Unauthorized,
    #[serde(rename = "404")]
    NotFound,
    #[serde(rename = "500")]
    InternalServerError,
}

/// Option exercise style.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ExerciseType {
    #[serde(rename = "A")]
    American,
    #[serde(rename = "E")]
    European,
}

/// Option expiration schedule type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ExpirationType {
    #[serde(rename = "M")]
    Monthly,
    #[serde(rename = "Q")]
    Quarterly,
    #[serde(rename = "S")]
    Standard,
    #[serde(rename = "W")]
    Weekly,
}

/// Fund investment strategy.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum FundStrategy {
    #[serde(rename = "A")]
    Active,
    #[serde(rename = "L")]
    Leveraged,
    #[serde(rename = "P")]
    Passive,
    #[serde(rename = "Q")]
    Quantitative,
    #[serde(rename = "S")]
    Short,
}

/// Mutual fund asset subtype classification.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum MutualFundAssetSubType {
    Oef,
    Cef,
    Mmf,
}

/// Option chain strategy classification.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum OptionStrategy {
    Single,
    Analytical,
    Covered,
    Vertical,
    Calendar,
    Strangle,
    Straddle,
    Butterfly,
    Condor,
    Diagonal,
    Collar,
    Roll,
}

/// Put or call indicator.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum PutCall {
    Put,
    Call,
}

/// Quote feed type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum QuoteType {
    Nbbo,
    Nfl,
}

/// Price movement direction.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum Direction {
    Up,
    Down,
}

/// Option settlement timing.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum SettlementType {
    #[serde(rename = "A")]
    Am,
    #[serde(rename = "P")]
    Pm,
}

/// Exchange identifier.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ExchangeName {
    Ind,
    Ase,
    Nys,
    #[serde(alias = "NASDAQ")]
    Nas,
    Nap,
    Pac,
    Opr,
    Bats,
}

/// Market type for market hours queries.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum MarketDataMarket {
    Equity,
    Option,
    Bond,
    Future,
    Forex,
}

/// Mover index or symbol group.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum MoverSymbol {
    #[serde(rename = "$DJI")]
    Dji,
    #[serde(rename = "$COMPX")]
    Compx,
    #[serde(rename = "$SPX")]
    Spx,
    Nyse,
    Nasdaq,
    Otcbb,
    IndexAll,
    EquityAll,
    OptionAll,
    OptionPut,
    OptionCall,
}

/// Option contract filter type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum OptionContractType {
    Call,
    Put,
    All,
}

/// Market data entitlement classification.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum Entitlement {
    Pn,
    Np,
    Pp,
}

/// Option expiration month filter.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ExpirationMonth {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
    All,
}

/// Mover sort criterion.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum Sort {
    Volume,
    Trades,
    PercentChangeUp,
    PercentChangeDown,
}

/// Price history candle frequency unit.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum FrequencyType {
    Minute,
    Daily,
    Weekly,
    Monthly,
}

/// Price history lookback period unit.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum PeriodType {
    Day,
    Month,
    Year,
    Ytd,
}

/// Instrument search projection type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum Projection {
    #[serde(rename = "symbol-search")]
    SymbolSearch,
    #[serde(rename = "symbol-regex")]
    SymbolRegex,
    #[serde(rename = "desc-search")]
    DescSearch,
    #[serde(rename = "desc-regex")]
    DescRegex,
    #[serde(rename = "search")]
    Search,
    #[serde(rename = "fundamental")]
    Fundamental,
}

// --- Trader API Enums ---

/// API rule action outcome.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ApiRuleAction {
    Accept,
    Alert,
    Reject,
    Review,
    #[serde(other)]
    Unknown,
}

/// Currency type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ApiCurrencyType {
    Usd,
    Cad,
    Eur,
    Jpy,
    /// Any value this crate does not know yet (Schwab adds them without notice).
    #[serde(other)]
    Unknown,
}

/// Cash equivalent instrument type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum CashEquivalentType {
    SweepVehicle,
    Savings,
    MoneyMarketFund,
    Unknown,
}

/// Option put or call indicator.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum OptionPutCall {
    Put,
    Call,
    Unknown,
}

/// Option instrument type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum OptionType {
    Vanilla,
    Binary,
    Barrier,
    Unknown,
}

/// Instrument asset classification.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum InstrumentAssetType {
    Equity,
    Option,
    Index,
    MutualFund,
    CashEquivalent,
    FixedIncome,
    Currency,
    CollectiveInvestment,
}

/// Collective investment instrument type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum CollectiveInvestmentType {
    UnitInvestmentTrust,
    ExchangeTradedFund,
    ClosedEndFund,
    Index,
    Units,
}

/// Fee classification.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum FeeType {
    Commission,
    SecFee,
    StrFee,
    RFee,
    CdscFee,
    OptRegFee,
    AdditionalFee,
    MiscellaneousFee,
    Ftt,
    FuturesClearingFee,
    FuturesDeskOfficeFee,
    FuturesExchangeFee,
    FuturesGlobexFee,
    FuturesNfaFee,
    FuturesPitBrokerageFee,
    FuturesTransactionFee,
    LowProceedsCommission,
    BaseCharge,
    GeneralCharge,
    GstFee,
    TafFee,
    IndexOptionFee,
    TefraTax,
    StateTax,
    #[serde(other)]
    Unknown,
}

/// Forex instrument type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ForexType {
    Standard,
    Nbbo,
    Unknown,
}

/// Future instrument type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum FutureType {
    Standard,
    Unknown,
}

/// Index classification.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum IndexType {
    BroadBased,
    NarrowBased,
    Unknown,
}

/// Order activity classification.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum OrderActivityType {
    Execution,
    OrderAction,
    /// Any value this crate does not know yet (Schwab adds them without notice).
    #[serde(other)]
    Unknown,
}

/// Order execution activity type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ExecutionType {
    Fill,
    Canceled,
    #[serde(other)]
    Unknown,
}

/// Dividend and capital gains handling method.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum DividendCapitalGains {
    Reinvest,
    Payout,
    /// Any value this crate does not know yet (Schwab adds them without notice).
    #[serde(other)]
    Unknown,
}

/// Position opening or closing effect.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum PositionEffect {
    Opening,
    Closing,
    Automatic,
    #[serde(other)]
    Unknown,
}

/// Order quantity unit.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum QuantityType {
    AllShares,
    Dollars,
    Shares,
    /// Any value this crate does not know yet (Schwab adds them without notice).
    #[serde(other)]
    Unknown,
}

/// Advanced order strategy type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum AdvancedOrderType {
    None,
    Oto,
    Oco,
    Otoco,
    #[serde(rename = "OT2OCO")]
    Ot2Oco,
    #[serde(rename = "OT3OCO")]
    Ot3Oco,
    BlastAll,
    Ota,
    Pair,
    /// Any value this crate does not know yet (Schwab adds them without notice).
    #[serde(other)]
    Unknown,
}

/// Product instrument type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ProductType {
    Tbd,
    Unknown,
}

/// Securities account type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum SecuritiesAccountType {
    Cash,
    Margin,
    /// Any value this crate does not know yet (Schwab adds them without notice).
    #[serde(other)]
    Unknown,
}

/// Transaction activity classification.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum TransactionActivityType {
    ActivityCorrection,
    Execution,
    OrderAction,
    Transfer,
    #[serde(other)]
    Unknown,
}

/// Transaction lifecycle status.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum TransactionStatus {
    Valid,
    Invalid,
    Pending,
    #[serde(other)]
    Unknown,
}

/// Account sub-ledger classification.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum SubAccount {
    Cash,
    Margin,
    Short,
    Div,
    Income,
    #[serde(other)]
    Unknown,
}

/// Transaction equity instrument type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum TransactionEquityType {
    CommonStock,
    PreferredStock,
    DepositoryReceipt,
    PreferredDepositoryReceipt,
    RestrictedStock,
    ComponentUnit,
    Right,
    Warrant,
    ConvertiblePreferredStock,
    ConvertibleStock,
    LimitedPartnership,
    WhenIssued,
    Unknown,
}

/// Transaction fixed income instrument type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum TransactionFixedIncomeType {
    BondUnit,
    CertificateOfDeposit,
    ConvertibleBond,
    CollateralizedMortgageObligation,
    CorporateBond,
    GovernmentMortgage,
    GnmaBonds,
    MunicipalAssessmentDistrict,
    MunicipalBond,
    OtherGovernment,
    ShortTermPaper,
    UsTreasuryBond,
    UsTreasuryBill,
    UsTreasuryNote,
    UsTreasuryZeroCoupon,
    AgencyBond,
    WhenAsAndIfIssuedBond,
    AssetBackedSecurity,
    Unknown,
}

/// Transaction mutual fund instrument type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum TransactionMutualFundType {
    NotApplicable,
    OpenEndNonTaxable,
    OpenEndTaxable,
    NoLoadNonTaxable,
    NoLoadTaxable,
    Unknown,
}

/// Transaction classification.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum TransactionType {
    Trade,
    ReceiveAndDeliver,
    DividendOrInterest,
    AchReceipt,
    AchDisbursement,
    CashReceipt,
    CashDisbursement,
    ElectronicFund,
    WireOut,
    WireIn,
    Journal,
    Memorandum,
    MarginCall,
    MoneyMarket,
    SmaAdjustment,
    /// Any value this crate does not know yet (Schwab adds them without notice).
    #[serde(other)]
    Unknown,
}

/// User classification.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum UserType {
    AdvisorUser,
    BrokerUser,
    ClientUser,
    SystemUser,
    #[serde(other)]
    Unknown,
}

/// Amount unit indicator.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum AmountIndicator {
    Dollars,
    Shares,
    AllShares,
    Percentage,
    #[serde(other)]
    Unknown,
}

/// Order lifecycle status.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum OrderStatus {
    AwaitingParentOrder,
    AwaitingCondition,
    AwaitingStopCondition,
    AwaitingManualReview,
    Accepted,
    AwaitingUrOut,
    PendingActivation,
    Queued,
    Working,
    Rejected,
    PendingCancel,
    Canceled,
    PendingReplace,
    Replaced,
    Filled,
    Expired,
    New,
    AwaitingReleaseTime,
    PendingAcknowledgement,
    PendingRecall,
    #[serde(other)]
    Unknown,
}

/// Trader asset classification.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum AssetType {
    Equity,
    MutualFund,
    Option,
    Future,
    Forex,
    Index,
    CashEquivalent,
    FixedIncome,
    Product,
    Currency,
    CollectiveInvestment,
    /// Any value this crate does not know yet (Schwab adds them without notice).
    #[serde(other)]
    Unknown,
}

/// Complex order strategy classification.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ComplexOrderStrategyType {
    None,
    Covered,
    Vertical,
    BackRatio,
    Calendar,
    Diagonal,
    Straddle,
    Strangle,
    CollarSynthetic,
    Butterfly,
    Condor,
    IronCondor,
    VerticalRoll,
    CollarWithStock,
    DoubleDiagonal,
    UnbalancedButterfly,
    UnbalancedCondor,
    UnbalancedIronCondor,
    UnbalancedVerticalRoll,
    MutualFundSwap,
    Custom,
    /// Any value this crate does not know yet (Schwab adds them without notice).
    #[serde(other)]
    Unknown,
}

/// Order time-in-force duration.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum Duration {
    Day,
    GoodTillCancel,
    FillOrKill,
    ImmediateOrCancel,
    EndOfWeek,
    EndOfMonth,
    NextEndOfMonth,
    #[serde(other)]
    Unknown,
}

/// Order instruction (buy, sell, etc.).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum Instruction {
    Buy,
    Sell,
    BuyToCover,
    SellShort,
    BuyToOpen,
    BuyToClose,
    SellToOpen,
    SellToClose,
    Exchange,
    SellShortExempt,
    /// Any value this crate does not know yet (Schwab adds them without notice).
    #[serde(other)]
    Unknown,
}

/// Order strategy classification.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum OrderStrategyType {
    Single,
    Cancel,
    Recall,
    Pair,
    Flatten,
    TwoDaySwap,
    BlastAll,
    Oco,
    Trigger,
    /// Any value this crate does not know yet (Schwab adds them without notice).
    #[serde(other)]
    Unknown,
}

/// Order type classification.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
    TrailingStop,
    Cabinet,
    NonMarketable,
    MarketOnClose,
    Exercise,
    TrailingStopLimit,
    NetDebit,
    NetCredit,
    NetZero,
    LimitOnClose,
    #[serde(other)]
    Unknown,
}

/// Order request type classification.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum OrderTypeRequest {
    Market,
    Limit,
    Stop,
    StopLimit,
    TrailingStop,
    Cabinet,
    NonMarketable,
    MarketOnClose,
    Exercise,
    TrailingStopLimit,
    NetDebit,
    NetCredit,
    NetZero,
    LimitOnClose,
}

/// Order price link reference basis.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum PriceLinkBasis {
    Manual,
    Base,
    Trigger,
    Last,
    Bid,
    Ask,
    AskBid,
    Mark,
    Average,
    /// Any value this crate does not know yet (Schwab adds them without notice).
    #[serde(other)]
    Unknown,
}

/// Order price link adjustment type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum PriceLinkType {
    Value,
    Percent,
    Tick,
    /// Any value this crate does not know yet (Schwab adds them without notice).
    #[serde(other)]
    Unknown,
}

/// Order routing destination.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum RequestedDestination {
    Inet,
    EcnArca,
    Cboe,
    Amex,
    Phlx,
    Ise,
    Box,
    Nyse,
    Nasdaq,
    Bats,
    C2,
    Auto,
    /// Any value this crate does not know yet (Schwab adds them without notice).
    #[serde(other)]
    Unknown,
}

/// Trading session classification.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum Session {
    Normal,
    Am,
    Pm,
    Seamless,
    /// Any value this crate does not know yet (Schwab adds them without notice).
    #[serde(other)]
    Unknown,
}

/// Order settlement instruction.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum SettlementInstruction {
    Regular,
    Cash,
    NextDay,
    #[serde(other)]
    Unknown,
}

/// Order special handling instruction.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum SpecialInstruction {
    AllOrNone,
    DoNotReduce,
    AllOrNoneDoNotReduce,
    /// Any value this crate does not know yet (Schwab adds them without notice).
    #[serde(other)]
    Unknown,
}

/// Stop price link reference basis.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum StopPriceLinkBasis {
    Manual,
    Base,
    Trigger,
    Last,
    Bid,
    Ask,
    AskBid,
    Mark,
    Average,
    /// Any value this crate does not know yet (Schwab adds them without notice).
    #[serde(other)]
    Unknown,
}

/// Stop price link adjustment type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum StopPriceLinkType {
    Value,
    Percent,
    Tick,
    /// Any value this crate does not know yet (Schwab adds them without notice).
    #[serde(other)]
    Unknown,
}

/// Stop trigger price type.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum StopType {
    Standard,
    Bid,
    Ask,
    Last,
    Mark,
    /// Any value this crate does not know yet (Schwab adds them without notice).
    #[serde(other)]
    Unknown,
}

/// Tax lot selection method.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum TaxLotMethod {
    Fifo,
    Lifo,
    HighCost,
    LowCost,
    AverageCost,
    SpecificLot,
    LossHarvester,
    /// Any value this crate does not know yet (Schwab adds them without notice).
    #[serde(other)]
    Unknown,
}
