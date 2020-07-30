use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

const NAME: &str = "catsay";

#[test]
fn test_with_defaults() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin(NAME).expect("binary exists").assert().success();
    Ok(())
}

#[test]
fn run_with_defaults() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin(NAME).expect("binary exists").assert().success().stdout(predicate::str::contains("Meow!"));
    Ok(())
}

#[test]
fn fail_on_non_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin(NAME).expect("binary exists").args(&["-f", "no/such/file.txt"])
        .assert().failure();
    Ok(())
}