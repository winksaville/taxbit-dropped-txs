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
$ cargo run --release testdata/473.tbr.csv testdata/473.tber.csv 
   Compiling autocfg v1.1.0
   Compiling proc-macro2 v1.0.36
   Compiling unicode-xid v0.2.2
   Compiling syn v1.0.89
...
   Compiling dec-utils v0.1.0 (https://github.com/winksaville/dec-utils#f195fd25)
   Compiling taxbitrec v0.4.0 (https://github.com/winksaville/taxbitrec#89ddf5f4)
   Compiling taxbit-export-rec v0.3.0 (https://github.com/winksaville/taxbit-export-rec#3b5d8619)
   Compiling taxbit-odd-txs v0.5.0 (/home/wink/prgs/rust/myrepos/taxbit-odd-txs)
    Finished release [optimized] target(s) in 8.05s
     Running `target/release/taxbit-odd-txs testdata/473.tbr.csv testdata/473.tber.csv`
0 tbr : 2019-05-21T21:39:50.000+00:00,Income,,,,4.4238,BTT,Binance,,,,
1 tbr : 2019-05-23T07:54:20.000+00:00,Income,,,,0.2916,MATIC,Binance,,,,
2 tbr : 2019-05-24T09:50:15.000+00:00,Income,,,,0.003,THETA,Binance,,,,
...
467 tbr : 2021-12-17T07:00:42.000+00:00,Income,,,,0.00641568,HIGH,Binance,,,, Dropped
468 tbr : 2021-12-20T04:28:52.000+00:00,Income,,,,0.0344277,ANY,Binance,,,, Dropped
469 tbr : 2021-12-23T16:39:26.000+00:00,Income,,,,955.4459717,OOKI,Binance,,,,
470 tbr : 2021-12-24T01:38:12.000+00:00,Income,,,,0.00051414,CVX,Binance,,,,
471 tbr : 2021-12-25T12:02:46.000+00:00,Income,,,,0.0034768,BICO,Binance,,,,
472 tbr : 2021-12-26T08:41:48.000+00:00,Income,,,,0.00615,UST,Binance,,,,
         Dropped: 31
         Invalid: 1
Currency Changed: 9
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
