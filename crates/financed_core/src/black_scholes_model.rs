use std::str::FromStr;

use chrono::{Months, NaiveDate};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::utils::norm_cdf;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlackScholesModelSettings {
    from_date: NaiveDate,
}

impl Default for BlackScholesModelSettings {
    fn default() -> Self {
        Self {
            from_date: Default::default(),
        }
    }
}

#[wasm_bindgen]
impl BlackScholesModelSettings {
    #[wasm_bindgen(constructor)]
    pub fn new(from_date: Option<String>) -> Self {
        Self {
            from_date: from_date
                .map(|date| NaiveDate::from_str(date.as_str()).unwrap())
                .unwrap_or(NaiveDate::default()),
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptionType {
    Put = 0,
    Call = 1,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExerciseStyle {
    European,
    American,
}

#[wasm_bindgen]
#[derive(Builder, Debug, Clone, Serialize, Deserialize)]
pub struct OptionContract {
    expiration_date: NaiveDate,
    strike_price: f64,
    option_type: OptionType,
    option_style: ExerciseStyle,
}

#[wasm_bindgen]
impl OptionContract {
    #[wasm_bindgen(constructor)]
    pub fn new(
        strike_price: f64,
        option_style: ExerciseStyle,
        option_type: OptionType,
        expiration_date: &str,
    ) -> Self {
        Self {
            strike_price,
            expiration_date: NaiveDate::from_str(&expiration_date).unwrap(),
            option_style,
            option_type,
        }
    }

    #[wasm_bindgen]
    pub fn get_date(&self) -> String {
        self.expiration_date.to_string()
    }

    pub fn set_date(&mut self, date: &str, formatter: &str) {
        self.expiration_date = NaiveDate::parse_from_str(date, formatter).unwrap();
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    spot_price: f64,
    risk_free_rate: f64,
    volatility: f64,
    dividend_yield: f64,
}

#[wasm_bindgen]
impl MarketData {
    #[wasm_bindgen(constructor)]
    pub fn new(spot_price: f64, risk_free_rate: f64, volatility: f64, dividend_yield: f64) -> Self {
        Self {
            dividend_yield,
            risk_free_rate,
            spot_price,
            volatility,
        }
    }
}

#[wasm_bindgen]
pub struct BlackScholesModel {
    settings: BlackScholesModelSettings,
}

#[wasm_bindgen]
impl BlackScholesModel {
    #[wasm_bindgen(constructor)]
    pub fn new(settings: &BlackScholesModelSettings) -> Self {
        Self {
            settings: settings.clone(),
        }
    }

    #[wasm_bindgen]
    pub fn calculate_price(&self, contract: &OptionContract, market_data: &MarketData) -> f64 {
        let time_delta_in_years =
            Self::calculate_time_to_expiry(self.settings.from_date, contract.expiration_date);

        let (d1, d2) = Self::calculate_d(
            market_data.spot_price,
            contract.strike_price,
            time_delta_in_years,
            market_data.risk_free_rate,
            market_data.volatility,
            Some(market_data.dividend_yield),
        );

        let f = market_data.spot_price
            * ((market_data.risk_free_rate - market_data.dividend_yield) * time_delta_in_years)
                .exp();

        match contract.option_type {
            OptionType::Call => {
                (-market_data.risk_free_rate * time_delta_in_years).exp()
                    * (f * norm_cdf(d1) - contract.strike_price * norm_cdf(d2))
            }
            OptionType::Put => {
                (-market_data.risk_free_rate * time_delta_in_years).exp()
                    * (contract.strike_price * norm_cdf(-d2) - f * norm_cdf(-d1))
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

    fn calculate_days_per_year(from_date: NaiveDate, to_date: NaiveDate) -> f64 {
        let years = to_date.years_since(from_date).unwrap_or(1) as f64;
        let years = if years == 0.0 { 1.0 } else { years };

        let from_date_after_years = from_date
            .checked_add_months(Months::new(12 * years as u32))
            .unwrap();

        let days_between_from_date_and_after = from_date_after_years
            .signed_duration_since(from_date)
            .num_days() as f64;

        days_between_from_date_and_after / years
    }

    fn calculate_time_to_expiry(from_date: NaiveDate, to_date: NaiveDate) -> f64 {
        let days = to_date.signed_duration_since(from_date).num_days() as f64;
        let years = Self::calculate_days_per_year(from_date, to_date);

        days / years
    }
}
