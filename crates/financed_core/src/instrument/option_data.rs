use wasm_newtype::prelude::*;
use wasm_newtype_proc_macro::wasm_newtype;
use chrono::NaiveDateTime;
use std::sync::LazyLock;
use regex::Regex;

use super::instrument_metadata::InstrumentId;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy, Hash)]
#[wasm_bindgen]
pub enum OptionType {
    Call,
    Put,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy, Hash)]
#[wasm_bindgen]
pub enum ExerciceStyle {
    American,
    European,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
#[wasm_newtype]
pub struct StrikePrice {
    value: f64,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
#[wasm_newtype]
pub struct ImpliedVolatility {
    value: f64,
}

static EXPIRATION_DATE_STRING_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2}):(\d{2})").unwrap()
});

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct OptionData {
    pub option_type: OptionType,
    pub option_style: ExerciceStyle,

    pub strike_price: StrikePrice,
    pub implied_volatility: ImpliedVolatility,
    pub expiration_date: ExpirationDate,
    pub underlying_asset: InstrumentId,
}

#[wasm_bindgen]
impl OptionData {
    #[wasm_bindgen(constructor)]
    pub fn new(option_type: OptionType,
        option_style: ExerciceStyle,
        strike_price: StrikePrice,
        implied_volatility: ImpliedVolatility,
        expiration_date: ExpirationDate,
        underlying_asset: InstrumentId
    ) -> Self {
            Self {
                expiration_date,
                implied_volatility,
                option_style,
                option_type,
                strike_price,
                underlying_asset,
        }
    }
}
