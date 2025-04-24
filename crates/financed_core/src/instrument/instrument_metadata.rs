use wasm_newtype::prelude::*;
use wasm_newtype_proc_macro::wasm_newtype;

use super::currency::Currency;

#[derive(Debug, Clone, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[wasm_newtype]
#[derive(Serialize, Deserialize)]
pub struct InstrumentId {
    value: String,
}

#[derive(Debug, Clone, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[wasm_newtype]
pub struct InstrumentName {
    value: String,
}

#[derive(Debug, Clone, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[wasm_newtype]
pub struct Exchange {
    value: String,
}

#[derive(Debug, Clone)]
#[derive(PartialEq, PartialOrd)]
#[wasm_newtype]
pub struct Price {
    value: f64,
}

#[derive(Debug, Clone)]
#[derive(PartialEq, PartialOrd)]
#[wasm_newtype]
pub struct LotSize {
    value: f64,
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct InstrumentMetadata {
    pub id: InstrumentId,
    pub name: InstrumentName,
    pub exchange: Exchange,
    pub currency: Currency,
    pub price: Price,
    pub lot_size: LotSize,
}

#[wasm_bindgen]
impl InstrumentMetadata {
    #[wasm_bindgen(constructor)]
    pub fn new(id: InstrumentId,
        name: InstrumentName,
        exchange: Exchange,
        currency: Currency,
        price: Price,
        lot_size: LotSize,
    ) -> Self {
        Self {
            currency,
            exchange,
            id,
            lot_size,
            name,
            price,
        }
    }
}
