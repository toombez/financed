use chrono::{Days, NaiveDateTime, Utc};
use financed_core::{black_scholes::{BlackScholes, BlackScholesContract, BlackScholesSettings}, instrument::{currency::{CurrencyId, CurrencyName, MoneyCurrencyData, RiskFreeRate}, instrument_metadata::{Exchange, InstrumentId, InstrumentMetadata, InstrumentName, LotSize, Price}, option_data::{ExpirationDate, ImpliedVolatility, OptionData, StrikePrice}, stock_data::{DividendYield, HistoricalVolatility, StockData}, Instrument}};

fn main() {
    let mut rub_currency = financed_core::instrument::currency::Currency::Money(MoneyCurrencyData {
        id: CurrencyId("RUB".to_string()),
        name: CurrencyName("Рубли".to_string()),
        risk_free_rate: RiskFreeRate(0.21),
    });

    let t_invest_exchange = Exchange("T Invest".to_string());

    println!("{rub_currency:?}");

    let mtlr_stock = Instrument {
        data: financed_core::instrument::InstrumentData::Stock(StockData {
            dividend_yield: DividendYield(0.21),
            historical_volatility: HistoricalVolatility(1.1),
        }),
        metadata: financed_core::instrument::instrument_metadata::InstrumentMetadata {
            currency: rub_currency.clone(),
            exchange: t_invest_exchange.clone(),
            id: InstrumentId("MTLR".to_string()),
            lot_size: LotSize(1.0),
            name: InstrumentName("Мечел".to_string()),
            price: Price(94.5),
        }
    };

    println!("{mtlr_stock:?}");

    let lkoh_stock = Instrument {
        data: financed_core::instrument::InstrumentData::Stock(StockData {
            dividend_yield: DividendYield(0.21),
            historical_volatility: HistoricalVolatility(1.1),
        }),
        metadata: financed_core::instrument::instrument_metadata::InstrumentMetadata {
            currency: rub_currency.clone(),
            exchange: t_invest_exchange.clone(),
            id: InstrumentId("LKOH".to_string()),
            lot_size: LotSize(1.0),
            name: InstrumentName("Лукойл".to_string()),
            price: Price(1000.0),
        }
    };

    println!("{lkoh_stock:?}");

    let lkoh_option = Instrument {
        data: financed_core::instrument::InstrumentData::Option(OptionData {
            expiration_date: ExpirationDate(NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap()),
            implied_volatility: ImpliedVolatility(1.1),
            option_style: financed_core::instrument::option_data::ExerciceStyle::European,
            option_type: financed_core::instrument::option_data::OptionType::Call,
            strike_price: StrikePrice(1000.0),
            underlying_asset: lkoh_stock.metadata.id.clone(),
        }),
        metadata: financed_core::instrument::instrument_metadata::InstrumentMetadata {
            currency: rub_currency.clone(),
            exchange: t_invest_exchange.clone(),
            id: InstrumentId("LKOH CALL".to_string()),
            lot_size: LotSize(1.0),
            name: InstrumentName("Лукойл CALL".to_string()),
            price: Price(50.0)
        }
    };

    println!("{lkoh_option:?}");

    if let financed_core::instrument::currency::Currency::Money(money) = &mut rub_currency {
        money.risk_free_rate = RiskFreeRate(0.05);
    }

    let now = Utc::now();

    let test_stock = Instrument {
        data: financed_core::instrument::InstrumentData::Stock(StockData {
            dividend_yield: DividendYield(0.0),
            historical_volatility: HistoricalVolatility(0.15),
        }),
        metadata: InstrumentMetadata {
            currency: rub_currency.clone(),
            exchange: t_invest_exchange.clone(),
            id: InstrumentId("test stock".to_string()),
            lot_size: LotSize(1.0),
            name: InstrumentName("test name".to_string()),
            price: Price(25.0),
        }
    };

    let test_option = Instrument {
        data: financed_core::instrument::InstrumentData::Option(OptionData {
            expiration_date: ExpirationDate(now.checked_add_days(Days::new(60)).unwrap().naive_utc()),
            implied_volatility: ImpliedVolatility(0.15),
            option_style: financed_core::instrument::option_data::ExerciceStyle::European,
            option_type: financed_core::instrument::option_data::OptionType::Put,
            strike_price: StrikePrice(30.0),
            underlying_asset: test_stock.metadata.id.clone(),
        }),
        metadata: InstrumentMetadata {
            id: InstrumentId("test option".to_string()),
            name: InstrumentName("test option".to_string()),
            exchange: t_invest_exchange.clone(),
            currency: rub_currency.clone(),
            price: Price(1.0),
            lot_size: LotSize(1.0),
        }
    };

    let contract = BlackScholesContract {
        option: test_option.clone(),
        underlying_asset: test_stock.clone(),
    };

    let model = BlackScholes {
        settings: BlackScholesSettings {
            from_date: now.naive_utc(),
        }
    };

    let price = model.calculate_price(contract);

    println!("{price:?}");
}
