
use derivative_data::DerivativeData;
use financial_instrument::FinancialInstrument;

pub mod financial_instrument;
pub mod derivative_data;

#[derive(Debug, Clone)]
pub enum InstrumentType {
    Stock,
    Option(DerivativeData),
    Future(DerivativeData),
}

#[derive(Debug, Clone)]
pub struct Instrument {
    pub base: FinancialInstrument,
    pub instrument_type: InstrumentType,
}
