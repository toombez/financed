use anyhow::Result;
use chrono::{Months, NaiveDate, NaiveDateTime};

use crate::instrument::{currency::Currency, option_data::OptionType, Instrument, InstrumentData};
use crate::utils::norm_cdf;

#[derive(Debug,)]
pub struct BlackScholesSettings {
    pub from_date: NaiveDateTime,
}

pub struct BlackScholes {
    pub settings: BlackScholesSettings,
}

#[derive(Debug, Clone, Copy)]
pub enum BlackScholesError {
    InvalidInstrumentType,
}

pub struct BlackScholesContract {
    pub underlying_asset: Instrument,
    pub option: Instrument,
}

impl BlackScholes {
    pub fn calculate_price(&self, contract: BlackScholesContract) -> Result<f64, BlackScholesError> {
        let spot_data = match contract.underlying_asset.data {
            InstrumentData::Stock(data) => data,
            _ => return Err(BlackScholesError::InvalidInstrumentType),
        };

        let option_data = match contract.option.data {
            InstrumentData::Option(option) => option,
            _ => return Err(BlackScholesError::InvalidInstrumentType),
        };

        let spot_price = contract.underlying_asset.metadata.price;
        let strike_price = option_data.strike_price;

        let volatility = option_data.implied_volatility;
        let dividend_yield = spot_data.dividend_yield;
        let risk_free_rate = match contract.underlying_asset.metadata.currency {
            Currency::Money(money) => money.risk_free_rate,
            // _ => return Err(BlackScholesError::InvalidInstrumentType),
        };

        let time_delta_in_years = Self::calculate_time_to_expiry(
            self.settings.from_date.date(),
            option_data.expiration_date.0.date()
        );

        let (d1, d2) = Self::calculate_d(
            spot_price.0,
            strike_price.0,
            time_delta_in_years,
            risk_free_rate.0,
            volatility.0,
            None,
        );

        let f = spot_price.0
            * ((risk_free_rate.0 - dividend_yield.0) * time_delta_in_years)
                .exp();

        match option_data.option_type {
            OptionType::Call => {
                Ok((-risk_free_rate.0 * time_delta_in_years).exp()
                    * (f * norm_cdf(d1) - strike_price.0 * norm_cdf(d2)))
            }
            OptionType::Put => {
                Ok((-risk_free_rate.0 * time_delta_in_years).exp()
                    * (strike_price.0 * norm_cdf(-d2) - f * norm_cdf(-d1)))
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
