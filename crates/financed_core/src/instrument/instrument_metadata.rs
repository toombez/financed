use super::currency::Currency;

#[derive(Debug, Clone, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct InstrumentId(pub String);

#[derive(Debug, Clone, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct InstrumentName(pub String);

#[derive(Debug, Clone, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Exchange(pub String);

#[derive(Debug, Clone)]
#[derive(PartialEq, PartialOrd)]
pub struct Price(pub f64);

#[derive(Debug, Clone)]
#[derive(PartialEq, PartialOrd)]
pub struct LotSize(pub f64);

#[derive(Debug, Clone)]
pub struct InstrumentMetadata {
    pub id: InstrumentId,
    pub name: InstrumentName,
    pub exchange: Exchange,
    pub currency: Currency,
    pub price: Price,
    pub lot_size: LotSize,
}
