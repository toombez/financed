use chrono::NaiveDateTime;

use super::instrument_metadata::InstrumentId;

#[derive(Debug, Clone, Copy, Hash)]
pub enum OptionType {
    Call,
    Put,
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum ExerciceStyle {
    American,
    European,
}

#[derive(Debug, Clone)]
#[derive(PartialEq, PartialOrd)]
pub struct StrikePrice(pub f64);

#[derive(Debug, Clone)]
#[derive(PartialEq, PartialOrd)]
pub struct ImpliedVolatility(pub f64);

#[derive(Debug, Clone)]
#[derive(PartialEq, PartialOrd)]
pub struct ExpirationDate(pub NaiveDateTime);

#[derive(Debug, Clone)]
pub struct OptionData {
    pub option_type: OptionType,
    pub option_style: ExerciceStyle,

    pub strike_price: StrikePrice,
    pub implied_volatility: ImpliedVolatility,
    pub expiration_date: ExpirationDate,
    pub underlying_asset: InstrumentId,
}
