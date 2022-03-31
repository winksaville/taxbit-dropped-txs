use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::error::Error;
use std::process::Command;

// Should be the same as APP_NAME in main
const APP_NAME: &str = "taxbit-odd-txs";

// If tarpaulin starts failing during github Actions disable
// by uncommenting #[cfg(not(tarpauling))]. This will reduce
// coverage as reported by tarpaulin but so be it!
//
// Note: execute "cargo tarpaulin --follow-exec" so these are run
#[test]
//#[cfg(not(tarpaulin))]
fn test_no_params() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(APP_NAME)?;

    cmd.assert()
        .code(predicate::eq(1))
        .stderr(predicate::str::starts_with(format!("Usage: {}", APP_NAME)));

    Ok(())
}

#[test]
//#[cfg(not(tarpaulin))]
fn test_3_recs() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(APP_NAME)?;

    cmd.arg("testdata/3.tbr.csv");
    cmd.arg("testdata/3.tber.csv");

    cmd.assert()
        .code(predicate::eq(0))
        .stdout(predicate::str::contains("Dropped: 0"))
        .stdout(predicate::str::contains("Invalid: 0"))
        .stdout(predicate::str::contains("Currency Changed: 0"));

    Ok(())
}

#[test]
//#[cfg(not(tarpaulin))]
fn test_4_recs() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(APP_NAME)?;

    cmd.arg("testdata/4.tbr.csv");
    cmd.arg("testdata/4.tber.csv");

    // This should probably be 1 eventually
    cmd.assert()
        .code(predicate::eq(0))
        .stdout(predicate::str::contains("Dropped: 1"))
        .stdout(predicate::str::contains("Invalid: 1"))
        .stdout(predicate::str::contains("Currency Changed: 1"));

    Ok(())
}

#[test]
//#[cfg(not(tarpaulin))]
fn test_473_recs() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(APP_NAME)?;

    cmd.arg("testdata/473.tbr.csv");
    cmd.arg("testdata/473.tber.csv");

    // This should probably be 1 eventually
    cmd.assert()
        .code(predicate::eq(0))
        .stdout(predicate::str::contains("Dropped: 31"))
        .stdout(predicate::str::contains("Invalid: 1"))
        .stdout(predicate::str::contains("Currency Changed: 9"));

    Ok(())
}
