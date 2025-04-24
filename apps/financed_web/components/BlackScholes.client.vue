<script setup lang="ts">
import init, { BlackScholes, BlackScholesContract, BlackScholesSettings, CurrencyId, CurrencyName, DividendYield, Exchange, ExerciceStyle, ExpirationDate, FromDate, HistoricalVolatility, ImpliedVolatility, Instrument, InstrumentId, InstrumentMetadata, InstrumentName, LotSize, MoneyCurrencyData, OptionData, OptionType, Price, RiskFreeRate, StockData, StrikePrice } from 'financed_core'

onMounted(async () => {
    await init()

    const settings = new BlackScholesSettings(new FromDate("2025-04-21 15:00:00"))
    const model = new BlackScholes(settings)

    const stock = new Instrument(
        new InstrumentMetadata(
            new InstrumentId("TST"),
            new InstrumentName("TST"),
            new Exchange("T invest"),
            new MoneyCurrencyData(
                new CurrencyId("RUB"),
                new CurrencyName("ruble"),
                new RiskFreeRate(0.21)
            ).money_currency(),
            new Price(25.0),
            new LotSize(1.0),
        ),
        new StockData(
            new DividendYield(0.0),
            new HistoricalVolatility(0.15),
        ).stock_instrument()
    );

    const option = new Instrument(
        new InstrumentMetadata(
            new InstrumentId("TST"),
            new InstrumentName("TST"),
            new Exchange("T invest"),
            new MoneyCurrencyData(
                new CurrencyId("RUB"),
                new CurrencyName("ruble"),
                new RiskFreeRate(0.21)
            ).money_currency(),
            new Price(1.0),
            new LotSize(1.0),
        ),
        new OptionData(
            OptionType.Put,
            ExerciceStyle.European,
            new StrikePrice(30.0),
            new ImpliedVolatility(0.15),
            new ExpirationDate("2025-06-20 20:00:00"),
            new InstrumentId("MTLR")
        ).option_instrument()
    )

    const contract = new BlackScholesContract(option, stock)
    const price = model.calculate_price(contract)

    console.log(price)
})
</script>

<template>
    <div>
        Black Scholes
    </div>
</template>
