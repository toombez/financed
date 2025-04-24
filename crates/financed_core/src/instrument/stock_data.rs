use wasm_newtype::prelude::*;
use wasm_newtype_proc_macro::wasm_newtype;

#[derive(Debug, Clone)]
#[derive(PartialEq, PartialOrd)]
#[derive(Serialize, Deserialize)]
#[wasm_newtype]
pub struct DividendYield {
    value: f64,
}

#[derive(Debug, Clone)]
#[derive(PartialEq, PartialOrd)]
#[derive(Serialize, Deserialize)]
#[wasm_newtype]
pub struct HistoricalVolatility {
    value: f64,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct StockData {
    pub dividend_yield: DividendYield,
    pub historical_volatility: HistoricalVolatility,
}

#[wasm_bindgen]
impl StockData {
    #[wasm_bindgen(constructor)]
    pub fn new(
        dividend_yield: DividendYield,
        historical_volatility: HistoricalVolatility,
    ) -> Self {
        Self {
            dividend_yield,
            historical_volatility,
        }
    }
}
