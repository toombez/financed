use chrono::NaiveDateTime;
use financed_core::instrument::{derivative_data::{DerivativeData, ExpirationDate, ImpliedVolatility, StrikePrice, UnderlyingAsset}, financial_instrument::{Currency, FinancialInstrument, InstrumentId, InstrumentName, LotSize}, Instrument};

fn main() {
    let i1 = Instrument {
        base: FinancialInstrument {
            currency: Currency("USD".to_string()),
            exchange: None,
            id: InstrumentId("MTLR_OPT_2025".to_string()),
            lot_size: LotSize(1),
            name: InstrumentName("Мечел".to_string()),
        },
        instrument_type: financed_core::instrument::InstrumentType::Option(DerivativeData {
            expiration_date: ExpirationDate(NaiveDateTime::parse_from_str("2025-04-17 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()),
            implied_volatility: Some(ImpliedVolatility(1.1)),
            strike_price: Some(StrikePrice(100.0)),
            underlying_asset: UnderlyingAsset("MTLR".to_string()),
        })
    };

    println!("{:?}", i1);
}
