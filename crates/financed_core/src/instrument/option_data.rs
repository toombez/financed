use wasm_newtype::prelude::*;
use wasm_newtype_proc_macro::wasm_newtype;
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
#[wasm_newtype]
pub struct StrikePrice {
    value: f64,
}

#[derive(Debug, Clone)]
#[derive(PartialEq, PartialOrd)]
#[wasm_newtype]
pub struct ImpliedVolatility {
    value: f64,
}

#[derive(Debug, Clone)]
#[derive(PartialEq, PartialOrd)]
pub struct ExpirationDate {
    pub value: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct OptionData {
    pub option_type: OptionType,
    pub option_style: ExerciceStyle,

    pub strike_price: StrikePrice,
    pub implied_volatility: ImpliedVolatility,
    pub expiration_date: ExpirationDate,
    pub underlying_asset: InstrumentId,
}
