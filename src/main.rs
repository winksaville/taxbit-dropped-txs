mod find_dropped_txs;

use find_dropped_txs::find_dropped_transactions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tbr_fname = "testdata/3.tbr.csv";
    let tber_fname = "testdata/3.tber.csv";

    let dropped = find_dropped_transactions(tbr_fname, tber_fname)?;
    println!("Dropped: {dropped}");

    Ok(())
}
