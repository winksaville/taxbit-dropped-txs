# TaxBit odd transactions

![ci-stable](https://github.com/winksaville/taxbit-odd-txs/actions/workflows/ci-stable.yml/badge.svg)
![ci-nightly](https://github.com/winksaville/taxbit-odd-txs/actions/workflows/ci-nightly.yml/badge.svg)
[![codecov](https://codecov.io/gh/winksaville/taxbit-odd-txs/branch/main/graph/badge.svg?token=cowZtK1KK1)](https://codecov.io/gh/winksaville/taxbit-odd-txs)

Determine what transactions where odd in TaxBit after uploading a CSV file.

> **Note: In no case can the authors of this program be held responsible
> for any damanges or monetary losses.**

## Build

`cargo build`

## Test

`cargo test`

## Run

Run for release:

```
$ cargo run --release testdata/4.tbr.csv testdata/4.tber.csv 
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/taxbit-odd-txs testdata/4.tbr.csv testdata/4.tber.csv`
1 tbr : 2019-05-21T21:39:50.000+00:00,Income,,,,4.4238,BTT,Binance,,,,
2 tbr : 2019-07-29T11:23:32.000+00:00,Income,,,,0.0338,ONG,Binance,,,, Currency Changed: ONGAS
3 tbr : 2020-01-22T12:43:02.000+00:00,Income,,,,0.00223642,TRY,Binance,,,, Dropped
4 tbr : 2020-12-08T14:35:35.000+00:00,Income,,,,0.0009676,CREAM,Binance,,,, Invalid

         Dropped: 1
         Invalid: 1
Currency Changed: 1
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
