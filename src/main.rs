mod find_dropped_txs;

use find_dropped_txs::find_dropped_transactions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("main:+");
    find_dropped_transactions(
        "data/b.com/b.com.uc.473.2019-2021.tbr.csv",
        "data/b.com/b.com.uc.473.2019-2021.tber.csv",
    )?;

    println!("main:- Ok");
    Ok(())
}
