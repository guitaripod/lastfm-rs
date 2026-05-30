// Commands module - implements all CLI commands

pub mod album;
pub mod artist;
pub mod auth;
pub mod cache;
pub mod chart;
pub mod config;
pub mod geo;
pub mod library;
pub mod my;
pub mod tag;
pub mod track;
pub mod user;
pub mod worker;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::sync::Arc;

use crate::cli::{
    error::Result,
    traits::{ApiClient, Command, CommandArgs, CommandOutput, OutputMetadata},
};

/// Base command implementation with common functionality
pub struct BaseCommand {
    pub name: String,
    pub description: String,
    pub api_client: Arc<dyn ApiClient>,
}

impl BaseCommand {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        api_client: Arc<dyn ApiClient>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            api_client,
        }
    }

    /// Execute an API call and return the result as CommandOutput
    pub async fn execute_api_call(
        &self,
        endpoint: &str,
        params: HashMap<String, String>,
    ) -> Result<CommandOutput> {
        let start = std::time::Instant::now();

        let data = self.api_client.get(endpoint, &params).await?;

        let metadata = OutputMetadata {
            cache_hit: false, // TODO: Get from response headers
            response_time_ms: start.elapsed().as_millis() as u64,
            api_calls_made: 1,
        };

        Ok(CommandOutput { data, metadata })
    }
}

/// Command registry for looking up commands
pub struct CommandRegistry {
    commands: HashMap<String, Box<dyn Command>>,
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    /// Register a command
    pub fn register(&mut self, command: Box<dyn Command>) {
        self.commands.insert(command.name().to_string(), command);
    }

    /// Get a command by name
    pub fn get(&self, name: &str) -> Option<&dyn Command> {
        self.commands.get(name).map(|c| c.as_ref())
    }

    /// Get all registered commands
    pub fn all(&self) -> Vec<&dyn Command> {
        self.commands.values().map(|c| c.as_ref()).collect()
    }

    /// Create a registry with all default commands
    pub fn with_defaults(api_client: Arc<dyn ApiClient>) -> Self {
        let mut registry = Self::new();

        // Register artist commands
        registry.register(Box::new(artist::ArtistInfoCommand::new(api_client.clone())));
        registry.register(Box::new(artist::ArtistSimilarCommand::new(
            api_client.clone(),
        )));
        registry.register(Box::new(artist::ArtistTopAlbumsCommand::new(
            api_client.clone(),
        )));
        registry.register(Box::new(artist::ArtistTopTracksCommand::new(
            api_client.clone(),
        )));
        registry.register(Box::new(artist::ArtistSearchCommand::new(
            api_client.clone(),
        )));

        // Register album commands
        registry.register(Box::new(album::AlbumInfoCommand::new(api_client.clone())));
        registry.register(Box::new(album::AlbumSearchCommand::new(api_client.clone())));

        // Register track commands
        registry.register(Box::new(track::TrackInfoCommand::new(api_client.clone())));
        registry.register(Box::new(track::TrackSimilarCommand::new(
            api_client.clone(),
        )));
        registry.register(Box::new(track::TrackSearchCommand::new(api_client.clone())));

        // Register chart commands
        registry.register(Box::new(chart::ChartTopArtistsCommand::new(
            api_client.clone(),
        )));
        registry.register(Box::new(chart::ChartTopTracksCommand::new(
            api_client.clone(),
        )));
        registry.register(Box::new(chart::ChartTopTagsCommand::new(
            api_client.clone(),
        )));

        // Register geo commands
        registry.register(Box::new(geo::GeoTopArtistsCommand::new(api_client.clone())));
        registry.register(Box::new(geo::GeoTopTracksCommand::new(api_client.clone())));

        // Register tag commands
        registry.register(Box::new(tag::TagInfoCommand::new(api_client.clone())));
        registry.register(Box::new(tag::TagTopArtistsCommand::new(api_client.clone())));
        registry.register(Box::new(tag::TagTopAlbumsCommand::new(api_client.clone())));
        registry.register(Box::new(tag::TagTopTracksCommand::new(api_client.clone())));

        // Register user commands
        registry.register(Box::new(user::UserInfoCommand::new(api_client.clone())));
        registry.register(Box::new(user::UserTopArtistsCommand::new(
            api_client.clone(),
        )));
        registry.register(Box::new(user::UserRecentTracksCommand::new(
            api_client.clone(),
        )));

        // Register library commands
        registry.register(Box::new(library::LibraryArtistsCommand::new(
            api_client.clone(),
        )));

        registry
    }

    /// Create a registry with all default commands including auth
    pub fn with_defaults_and_auth(
        api_client: Arc<dyn ApiClient>,
        config_manager: crate::cli::config::ConfigManager,
    ) -> Self {
        let mut registry = Self::with_defaults(api_client.clone());

        // Register auth commands
        registry.register(Box::new(auth::AuthLoginCommand::new(
            api_client.clone(),
            config_manager.clone(),
        )));
        registry.register(Box::new(auth::AuthStatusCommand::new(
            api_client.clone(),
            config_manager.clone(),
        )));
        registry.register(Box::new(auth::AuthLogoutCommand::new(
            api_client.clone(),
            config_manager.clone(),
        )));

        // Register "my" commands for authenticated users
        registry.register(Box::new(my::MyRecentTracksCommand::new(
            api_client.clone(),
            config_manager.clone(),
        )));
        registry.register(Box::new(my::MyTopArtistsCommand::new(
            api_client.clone(),
            config_manager.clone(),
        )));
        registry.register(Box::new(my::MyTopTracksCommand::new(
            api_client.clone(),
            config_manager.clone(),
        )));
        registry.register(Box::new(my::MyTopAlbumsCommand::new(
            api_client.clone(),
            config_manager.clone(),
        )));
        registry.register(Box::new(my::MyInfoCommand::new(
            api_client.clone(),
            config_manager.clone(),
        )));

        // Register configuration commands
        registry.register(Box::new(config::SetConfigCommand::new(
            config_manager.clone(),
        )));
        registry.register(Box::new(config::GetConfigCommand::new(
            config_manager.clone(),
        )));
        registry.register(Box::new(config::ListConfigCommand::new(config_manager)));

        registry
    }
}

/// Helper to extract a required argument
pub fn get_required_arg(args: &CommandArgs, name: &str) -> Result<String> {
    args.named
        .get(name)
        .cloned()
        .or_else(|| args.positional.first().cloned())
        .ok_or_else(|| crate::cli::error::CliError::missing_argument(name))
}

/// Helper to extract an optional argument
pub fn get_optional_arg(args: &CommandArgs, name: &str, default: Option<&str>) -> String {
    args.named
        .get(name)
        .cloned()
        .or_else(|| default.map(|s| s.to_string()))
        .unwrap_or_default()
}

/// Helper to extract a flag
pub fn get_flag(args: &CommandArgs, name: &str) -> bool {
    args.flags.get(name).copied().unwrap_or(false)
}
