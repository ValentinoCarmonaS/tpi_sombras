use assert_cmd::prelude::*; // add methods on commands
use predicates::prelude::*; // used for writing assertions
use std::{fs::File, process::Command}; // run programs

#[test]
fn test01() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tpi_sombras")?;
    cmd.stdin(File::open("./tests/test01.txt")?);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("15.000000000000"));
    Ok(())
}

#[test]
fn test02() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tpi_sombras")?;
    cmd.stdin(File::open("./tests/test02.txt")?);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("446.4101615137755"));
    Ok(())
}

#[test]
fn test03() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tpi_sombras")?;
    cmd.stdin(File::open("./tests/test03.txt")?);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("300"));
    Ok(())
}