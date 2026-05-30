// Configuration command implementations

use crate::cli::{
    config::{CliConfig, ConfigField},
    error::{CliError, Result},
    traits::{Command, CommandArgs, CommandOutput, Configurable, OutputMetadata},
};
use async_trait::async_trait;

use super::get_required_arg;

/// Set configuration value command
pub struct SetConfigCommand<C: Configurable<Config = CliConfig>> {
    config_manager: C,
}

impl<C: Configurable<Config = CliConfig>> SetConfigCommand<C> {
    pub fn new(config_manager: C) -> Self {
        Self { config_manager }
    }
}

#[async_trait]
impl<C: Configurable<Config = CliConfig> + Send + Sync> Command for SetConfigCommand<C> {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let key = get_required_arg(args, "key")?;
        let value = get_required_arg(args, "value")?;

        let field = ConfigField::from_string(&key)
            .ok_or_else(|| CliError::invalid_argument(format!("Unknown config key: {key}")))?;

        let mut config = self.config_manager.load().await?;
        config.set_value(field, &value)?;

        self.config_manager.validate(&config)?;
        self.config_manager.save(&config).await?;

        let data = serde_json::json!({
            "status": "success",
            "key": key,
            "value": value
        });

        Ok(CommandOutput {
            data,
            metadata: OutputMetadata::default(),
        })
    }

    fn name(&self) -> &str {
        "config.set"
    }

    fn description(&self) -> &str {
        "Set a configuration value"
    }

    fn validate_args(&self, args: &CommandArgs) -> Result<()> {
        if !args.named.contains_key("key") || !args.named.contains_key("value") {
            return Err(CliError::validation("Both key and value are required"));
        }
        Ok(())
    }
}

/// Get a single configuration value command
pub struct GetConfigCommand<C: Configurable<Config = CliConfig>> {
    config_manager: C,
}

impl<C: Configurable<Config = CliConfig>> GetConfigCommand<C> {
    pub fn new(config_manager: C) -> Self {
        Self { config_manager }
    }
}

#[async_trait]
impl<C: Configurable<Config = CliConfig> + Send + Sync> Command for GetConfigCommand<C> {
    async fn execute(&self, args: &CommandArgs) -> Result<CommandOutput> {
        let key = get_required_arg(args, "key")?;

        let field = ConfigField::from_string(&key)
            .ok_or_else(|| CliError::invalid_argument(format!("Unknown config key: {key}")))?;

        let config = self.config_manager.load().await?;
        let value = config.get_value(field);

        let data = serde_json::json!({ "key": key, "value": value });

        Ok(CommandOutput {
            data,
            metadata: OutputMetadata::default(),
        })
    }

    fn name(&self) -> &str {
        "config.get"
    }

    fn description(&self) -> &str {
        "Get a configuration value"
    }

    fn validate_args(&self, args: &CommandArgs) -> Result<()> {
        if !args.named.contains_key("key") {
            return Err(CliError::validation("A config key is required"));
        }
        Ok(())
    }
}

/// List all configuration values command
pub struct ListConfigCommand<C: Configurable<Config = CliConfig>> {
    config_manager: C,
}

impl<C: Configurable<Config = CliConfig>> ListConfigCommand<C> {
    pub fn new(config_manager: C) -> Self {
        Self { config_manager }
    }
}

#[async_trait]
impl<C: Configurable<Config = CliConfig> + Send + Sync> Command for ListConfigCommand<C> {
    async fn execute(&self, _args: &CommandArgs) -> Result<CommandOutput> {
        let config = self.config_manager.load().await?;

        let data = serde_json::json!({
            "worker_url": config.get_value(ConfigField::WorkerUrl),
            "output_format": config.get_value(ConfigField::OutputFormat),
            "cache_ttl": config.get_value(ConfigField::CacheTtl),
            "interactive_history_size": config.get_value(ConfigField::InteractiveHistorySize),
            "color_output": config.get_value(ConfigField::ColorOutput),
            "request_timeout_secs": config.get_value(ConfigField::RequestTimeoutSecs),
        });

        Ok(CommandOutput {
            data,
            metadata: OutputMetadata::default(),
        })
    }

    fn name(&self) -> &str {
        "config.list"
    }

    fn description(&self) -> &str {
        "List all configuration values"
    }

    fn validate_args(&self, _args: &CommandArgs) -> Result<()> {
        Ok(())
    }
}
