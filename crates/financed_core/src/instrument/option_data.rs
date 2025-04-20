use wasm_newtype::prelude::*;
use wasm_newtype_proc_macro::wasm_newtype;
use chrono::NaiveDateTime;
use std::sync::LazyLock;
use regex::Regex;

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

static EXPIRATION_DATE_STRING_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2}):(\d{2})").unwrap()
});

#[derive(Debug, Clone)]
#[derive(PartialEq, PartialOrd)]
#[wasm_newtype]
pub struct ExpirationDate {
    #[validate(regex(path = *EXPIRATION_DATE_STRING_REGEX, message = "Expiration date must be formatted with `%Y-%m-%d %H:%M:%S`"))]
    value: String,
}

impl Into<NaiveDateTime> for ExpirationDate {
    fn into(self) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(&self.value, "%Y-%m-%d %H:%M:%S").unwrap()
    }
}

impl From<NaiveDateTime> for ExpirationDate {
    fn from(value: NaiveDateTime) -> Self {
        Self::new(value.to_string())
    }
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
