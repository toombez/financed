use statrs::function::erf::erf;

pub fn norm_cdf(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / 2.0f64.sqrt()))
}

pub fn norm_pdf(x: f64) -> f64 {
    (-x.powi(2) / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt()
}
