use statrs::function::erf::erf;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn norm_cdf(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / 2.0f64.sqrt()))
}

#[wasm_bindgen]
pub fn norm_pdf(x: f64) -> f64 {
    (-x.powi(2) / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt()
}
