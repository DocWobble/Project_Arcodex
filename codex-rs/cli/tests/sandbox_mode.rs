use codex_cli::debug_sandbox::create_sandbox_mode;
use codex_core::config_types::SandboxMode;

#[test]
fn full_auto_enables_workspace_write() {
    assert_eq!(create_sandbox_mode(true), SandboxMode::WorkspaceWrite);
}

#[test]
fn default_is_read_only() {
    assert_eq!(create_sandbox_mode(false), SandboxMode::ReadOnly);
}
