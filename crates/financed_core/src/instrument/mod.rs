pub mod currency;
pub mod instrument_metadata;
pub mod stock_data;
pub mod option_data;

use instrument_metadata::InstrumentMetadata;
use option_data::OptionData;
use stock_data::StockData;

#[derive(Debug, Clone)]
pub enum InstrumentData {
    Stock(StockData),
    Option(OptionData),
}

#[derive(Debug, Clone)]
pub struct Instrument {
    pub metadata: InstrumentMetadata,
    pub data: InstrumentData,
}
