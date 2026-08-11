//! Local TOML config for the scraper CLI.
//!
//! Development artifact: holds store credentials for proving the scraper
//! against a real, login-gated site. Not a shipped config surface; the API
//! server will invoke the scraper directly later.

use std::path::Path;

use serde::Deserialize;

/// Credentials and target for a single store login.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Config {
    pub store_url: String,
    pub username: String,
    pub password: String,
}

/// Parse a config TOML file from disk.
pub fn read_config(path: &Path) -> anyhow::Result<Config> {
    let raw = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("failed to read config at {}: {e}", path.display()))?;
    let cfg: Config = toml::from_str(&raw)
        .map_err(|e| anyhow::anyhow!("failed to parse config at {}: {e}", path.display()))?;
    Ok(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::fixture::FileWriteStr;
    use assert_fs::fixture::PathChild;

    fn sample_toml() -> &'static str {
        r#"
        store_url = "https://example-store.example/login"
        username = "shopper"
        password = "hunter2"
        "#
    }

    #[test]
    fn when_config_toml_is_valid_then_read_config_should_parse_all_fields() {
        let dir = assert_fs::TempDir::new().unwrap();
        let path = dir.child("secrets.toml");
        path.write_str(sample_toml()).unwrap();

        let cfg = read_config(path.path()).unwrap();

        assert_eq!(cfg.store_url, "https://example-store.example/login");
        assert_eq!(cfg.username, "shopper");
        assert_eq!(cfg.password, "hunter2");
    }

    #[test]
    fn when_config_path_does_not_exist_then_read_config_should_error() {
        let path = std::path::Path::new("/nonexistent/secrets.toml");
        let err = read_config(path).unwrap_err();
        assert!(
            err.to_string().contains("failed to read config"),
            "expected read-config error, got: {err}"
        );
    }

    #[test]
    fn when_config_toml_is_missing_a_field_then_read_config_should_error() {
        let dir = assert_fs::TempDir::new().unwrap();
        let path = dir.child("secrets.toml");
        path.write_str(
            r#"
            store_url = "https://example-store.example/login"
            username = "shopper"
            "#,
        )
        .unwrap();

        let err = read_config(path.path()).unwrap_err();
        assert!(
            err.to_string().contains("failed to parse config"),
            "expected parse error, got: {err}"
        );
    }
}
