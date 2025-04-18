#[derive(Debug, Clone, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CurrencyId(pub String);

#[derive(Debug, Clone, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CurrencyName(pub String);

#[derive(Debug, Clone)]
#[derive(PartialEq, PartialOrd)]
pub struct RiskFreeRate(pub f64);

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
