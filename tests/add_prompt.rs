use std::env;
use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn test_add_prompt() {
    env::set_var("PROMPT_DB_NAME", "db/test_add_prompt");
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("add").arg("joke").arg("tell me a joke");
    cmd.assert()
        .stdout("Added prompt \"joke\" with template \"tell me a joke\" with 0 parameters\n");
}
