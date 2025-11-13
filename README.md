# Currency Converter

This is a CLI tool that uses a [keyless API](https://github.com/fawazahmed0/exchange-api) for daily reports to calculate conversions.

## Usage

```
Usage: CURRENCY1 CURRENCY2 AMOUNT
```

Using exe:

```
./currency_converter.exe usd cad 15.8
```


Using Cargo:
```
cargo run usd eur 20
```

Response:

```
Converting 20 USD to EUR
Conversion Rate: 0.86
Amount: 17.28
```