use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::error::Error;
use std::process::Command;

const APP_NAME: &str = "taxbit-dropped-txs";

// For some reason these tests are unreliable on tarpaulin when
// executed via github Actions. The cfg(not(tarpaulin)) causes
// them to be skipped.

#[test]
#[cfg(not(tarpaulin))]
fn test_no_params() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin(APP_NAME)?;

    cmd.assert()
        .code(predicate::eq(0))
        .stdout(predicate::str::starts_with("Dropped: 0"));

    Ok(())
}
