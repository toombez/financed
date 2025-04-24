use chrono::{Days, NaiveDateTime, NaiveTime, Utc};
use financed_core::{black_scholes::{BlackScholes, BlackScholesContract, BlackScholesSettings, FromDate}, instrument::{currency::{CurrencyId, CurrencyName, MoneyCurrencyData, RiskFreeRate}, instrument_metadata::{Exchange, InstrumentId, InstrumentMetadata, InstrumentName, LotSize, Price}, option_data::{ExpirationDate, ImpliedVolatility, OptionData, StrikePrice}, stock_data::{DividendYield, HistoricalVolatility, StockData}, Instrument}};

fn main() {
    let mut rub_currency = financed_core::instrument::currency::Currency::Money(MoneyCurrencyData {
        id: CurrencyId::new("RUB".to_string()),
        name: CurrencyName::new("Рубли".to_string()),
        risk_free_rate: RiskFreeRate::new(0.21),
    });

    let t_invest_exchange = Exchange::new("T Invest".to_string());

    // println!("{rub_currency:?}");

    let mtlr_stock = Instrument {
        data: financed_core::instrument::InstrumentData::Stock(StockData {
            dividend_yield: DividendYield::new(0.21),
            historical_volatility: HistoricalVolatility::new(1.1),
        }),
        metadata: financed_core::instrument::instrument_metadata::InstrumentMetadata {
            currency: rub_currency.clone(),
            exchange: t_invest_exchange.clone(),
            id: InstrumentId::new("MTLR".to_string()),
            lot_size: LotSize::new(1.0),
            name: InstrumentName::new("Мечел".to_string()),
            price: Price::new(94.5),
        }
    };

    // println!("{mtlr_stock:?}");

    let lkoh_stock = Instrument {
        data: financed_core::instrument::InstrumentData::Stock(StockData {
            dividend_yield: DividendYield::new(0.21),
            historical_volatility: HistoricalVolatility::new(1.1),
        }),
        metadata: financed_core::instrument::instrument_metadata::InstrumentMetadata {
            currency: rub_currency.clone(),
            exchange: t_invest_exchange.clone(),
            id: InstrumentId::new("LKOH".to_string()),
            lot_size: LotSize::new(1.0),
            name: InstrumentName::new("Лукойл".to_string()),
            price: Price::new(1000.0),
        }
    };

    // println!("{lkoh_stock:?}");

    let lkoh_option = Instrument {
        data: financed_core::instrument::InstrumentData::Option(OptionData {
            expiration_date: ExpirationDate::new("2015-09-05 23:56:04".to_string()),
            implied_volatility: ImpliedVolatility::new(1.1),
            option_style: financed_core::instrument::option_data::ExerciceStyle::European,
            option_type: financed_core::instrument::option_data::OptionType::Call,
            strike_price: StrikePrice::new(1000.0),
            underlying_asset: lkoh_stock.metadata.id.clone(),
        }),
        metadata: financed_core::instrument::instrument_metadata::InstrumentMetadata {
            currency: rub_currency.clone(),
            exchange: t_invest_exchange.clone(),
            id: InstrumentId::new("LKOH CALL".to_string()),
            lot_size: LotSize::new(1.0),
            name: InstrumentName::new("Лукойл CALL".to_string()),
            price: Price::new(50.0)
        }
    };

    // println!("{lkoh_option:?}");

    if let financed_core::instrument::currency::Currency::Money(money) = &mut rub_currency {
        money.risk_free_rate = RiskFreeRate::new(0.05);
    }

    let now = Utc::now();

    let test_stock = Instrument {
        data: financed_core::instrument::InstrumentData::Stock(StockData {
            dividend_yield: DividendYield::new(0.0),
            historical_volatility: HistoricalVolatility::new(0.15),
        }),
        metadata: InstrumentMetadata {
            currency: rub_currency.clone(),
            exchange: t_invest_exchange.clone(),
            id: InstrumentId::new("test stock".to_string()),
            lot_size: LotSize::new(1.0),
            name: InstrumentName::new("test name".to_string()),
            price: Price::new(25.0),
        }
    };

    let test_option = Instrument {
        data: financed_core::instrument::InstrumentData::Option(OptionData {
            expiration_date: ExpirationDate::new(
                now.checked_add_days(Days::new(60)).unwrap().with_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()).unwrap().format("%Y-%m-%d %H:%M:%S").to_string()
            ),
            implied_volatility: ImpliedVolatility::new(0.15),
            option_style: financed_core::instrument::option_data::ExerciceStyle::European,
            option_type: financed_core::instrument::option_data::OptionType::Put,
            strike_price: StrikePrice::new(30.0),
            underlying_asset: test_stock.metadata.id.clone(),
        }),
        metadata: InstrumentMetadata {
            id: InstrumentId::new("test option".to_string()),
            name: InstrumentName::new("test option".to_string()),
            exchange: t_invest_exchange.clone(),
            currency: rub_currency.clone(),
            price: Price::new(1.0),
            lot_size: LotSize::new(1.0),
        }
    };

    let contract = BlackScholesContract {
        option: test_option.clone(),
        underlying_asset: test_stock.clone(),
    };

    let model = BlackScholes {
        settings: BlackScholesSettings {
            from_date: FromDate::new(now.naive_utc().format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    };

    let price = model.calculate_price(&contract);

    println!("{price:?}");
}
