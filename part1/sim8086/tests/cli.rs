use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;

fn compare(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let asm_path = format!("{}.asm", path);
    let expected_output = fs::read_to_string(asm_path)?;

    let mut cmd = Command::cargo_bin("sim8086")?;
    cmd.arg(path);

    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(expected_output).unwrap());

    Ok(())
}

#[test]
fn listing_0037_single_register_mov() -> Result<(), Box<dyn std::error::Error>> {
    compare("tests/resources/listing_0037_single_register_mov")?;
    Ok(())
}

#[test]
fn listing_0038_single_register_mov() -> Result<(), Box<dyn std::error::Error>> {
    compare("tests/resources/listing_0038_many_register_mov")?;
    Ok(())
}

#[test]
fn listing_0039_single_register_mov() -> Result<(), Box<dyn std::error::Error>> {
    compare("tests/resources/listing_0039_more_movs")?;
    Ok(())
}

#[test]
fn listing_0040_single_register_mov() -> Result<(), Box<dyn std::error::Error>> {
    compare("tests/resources/listing_0040_challenge_movs")?;
    Ok(())
}
