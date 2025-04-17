use chrono::NaiveDateTime;

#[derive(Debug, Clone, Copy, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct ExpirationDate(pub NaiveDateTime);

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, PartialOrd)]
pub struct StrikePrice(pub f64);

#[derive(Debug, Clone, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct UnderlyingAsset(pub String);

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, PartialOrd)]
pub struct ImpliedVolatility(pub f64);

#[derive(Debug, Clone)]
pub struct DerivativeData {
    pub expiration_date: ExpirationDate,
    pub strike_price: Option<StrikePrice>,
    pub underlying_asset: UnderlyingAsset,
    pub implied_volatility: Option<ImpliedVolatility>,
}
