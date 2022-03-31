use serde::Deserialize;
use std::{fs::File, io::BufReader};
use taxbit_export_rec::TaxBitExportRec;
use taxbitrec::{TaxBitRec, TaxBitRecType};
//use rust_decimal_macros::dec;

fn open_file(fname: &str) -> Result<File, Box<dyn std::error::Error>> {
    Ok(match File::open(fname) {
        Ok(f) => f,
        Err(e) => return Err(format!("Unable to open '{fname}'; {e}").into()),
    })
}

fn open_file_buf_reader(fname: &str) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let file = open_file(fname)?;

    Ok(BufReader::new(file))
}

fn read_csv<T>(fname: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: for<'de> Deserialize<'de>,
{
    let reader = open_file_buf_reader(fname)?;
    let mut csv_reader = csv::Reader::from_reader(reader);
    let mut rec_a = Vec::<T>::new();

    for entry in csv_reader.deserialize() {
        let rec: T = match entry {
            Ok(r) => r,
            Err(e) => return Err(format!("processing {}; {}", fname, e).into()),
        };
        rec_a.push(rec);
    }

    Ok(rec_a)
}

pub struct OddTxs {
    pub dropped: usize,
    pub invalid: usize,
    pub currency_changed: usize,
}

impl OddTxs {
    fn new() -> OddTxs {
        OddTxs {
            dropped: 0usize,
            invalid: 0usize,
            currency_changed: 0usize,
        }
    }
}

/// Find odd transactions
pub fn find_odd_txs(
    tbr_fname: &str,
    tber_fname: &str,
) -> Result<OddTxs, Box<dyn std::error::Error>> {
    let mut tbr_a = read_csv::<TaxBitRec>(tbr_fname)?;
    let mut tber_a = read_csv::<TaxBitExportRec>(tber_fname)?;

    // True for now if there has been no editing
    assert!(tbr_a.len() >= tber_a.len());

    // Sort the arrays to be sure time's are ordered
    tbr_a.sort();
    tber_a.sort();

    let tbr_a = tbr_a;
    let tber_a = tber_a;

    let mut odd_transactions = OddTxs::new();
    let mut tber_iter = tber_a.iter();
    let mut tber_cur = tber_iter.next();
    for (idx, tbr) in tbr_a.iter().enumerate() {
        let rec_num = idx + 1;
        print!("{rec_num} tbr : {}", tbr);
        match tber_cur {
            None => {
                print!(" Dropped");
                //print!(", no more tber entries");
                odd_transactions.dropped += 1;
            }
            Some(tber) => {
                if tbr.time != tber.time {
                    print!(" Dropped");
                    //print!(", see tber_cur: {:?}", tber_cur);
                    odd_transactions.dropped += 1;
                } else {
                    if tber.type_txs == TaxBitRecType::Invalid {
                        print!(" Invalid");
                        odd_transactions.invalid += 1;
                    }
                    //println!();
                    //print!("{rec_num} tber: {} CHECKING", tber);
                    //io::stdout().flush().unwrap();

                    // Since ATM the files I'm checking only have income
                    // only received values are present. And it turns out
                    // TaxBit is using different tickers for some.
                    let tbr_rc_str = match tbr.received_currency.as_str() {
                        "ONG" => "ONGAS",
                        "NANO" => "XNO",
                        "COS" => "CONT",
                        "YOYO" => "YOYOW",
                        "BQX" => "VGX",
                        "RDN" => "RDNN",
                        "HOT" => "HOLO",
                        "MDT" => "MSDT",
                        "AVA" => "AVALA",
                        _ => &tbr.received_currency,
                    };
                    if tbr_rc_str != tbr.received_currency {
                        print!(" Currency Changed: {}", tbr_rc_str);
                        odd_transactions.currency_changed += 1;
                    }
                    assert_eq!(tbr_rc_str, &tber.received_currency);
                    assert_eq!(tbr.received_quantity, tber.received_quantity);

                    // For the moment these will be empty
                    assert!(tbr.sent_currency.is_empty());
                    assert_eq!(tbr.sent_currency, tber.sent_currency);
                    assert!(tbr.sent_quantity.is_none());
                    assert_eq!(tbr.sent_quantity, tber.sent_quantity);

                    assert!(tbr.fee_currency.is_empty());
                    assert_eq!(tbr.fee_currency, tber.fee_currency);
                    assert!(tbr.fee_quantity.is_none());
                    assert_eq!(tbr.fee_quantity, tber.fee_amount);

                    tber_cur = tber_iter.next();
                }
                println!();
            }
        }
    }
    println!();

    Ok(odd_transactions)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3() {
        let tbr_3_a = read_csv::<TaxBitRec>("testdata/3.tbr.csv").unwrap();
        let tber_3_a = read_csv::<TaxBitExportRec>("testdata/3.tber.csv").unwrap();

        assert_eq!(tbr_3_a.len(), 3);
        assert_eq!(tber_3_a.len(), 3);
    }

    #[test]
    fn test_find_odd_transactions() {
        let odd_transactions = find_odd_txs("testdata/3.tbr.csv", "testdata/3.tber.csv").unwrap();

        assert_eq!(odd_transactions.dropped, 0);
        assert_eq!(odd_transactions.invalid, 0);
        assert_eq!(odd_transactions.currency_changed, 0);
    }

    #[test]
    fn test_find_odd_transactions_with_invalid() {
        let odd_transactions = find_odd_txs("testdata/4.tbr.csv", "testdata/4.tber.csv").unwrap();

        assert_eq!(odd_transactions.dropped, 1);
        assert_eq!(odd_transactions.invalid, 1);
        assert_eq!(odd_transactions.currency_changed, 1);
    }

    #[test]
    fn test_find_odd_transactions_in_473() {
        let odd_transactions =
            find_odd_txs("testdata/473.tbr.csv", "testdata/473.tber.csv").unwrap();

        assert_eq!(odd_transactions.dropped, 31);
        assert_eq!(odd_transactions.invalid, 1);
        assert_eq!(odd_transactions.currency_changed, 9);
    }
}
