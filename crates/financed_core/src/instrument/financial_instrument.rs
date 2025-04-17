#[derive(Debug, Clone, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct InstrumentId(String);

#[derive(Debug, Clone, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct InstrumentName(String);

#[derive(Debug, Clone, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Currency(String);

#[derive(Debug, Clone, Copy, Hash)]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct LotSize(u32);

#[derive(Debug, Clone, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Exchange(String);

#[derive(Debug, Clone)]
pub struct FinancialInstrument {
    pub id: InstrumentId,
    pub name: InstrumentName,
    pub currency: Currency,
    pub lot_size: LotSize,
    pub exchange: Option<Exchange>,
}
