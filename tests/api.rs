use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn launch_cli() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gts")?;
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("gts 0.0.1\n\n"));
    Ok(())
}
