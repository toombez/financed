use wasm_newtype::prelude::*;
use wasm_newtype_proc_macro::wasm_newtype;

use super::currency::Currency;

#[derive(Debug, Clone, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[wasm_newtype]
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
pub struct InstrumentMetadata {
    pub id: InstrumentId,
    pub name: InstrumentName,
    pub exchange: Exchange,
    pub currency: Currency,
    pub price: Price,
    pub lot_size: LotSize,
}
