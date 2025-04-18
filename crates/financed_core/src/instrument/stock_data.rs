#[derive(Debug, Clone)]
#[derive(PartialEq, PartialOrd)]
pub struct DividendYield(pub f64);

#[derive(Debug, Clone)]
#[derive(PartialEq, PartialOrd)]
pub struct HistoricalVolatility(pub f64);

#[derive(Debug, Clone)]
pub struct StockData {
    pub dividend_yield: DividendYield,
    pub historical_volatility: HistoricalVolatility,
}
