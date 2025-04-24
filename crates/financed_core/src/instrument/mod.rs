pub mod currency;
pub mod instrument_metadata;
pub mod stock_data;
pub mod option_data;

use instrument_metadata::InstrumentMetadata;
use option_data::OptionData;
use serde::{Deserialize, Serialize};
use stock_data::StockData;
use wasm_bindgen::prelude::wasm_bindgen;
use tsify::Tsify;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum InstrumentData {
    Stock(StockData),
    Option(OptionData),
}

#[wasm_bindgen]
impl StockData {
    #[wasm_bindgen]
    pub fn stock_instrument(self) -> InstrumentData {
        InstrumentData::Stock(self)
    }
}

impl Into<InstrumentData> for StockData {
    fn into(self) -> InstrumentData {
        InstrumentData::Stock(self)
    }
}

#[wasm_bindgen]
impl OptionData {
    #[wasm_bindgen]
    pub fn option_instrument(self) -> InstrumentData {
        InstrumentData::Option(self)
    }
}

impl Into<InstrumentData> for OptionData {
    fn into(self) -> InstrumentData {
        InstrumentData::Option(self)
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct Instrument {
    pub metadata: InstrumentMetadata,
    pub data: InstrumentData,
}

#[wasm_bindgen]
impl Instrument {
    #[wasm_bindgen(constructor)]
    pub fn new(metadata: InstrumentMetadata,
        data: InstrumentData,) -> Self {
            Self {
                metadata,
                data,
            }
        }
}
