mod find_odd_txs;

use std::env;

use find_odd_txs::find_odd_txs;

const APP_NAME: &str = "taxbit-odd-txs";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <tbr.csv> <tber.csv>", APP_NAME);

        std::process::exit(1);
    }

    let odd_transactions = find_odd_txs(&args[1], &args[2])?;
    println!("         Dropped: {}", odd_transactions.dropped);
    println!("         Invalid: {}", odd_transactions.invalid);
    println!("Currency Changed: {}", odd_transactions.currency_changed);

    Ok(())
}
