use chrono::{Months, NaiveDate};
use statrs::function::erf::erf;

pub fn norm_cdf(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / 2.0f64.sqrt()))
}

pub fn norm_pdf(x: f64) -> f64 {
    (-x.powi(2) / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt()
}

pub fn calculate_days_per_year(from_date: NaiveDate, to_date: NaiveDate) -> f64 {
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

pub fn calculate_time_to_expiry(from_date: NaiveDate, to_date: NaiveDate) -> f64 {
    let days = to_date.signed_duration_since(from_date).num_days() as f64;
    let years = calculate_days_per_year(from_date, to_date);

    days / years
}
