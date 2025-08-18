use anyhow::{Result, anyhow};
use codex_common::CliConfigOverrides;
use codex_core::config::{Config, ConfigOverrides};
use codex_login::{
    AuthMode, CLIENT_ID, CodexAuth, OPENAI_API_KEY_ENV_VAR, ServerOptions, login_with_api_key,
    logout, run_login_server,
};
use std::env;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum LoginStatus {
    LoggedIn,
    NotLoggedIn,
}

#[derive(Debug, PartialEq)]
pub enum LogoutStatus {
    LoggedOut,
    NotLoggedIn,
}

pub async fn login_with_chatgpt(codex_home: PathBuf) -> std::io::Result<()> {
    let opts = ServerOptions::new(codex_home, CLIENT_ID.to_string());
    let server = run_login_server(opts, None)?;

    eprintln!(
        "Starting local login server on http://localhost:{}.\nIf your browser did not open, navigate to this URL to authenticate:\n\n{}",
        server.actual_port, server.auth_url,
    );

    server.block_until_done()
}

pub async fn run_login_with_chatgpt(cli_config_overrides: CliConfigOverrides) -> Result<()> {
    let config = load_config(cli_config_overrides)?;

    match login_with_chatgpt(config.codex_home).await {
        Ok(_) => {
            eprintln!("Successfully logged in");
            Ok(())
        }
        Err(e) => {
            eprintln!("Error logging in: {e}");
            Err(e.into())
        }
    }
}

pub async fn run_login_with_api_key(
    cli_config_overrides: CliConfigOverrides,
    api_key: String,
) -> Result<()> {
    let config = load_config(cli_config_overrides)?;

    match login_with_api_key(&config.codex_home, &api_key) {
        Ok(_) => {
            eprintln!("Successfully logged in");
            Ok(())
        }
        Err(e) => {
            eprintln!("Error logging in: {e}");
            Err(e.into())
        }
    }
}

pub async fn run_login_status(cli_config_overrides: CliConfigOverrides) -> Result<LoginStatus> {
    let config = load_config(cli_config_overrides)?;

    match CodexAuth::from_codex_home(&config.codex_home) {
        Ok(Some(auth)) => match auth.mode {
            AuthMode::ApiKey => match auth.get_token().await {
                Ok(api_key) => {
                    eprintln!("Logged in using an API key - {}", safe_format_key(&api_key));

                    if let Ok(env_api_key) = env::var(OPENAI_API_KEY_ENV_VAR) {
                        if env_api_key == api_key {
                            eprintln!(
                                "   API loaded from OPENAI_API_KEY environment variable or .env file",
                            );
                        }
                    }
                    Ok(LoginStatus::LoggedIn)
                }
                Err(e) => {
                    eprintln!("Unexpected error retrieving API key: {e}");
                    Err(e.into())
                }
            },
            AuthMode::ChatGPT => {
                eprintln!("Logged in using ChatGPT");
                Ok(LoginStatus::LoggedIn)
            }
        },
        Ok(None) => {
            eprintln!("Not logged in");
            Ok(LoginStatus::NotLoggedIn)
        }
        Err(e) => {
            eprintln!("Error checking login status: {e}");
            Err(e.into())
        }
    }
}

pub async fn run_logout(cli_config_overrides: CliConfigOverrides) -> Result<LogoutStatus> {
    let config = load_config(cli_config_overrides)?;

    match logout(&config.codex_home) {
        Ok(true) => {
            eprintln!("Successfully logged out");
            Ok(LogoutStatus::LoggedOut)
        }
        Ok(false) => {
            eprintln!("Not logged in");
            Ok(LogoutStatus::NotLoggedIn)
        }
        Err(e) => {
            eprintln!("Error logging out: {e}");
            Err(e.into())
        }
    }
}

fn load_config(cli_config_overrides: CliConfigOverrides) -> Result<Config> {
    let cli_overrides = cli_config_overrides.parse_overrides().map_err(|e| {
        eprintln!("Error parsing -c overrides: {e}");
        anyhow!(e)
    })?;

    let config_overrides = ConfigOverrides::default();
    Config::load_with_cli_overrides(cli_overrides, config_overrides).map_err(|e| {
        eprintln!("Error loading configuration: {e}");
        e.into()
    })
}

fn safe_format_key(key: &str) -> String {
    if key.len() <= 13 {
        return "***".to_string();
    }
    let prefix = &key[..8];
    let suffix = &key[key.len() - 5..];
    format!("{prefix}***{suffix}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex_common::CliConfigOverrides;
    use std::sync::Mutex;
    use tempfile::tempdir;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn formats_long_key() {
        let key = "sk-proj-1234567890ABCDE";
        assert_eq!(safe_format_key(key), "sk-proj-***ABCDE");
    }

    #[test]
    fn short_key_returns_stars() {
        let key = "sk-proj-12345";
        assert_eq!(safe_format_key(key), "***");
    }

    #[tokio::test]
    async fn login_with_api_key_and_status() {
        let _guard = ENV_LOCK.lock().unwrap();
        let dir = tempdir().unwrap();
        unsafe { std::env::set_var("CODEX_HOME", dir.path()); }
        let overrides = CliConfigOverrides::default();
        run_login_with_api_key(overrides.clone(), "sk-test-key".into())
            .await
            .unwrap();
        let status = run_login_status(overrides).await.unwrap();
        assert_eq!(status, LoginStatus::LoggedIn);
        unsafe { std::env::remove_var("CODEX_HOME"); }
    }

    #[tokio::test]
    async fn login_with_api_key_bad_override() {
        let _guard = ENV_LOCK.lock().unwrap();
        let dir = tempdir().unwrap();
        unsafe { std::env::set_var("CODEX_HOME", dir.path()); }
        let overrides = CliConfigOverrides {
            raw_overrides: vec!["bad".into()],
        };
        assert!(
            run_login_with_api_key(overrides, "sk".into())
                .await
                .is_err()
        );
        unsafe { std::env::remove_var("CODEX_HOME"); }
    }

    #[tokio::test]
    async fn login_status_not_logged_in() {
        let _guard = ENV_LOCK.lock().unwrap();
        let dir = tempdir().unwrap();
        unsafe { std::env::set_var("CODEX_HOME", dir.path()); }
        let overrides = CliConfigOverrides::default();
        let status = run_login_status(overrides).await.unwrap();
        assert_eq!(status, LoginStatus::NotLoggedIn);
        unsafe { std::env::remove_var("CODEX_HOME"); }
    }

    #[tokio::test]
    async fn logout_when_not_logged_in() {
        let _guard = ENV_LOCK.lock().unwrap();
        let dir = tempdir().unwrap();
        unsafe { std::env::set_var("CODEX_HOME", dir.path()); }
        let overrides = CliConfigOverrides::default();
        let status = run_logout(overrides).await.unwrap();
        assert_eq!(status, LogoutStatus::NotLoggedIn);
        unsafe { std::env::remove_var("CODEX_HOME"); }
    }

    #[tokio::test]
    async fn logout_after_login() {
        let _guard = ENV_LOCK.lock().unwrap();
        let dir = tempdir().unwrap();
        unsafe { std::env::set_var("CODEX_HOME", dir.path()); }
        let overrides = CliConfigOverrides::default();
        run_login_with_api_key(overrides.clone(), "sk-test".into())
            .await
            .unwrap();
        let status = run_logout(overrides).await.unwrap();
        assert_eq!(status, LogoutStatus::LoggedOut);
        unsafe { std::env::remove_var("CODEX_HOME"); }
    }

    #[tokio::test]
    async fn logout_bad_override() {
        let _guard = ENV_LOCK.lock().unwrap();
        let dir = tempdir().unwrap();
        unsafe { std::env::set_var("CODEX_HOME", dir.path()); }
        let overrides = CliConfigOverrides {
            raw_overrides: vec!["bad".into()],
        };
        assert!(run_logout(overrides).await.is_err());
        unsafe { std::env::remove_var("CODEX_HOME"); }
    }
}
