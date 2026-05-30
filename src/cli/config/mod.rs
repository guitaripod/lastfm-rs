// Configuration management module

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

use crate::cli::{
    auth::AuthConfig,
    error::{CliError, Result},
    traits::Configurable,
    OutputFormat,
};

/// CLI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CliConfig {
    pub worker_url: String,
    pub output_format: OutputFormat,
    pub cache_ttl: u64,
    pub interactive_history_size: usize,
    pub color_output: bool,
    pub request_timeout_secs: u64,
    #[serde(default)]
    pub auth: AuthConfig,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            worker_url: String::new(),
            output_format: OutputFormat::Pretty,
            cache_ttl: 3600,
            interactive_history_size: 1000,
            color_output: true,
            request_timeout_secs: 30,
            auth: AuthConfig::default(),
        }
    }
}

/// Configuration manager implementation
#[derive(Clone)]
pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    /// Create a new configuration manager
    pub fn new() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        Ok(Self { config_path })
    }

    /// Get the configuration file path
    fn get_config_path() -> Result<PathBuf> {
        let home =
            dirs::home_dir().ok_or_else(|| CliError::config("Could not find home directory"))?;
        Ok(home.join(".config").join("lastfm-cli").join("config.toml"))
    }

    /// Ensure the configuration directory exists
    async fn ensure_config_dir(&self) -> Result<()> {
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| CliError::config(format!("Failed to create config directory: {e}")))?;
        }
        Ok(())
    }
}

#[async_trait]
impl Configurable for ConfigManager {
    type Config = CliConfig;

    async fn load(&self) -> Result<Self::Config> {
        match fs::read_to_string(&self.config_path).await {
            Ok(content) => toml::from_str(&content)
                .map_err(|e| CliError::config(format!("Invalid config file: {e}"))),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // Config file doesn't exist, use default
                let config = self.default_config();
                self.save(&config).await?;
                Ok(config)
            }
            Err(e) => Err(CliError::config(format!("Failed to read config file: {e}"))),
        }
    }

    async fn save(&self, config: &Self::Config) -> Result<()> {
        self.ensure_config_dir().await?;

        let content = toml::to_string_pretty(config)
            .map_err(|e| CliError::config(format!("Failed to serialize config: {e}")))?;

        fs::write(&self.config_path, content)
            .await
            .map_err(|e| CliError::config(format!("Failed to write config file: {e}")))?;

        Ok(())
    }

    fn default_config(&self) -> Self::Config {
        CliConfig::default()
    }

    fn validate(&self, config: &Self::Config) -> Result<()> {
        if config.request_timeout_secs == 0 {
            return Err(CliError::validation(
                "Request timeout must be greater than 0",
            ));
        }

        // Validate cache TTL
        if config.cache_ttl == 0 {
            return Err(CliError::validation("Cache TTL must be greater than 0"));
        }

        Ok(())
    }
}

/// Configuration field names for the CLI
#[derive(Debug, PartialEq)]
pub enum ConfigField {
    WorkerUrl,
    OutputFormat,
    CacheTtl,
    InteractiveHistorySize,
    ColorOutput,
    RequestTimeoutSecs,
}

impl ConfigField {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WorkerUrl => "worker_url",
            Self::OutputFormat => "output_format",
            Self::CacheTtl => "cache_ttl",
            Self::InteractiveHistorySize => "interactive_history_size",
            Self::ColorOutput => "color_output",
            Self::RequestTimeoutSecs => "request_timeout_secs",
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "worker_url" => Some(Self::WorkerUrl),
            "output_format" => Some(Self::OutputFormat),
            "cache_ttl" => Some(Self::CacheTtl),
            "interactive_history_size" => Some(Self::InteractiveHistorySize),
            "color_output" => Some(Self::ColorOutput),
            "request_timeout_secs" => Some(Self::RequestTimeoutSecs),
            _ => None,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::WorkerUrl => "URL of the Last.fm proxy worker",
            Self::OutputFormat => "Default output format (json, table, pretty, compact)",
            Self::CacheTtl => "Cache time-to-live in seconds",
            Self::InteractiveHistorySize => "Number of commands to keep in interactive history",
            Self::ColorOutput => "Enable colored output",
            Self::RequestTimeoutSecs => "Request timeout in seconds",
        }
    }
}

impl CliConfig {
    /// Get a configuration value as a string
    pub fn get_value(&self, field: ConfigField) -> String {
        match field {
            ConfigField::WorkerUrl => self.worker_url.clone(),
            ConfigField::OutputFormat => format!("{:?}", self.output_format).to_lowercase(),
            ConfigField::CacheTtl => self.cache_ttl.to_string(),
            ConfigField::InteractiveHistorySize => self.interactive_history_size.to_string(),
            ConfigField::ColorOutput => self.color_output.to_string(),
            ConfigField::RequestTimeoutSecs => self.request_timeout_secs.to_string(),
        }
    }

    /// Set a configuration value from a string
    pub fn set_value(&mut self, field: ConfigField, value: &str) -> Result<()> {
        match field {
            ConfigField::WorkerUrl => self.worker_url = value.to_string(),
            ConfigField::OutputFormat => {
                self.output_format = match value {
                    "json" => OutputFormat::Json,
                    "table" => OutputFormat::Table,
                    "pretty" => OutputFormat::Pretty,
                    "compact" => OutputFormat::Compact,
                    _ => return Err(CliError::validation("Invalid output format")),
                };
            }
            ConfigField::CacheTtl => {
                self.cache_ttl = value
                    .parse()
                    .map_err(|_| CliError::validation("Invalid cache TTL"))?;
            }
            ConfigField::InteractiveHistorySize => {
                self.interactive_history_size = value
                    .parse()
                    .map_err(|_| CliError::validation("Invalid history size"))?;
            }
            ConfigField::ColorOutput => {
                self.color_output = value
                    .parse()
                    .map_err(|_| CliError::validation("Invalid boolean value"))?;
            }
            ConfigField::RequestTimeoutSecs => {
                self.request_timeout_secs = value
                    .parse()
                    .map_err(|_| CliError::validation("Invalid timeout value"))?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_field_conversion() {
        assert_eq!(
            ConfigField::from_string("worker_url"),
            Some(ConfigField::WorkerUrl)
        );
        assert_eq!(ConfigField::from_string("invalid"), None);

        assert_eq!(ConfigField::WorkerUrl.as_str(), "worker_url");
    }

    #[test]
    fn test_config_get_set_value() {
        let mut config = CliConfig::default();

        // Test getting values
        assert_eq!(config.get_value(ConfigField::WorkerUrl), "");
        assert_eq!(config.get_value(ConfigField::CacheTtl), "3600");

        // Test setting values
        config
            .set_value(ConfigField::WorkerUrl, "https://example.com")
            .unwrap();
        assert_eq!(config.worker_url, "https://example.com");

        config.set_value(ConfigField::CacheTtl, "7200").unwrap();
        assert_eq!(config.cache_ttl, 7200);

        // Test invalid values
        assert!(config.set_value(ConfigField::CacheTtl, "invalid").is_err());
        assert!(config
            .set_value(ConfigField::OutputFormat, "invalid")
            .is_err());
    }

    #[test]
    fn test_validate_allows_empty_worker_url_and_rejects_zero() {
        let manager = ConfigManager::new().unwrap();

        let unconfigured = CliConfig::default();
        assert!(unconfigured.worker_url.is_empty());
        assert!(manager.validate(&unconfigured).is_ok());

        let zero_timeout = CliConfig {
            request_timeout_secs: 0,
            ..CliConfig::default()
        };
        assert!(manager.validate(&zero_timeout).is_err());

        let zero_ttl = CliConfig {
            cache_ttl: 0,
            ..CliConfig::default()
        };
        assert!(manager.validate(&zero_ttl).is_err());
    }

    #[test]
    fn test_partial_config_uses_defaults_for_missing_fields() {
        let partial = "worker_url = \"https://partial.worker.dev\"\noutput_format = \"json\"\n";
        let config: CliConfig = toml::from_str(partial).unwrap();

        assert_eq!(config.worker_url, "https://partial.worker.dev");
        assert_eq!(config.get_value(ConfigField::OutputFormat), "json");
        assert!(config.color_output);
        assert_eq!(config.request_timeout_secs, 30);
        assert_eq!(config.cache_ttl, 3600);
    }
}
