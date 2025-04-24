use tsify::Tsify;
use wasm_newtype::prelude::*;
use wasm_newtype_proc_macro::wasm_newtype;

#[derive(Debug, Clone, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[wasm_newtype]
pub struct CurrencyId {
    value: String,
}

#[derive(Debug, Clone, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[wasm_newtype]
pub struct CurrencyName {
    value: String,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
#[wasm_newtype]
pub struct RiskFreeRate {
    value: f64,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct MoneyCurrencyData {
    pub id: CurrencyId,
    pub name: CurrencyName,
    pub risk_free_rate: RiskFreeRate,
}

#[wasm_bindgen]
impl MoneyCurrencyData {
    #[wasm_bindgen(constructor)]
    pub fn new(id: CurrencyId, name: CurrencyName, risk_free_rate: RiskFreeRate) -> Self {
        Self {
            id,
            name,
            risk_free_rate,
        }
    }
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Currency {
    Money(MoneyCurrencyData),
    // Crypto,
}

#[wasm_bindgen]
impl MoneyCurrencyData {
    #[wasm_bindgen]
    pub fn money_currency(self) -> Currency {
        self.into()
    }
}

impl Into<Currency> for MoneyCurrencyData {
    fn into(self) -> Currency {
        Currency::Money(self)
    }
}
