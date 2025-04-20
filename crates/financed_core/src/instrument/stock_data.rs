use wasm_newtype::prelude::*;
use wasm_newtype_proc_macro::wasm_newtype;

#[derive(Debug, Clone)]
#[derive(PartialEq, PartialOrd)]
#[wasm_newtype]
pub struct DividendYield {
    value: f64,
}

#[derive(Debug, Clone)]
#[derive(PartialEq, PartialOrd)]
#[wasm_newtype]
pub struct HistoricalVolatility {
    value: f64,
}

#[derive(Debug, Clone)]
pub struct StockData {
    pub dividend_yield: DividendYield,
    pub historical_volatility: HistoricalVolatility,
}
