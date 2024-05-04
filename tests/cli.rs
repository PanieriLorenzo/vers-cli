use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn bin() -> Result<Command> {
    Ok(Command::cargo_bin("vers-cli")?)
}

#[test]
fn major() -> Result<()> {
    let mut cmd = bin()?;
    cmd.arg("bump").arg("major").arg("v1.2.3");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("v2.0.0"));
    Ok(())
}

#[test]
fn minor() -> Result<()> {
    let mut cmd = bin()?;
    cmd.arg("bump").arg("minor").arg("v1.2.3");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("v1.3.0"));
    Ok(())
}

#[test]
fn patch() -> Result<()> {
    let mut cmd = bin()?;
    cmd.arg("bump").arg("patch").arg("v1.2.3");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("v1.2.4"));
    Ok(())
}

#[test]
fn strict() -> Result<()> {
    let mut cmd = bin()?;
    cmd.arg("bump").arg("minor").arg("1.2.3").arg("--strict");
    cmd.assert().success();
    Ok(())
}

#[test]
fn strict_neg() -> Result<()> {
    let mut cmd = bin()?;
    cmd.arg("bump").arg("minor").arg("v1.2.3").arg("--strict");
    cmd.assert().failure();
    Ok(())
}

#[test]
fn pre_build() -> Result<()> {
    let mut cmd = bin()?;
    cmd.arg("bump").arg("minor").arg("v1.2.3-foo+bar");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("v1.3.0-foo+bar"));
    Ok(())
}
