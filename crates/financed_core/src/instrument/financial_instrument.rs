#[derive(Debug, Clone, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct InstrumentId(pub String);

#[derive(Debug, Clone, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct InstrumentName(pub String);

#[derive(Debug, Clone, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Currency(pub String);

#[derive(Debug, Clone, Copy, Hash)]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct LotSize(pub u32);

#[derive(Debug, Clone, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Exchange(pub String);

#[derive(Debug, Clone)]
pub struct FinancialInstrument {
    pub id: InstrumentId,
    pub name: InstrumentName,
    pub currency: Currency,
    pub lot_size: LotSize,
    pub exchange: Option<Exchange>,
}
