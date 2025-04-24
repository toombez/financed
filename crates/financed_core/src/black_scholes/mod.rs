use std::sync::LazyLock;

use anyhow::Result;
use chrono::NaiveDateTime;
use regex::Regex;
use crate::utils::calculate_time_to_expiry;
use wasm_newtype_proc_macro::wasm_newtype;
use wasm_newtype::prelude::*;

use crate::instrument::{currency::Currency, option_data::OptionType, Instrument, InstrumentData};
use crate::utils::norm_cdf;

static EXPIRATION_DATE_STRING_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2}):(\d{2})").unwrap()
});

#[derive(Debug, Clone)]
#[wasm_newtype]
pub struct FromDate {
    #[validate(regex(path = *EXPIRATION_DATE_STRING_REGEX, message = "From date must be formatted with `%Y-%m-%d %H:%M:%S`"))]
    value: String,
}

impl Into<NaiveDateTime> for FromDate {
    fn into(self) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(&self.get_value(), "%Y-%m-%d %H:%M:%S").unwrap()
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct BlackScholesSettings {
    pub from_date: FromDate,
}

#[wasm_bindgen]
impl BlackScholesSettings {
    #[wasm_bindgen(constructor)]
    pub fn new(from_date: FromDate) -> Self {
        Self { from_date }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct BlackScholes {
    pub settings: BlackScholesSettings,
}

#[wasm_bindgen]
impl BlackScholes {
    #[wasm_bindgen(constructor)]
    pub fn new(settings: BlackScholesSettings) -> Self {
        Self {
            settings
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub enum BlackScholesError {
    InvalidInstrumentType,
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct BlackScholesContract {
    pub underlying_asset: Instrument,
    pub option: Instrument,
}

#[wasm_bindgen]
impl BlackScholesContract {
    #[wasm_bindgen(constructor)]
    pub fn new(option: Instrument, underlying_asset: Instrument) -> Self {
        Self {
            option,
            underlying_asset,
        }
    }
}

#[wasm_bindgen]
impl BlackScholes {
    #[wasm_bindgen]
    pub fn calculate_price(&self, contract: &BlackScholesContract) -> Result<f64, BlackScholesError> {
        let spot_data = match &contract.underlying_asset.data {
            InstrumentData::Stock(data) => data,
            _ => return Err(BlackScholesError::InvalidInstrumentType),
        };

        let option_data = match &contract.option.data {
            InstrumentData::Option(option) => option,
            _ => return Err(BlackScholesError::InvalidInstrumentType),
        };

        let spot_price = &contract.underlying_asset.metadata.price;
        let strike_price = &option_data.strike_price;

        let volatility = &option_data.implied_volatility;
        let dividend_yield = &spot_data.dividend_yield;
        let risk_free_rate = match &contract.underlying_asset.metadata.currency {
            Currency::Money(money) => &money.risk_free_rate,
            _ => return Err(BlackScholesError::InvalidInstrumentType),
        };

        let from_date: NaiveDateTime = self.settings.from_date.clone().into();
        let expiration_date: NaiveDateTime = option_data.expiration_date.clone().into();

        let time_delta_in_years = calculate_time_to_expiry(
            from_date.into(),
            expiration_date.into(),
        );

        let (d1, d2) = Self::calculate_d(
            spot_price.get_value(),
            strike_price.get_value(),
            time_delta_in_years,
            risk_free_rate.get_value(),
            volatility.get_value(),
            None,
        );

        let f = spot_price.get_value()
            * ((risk_free_rate.get_value() - dividend_yield.get_value()) * time_delta_in_years)
                .exp();

        match option_data.option_type {
            OptionType::Call => {
                Ok((-risk_free_rate.get_value() * time_delta_in_years).exp()
                    * (f * norm_cdf(d1) - strike_price.get_value() * norm_cdf(d2)))
            }
            OptionType::Put => {
                Ok((-risk_free_rate.get_value() * time_delta_in_years).exp()
                    * (strike_price.get_value() * norm_cdf(-d2) - f * norm_cdf(-d1)))
            }
        }
    }

    fn calculate_d(
        // S
        spot_price: f64,
        // K
        strike_price: f64,
        // tau
        time_delta_in_years: f64,
        // r
        risk_free_rate: f64,
        // sigma
        volatility: f64,
        dividend_yield: Option<f64>,
    ) -> (f64, f64) {
        // q
        let dividend_yield = dividend_yield.unwrap_or(0.0);

        let d1 = ((spot_price / strike_price).ln()
            + (risk_free_rate - dividend_yield + 0.5 * volatility.powi(2)) * time_delta_in_years)
            / (volatility * time_delta_in_years.sqrt());
        let d2 = d1 - volatility * time_delta_in_years.sqrt();

        (d1, d2)
    }
}
