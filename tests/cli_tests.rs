use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn test_add_prompt() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("add").arg("joke").arg("tell me a joke");
    cmd.assert()
        .stdout("Added prompt \"joke\" with template \"tell me a joke\" with 0 parameters\n");
}

#[test]
fn test_add_prompt_with_args() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("add").arg("joke").arg("tell me a joke as [name]");
    cmd.assert()
        .stdout("Added prompt \"joke\" with template \"tell me a joke as [name]\" with 1 parameters\n");
}
