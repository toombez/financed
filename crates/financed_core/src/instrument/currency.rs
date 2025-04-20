use wasm_newtype::prelude::*;
use wasm_newtype_proc_macro::wasm_newtype;

#[derive(Debug, Clone, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[wasm_newtype]
pub struct CurrencyId {
    value: String,
}

#[derive(Debug, Clone, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[wasm_newtype]
pub struct CurrencyName {
    value: String,
}

#[derive(Debug, Clone)]
#[derive(PartialEq, PartialOrd)]
#[wasm_newtype]
pub struct RiskFreeRate {
    value: f64,
}

#[derive(Debug, Clone)]
pub struct MoneyCurrencyData {
    pub id: CurrencyId,
    pub name: CurrencyName,
    pub risk_free_rate: RiskFreeRate,
}

#[derive(Debug, Clone)]
pub enum Currency {
    Money(MoneyCurrencyData),
    // Crypto,
}
