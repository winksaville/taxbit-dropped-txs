mod find_dropped_txs;

use std::env;

use find_dropped_txs::find_dropped_transactions;

const APP_NAME: &str = "taxbit-dropped-txs";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <tbr.csv> <tber.csv>", APP_NAME);

        std::process::exit(1);
    }

    let dropped = find_dropped_transactions(&args[1], &args[2])?;
    println!("Dropped: {dropped}");

    Ok(())
}
