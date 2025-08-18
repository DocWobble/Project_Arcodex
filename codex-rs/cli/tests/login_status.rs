use assert_cmd::Command;
use predicates::str::contains;
use tempfile::tempdir;

#[test]
fn login_status_not_logged_in() {
    let dir = tempdir().unwrap();
    let mut cmd = Command::cargo_bin("codex").unwrap();
    cmd.env("CODEX_HOME", dir.path())
        .env_remove("OPENAI_API_KEY")
        .arg("login")
        .arg("status")
        .assert()
        .code(1)
        .stderr(contains("Not logged in"));
}
