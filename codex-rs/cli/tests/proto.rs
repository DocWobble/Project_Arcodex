use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn proto_help_runs() {
    Command::cargo_bin("codex")
        .unwrap()
        .args(["proto", "--help"])
        .assert()
        .code(0)
        .stdout(contains("Protocol"));
}
