// Last.fm CLI - Main entry point

use clap::{Arg, Command};
use lastfm_proxy_worker::cli::{
    api::LastfmApiClient,
    commands::CommandRegistry,
    config::{CliConfig, ConfigManager},
    error::{CliError, Result},
    output::{create_formatter, OutputFormat},
    traits::{ApiClient, CommandArgs, Configurable},
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let app = build_cli();
    let matches = app.get_matches();

    // Load configuration
    let config_manager = ConfigManager::new()?;
    let mut config = config_manager.load().await?;

    config.worker_url =
        resolve_worker_url(config.worker_url, matches.get_one::<String>("worker-url"));

    if config.worker_url.trim().is_empty() {
        if let Some((category, _)) = matches.subcommand() {
            if category != "config" {
                eprintln!("{}", worker_not_configured_message());
                std::process::exit(1);
            }
        }
    }

    if let Some(output) = matches.get_one::<String>("output") {
        config.output_format = match output.as_str() {
            "json" => OutputFormat::Json,
            "table" => OutputFormat::Table,
            "pretty" => OutputFormat::Pretty,
            "compact" => OutputFormat::Compact,
            _ => config.output_format,
        };
    }

    // Create API client
    let api_client: Arc<dyn ApiClient> = Arc::new(LastfmApiClient::new(config.worker_url.clone()));

    // Create command registry with auth support
    let registry =
        CommandRegistry::with_defaults_and_auth(api_client.clone(), config_manager.clone());

    // Handle subcommands
    match matches.subcommand() {
        Some((category, sub_matches)) => {
            handle_category_command(category, sub_matches, &registry, &config).await?;
        }
        None => {
            eprintln!("No command specified. Use --help for usage information.");
        }
    }

    Ok(())
}

/// Resolves the backend worker URL using the precedence
/// `--worker-url` flag > `LASTFM_WORKER_URL` env var > stored config value.
fn resolve_worker_url(config_value: String, flag: Option<&String>) -> String {
    if let Some(flag_url) = flag {
        return flag_url.clone();
    }

    if let Ok(env_url) = std::env::var("LASTFM_WORKER_URL") {
        if !env_url.trim().is_empty() {
            return env_url.trim().to_string();
        }
    }

    config_value
}

/// Guidance shown when no worker backend has been configured yet.
fn worker_not_configured_message() -> String {
    "Error: no Last.fm worker backend is configured.\n\n\
     lastfm-cli talks to a Cloudflare Worker that you host yourself. Deploy one\n\
     (see https://github.com/guitaripod/lastfm-rs#self-hosting), then point the CLI at it:\n\n\
     \x20\x20export LASTFM_WORKER_URL=https://your-worker.workers.dev\n\
     \x20\x20# or persist it:\n\
     \x20\x20lastfm-cli config set worker_url https://your-worker.workers.dev"
        .to_string()
}

fn build_cli() -> Command {
    Command::new("lastfm-cli")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Last.fm Proxy Worker CLI")
        .about("Command-line interface for Last.fm Proxy Worker")
        .arg_required_else_help(true)
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .help("Output format")
                .value_parser(["json", "table", "pretty", "compact"])
                .global(true),
        )
        .arg(
            Arg::new("worker-url")
                .long("worker-url")
                .help("Worker URL")
                .global(true),
        )
        .subcommand(build_artist_command())
        .subcommand(build_album_command())
        .subcommand(build_track_command())
        .subcommand(build_chart_command())
        .subcommand(build_geo_command())
        .subcommand(build_tag_command())
        .subcommand(build_user_command())
        .subcommand(build_library_command())
        .subcommand(build_auth_command())
        .subcommand(build_my_command())
        .subcommand(build_config_command())
}

fn build_config_command() -> Command {
    Command::new("config")
        .about("Manage CLI configuration")
        .subcommand(
            Command::new("set")
                .about("Set a configuration value")
                .arg(
                    Arg::new("key")
                        .help("Config key (e.g. worker_url)")
                        .required(true),
                )
                .arg(Arg::new("value").help("Config value").required(true)),
        )
        .subcommand(
            Command::new("get")
                .about("Get a configuration value")
                .arg(Arg::new("key").help("Config key").required(true)),
        )
        .subcommand(Command::new("list").about("List all configuration values"))
}

fn build_artist_command() -> Command {
    Command::new("artist")
        .about("Artist-related commands")
        .subcommand(
            Command::new("info")
                .about("Get artist information")
                .arg(Arg::new("artist").help("Artist name").required(true))
                .arg(Arg::new("mbid").help("MusicBrainz ID").long("mbid"))
                .arg(
                    Arg::new("autocorrect")
                        .help("Autocorrect")
                        .long("autocorrect")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("similar")
                .about("Get similar artists")
                .arg(Arg::new("artist").help("Artist name").required(true))
                .arg(
                    Arg::new("limit")
                        .help("Limit")
                        .long("limit")
                        .default_value("50"),
                ),
        )
        .subcommand(
            Command::new("search")
                .about("Search for artists")
                .arg(Arg::new("query").help("Search query").required(true))
                .arg(
                    Arg::new("limit")
                        .help("Limit")
                        .long("limit")
                        .default_value("30"),
                )
                .arg(
                    Arg::new("page")
                        .help("Page")
                        .long("page")
                        .default_value("1"),
                ),
        )
        .subcommand(
            Command::new("top-albums")
                .about("Get top albums for an artist")
                .arg(Arg::new("artist").help("Artist name").required(true))
                .arg(
                    Arg::new("limit")
                        .help("Limit")
                        .long("limit")
                        .default_value("50"),
                )
                .arg(
                    Arg::new("page")
                        .help("Page")
                        .long("page")
                        .default_value("1"),
                ),
        )
        .subcommand(
            Command::new("top-tracks")
                .about("Get top tracks for an artist")
                .arg(Arg::new("artist").help("Artist name").required(true))
                .arg(
                    Arg::new("limit")
                        .help("Limit")
                        .long("limit")
                        .default_value("50"),
                )
                .arg(
                    Arg::new("page")
                        .help("Page")
                        .long("page")
                        .default_value("1"),
                ),
        )
}

fn build_album_command() -> Command {
    Command::new("album")
        .about("Album-related commands")
        .subcommand(
            Command::new("info")
                .about("Get album information")
                .arg(Arg::new("artist").help("Artist name").required(true))
                .arg(Arg::new("album").help("Album name").required(true)),
        )
        .subcommand(
            Command::new("search")
                .about("Search for albums")
                .arg(Arg::new("query").help("Search query").required(true))
                .arg(
                    Arg::new("limit")
                        .help("Limit")
                        .long("limit")
                        .default_value("30"),
                )
                .arg(
                    Arg::new("page")
                        .help("Page")
                        .long("page")
                        .default_value("1"),
                ),
        )
}

fn build_track_command() -> Command {
    Command::new("track")
        .about("Track-related commands")
        .subcommand(
            Command::new("info")
                .about("Get track information")
                .arg(Arg::new("artist").help("Artist name").required(true))
                .arg(Arg::new("track").help("Track name").required(true)),
        )
        .subcommand(
            Command::new("similar")
                .about("Get similar tracks")
                .arg(Arg::new("artist").help("Artist name").required(true))
                .arg(Arg::new("track").help("Track name").required(true))
                .arg(
                    Arg::new("limit")
                        .help("Limit")
                        .long("limit")
                        .default_value("50"),
                ),
        )
        .subcommand(
            Command::new("search")
                .about("Search for tracks")
                .arg(Arg::new("query").help("Search query").required(true))
                .arg(
                    Arg::new("limit")
                        .help("Limit")
                        .long("limit")
                        .default_value("30"),
                )
                .arg(
                    Arg::new("page")
                        .help("Page")
                        .long("page")
                        .default_value("1"),
                ),
        )
}

fn build_chart_command() -> Command {
    Command::new("chart")
        .about("Chart commands")
        .subcommand(
            Command::new("top-artists")
                .about("Get top artists chart")
                .arg(
                    Arg::new("limit")
                        .help("Limit")
                        .long("limit")
                        .default_value("50"),
                )
                .arg(
                    Arg::new("page")
                        .help("Page")
                        .long("page")
                        .default_value("1"),
                ),
        )
        .subcommand(
            Command::new("top-tracks")
                .about("Get top tracks chart")
                .arg(
                    Arg::new("limit")
                        .help("Limit")
                        .long("limit")
                        .default_value("50"),
                )
                .arg(
                    Arg::new("page")
                        .help("Page")
                        .long("page")
                        .default_value("1"),
                ),
        )
        .subcommand(
            Command::new("top-tags")
                .about("Get top tags chart")
                .arg(
                    Arg::new("limit")
                        .help("Limit")
                        .long("limit")
                        .default_value("50"),
                )
                .arg(
                    Arg::new("page")
                        .help("Page")
                        .long("page")
                        .default_value("1"),
                ),
        )
}

fn build_geo_command() -> Command {
    Command::new("geo")
        .about("Geographic commands")
        .subcommand(
            Command::new("top-artists")
                .about("Get top artists by country")
                .arg(Arg::new("country").help("Country").required(true))
                .arg(
                    Arg::new("limit")
                        .help("Limit")
                        .long("limit")
                        .default_value("50"),
                )
                .arg(
                    Arg::new("page")
                        .help("Page")
                        .long("page")
                        .default_value("1"),
                ),
        )
        .subcommand(
            Command::new("top-tracks")
                .about("Get top tracks by country")
                .arg(Arg::new("country").help("Country").required(true))
                .arg(
                    Arg::new("limit")
                        .help("Limit")
                        .long("limit")
                        .default_value("50"),
                )
                .arg(
                    Arg::new("page")
                        .help("Page")
                        .long("page")
                        .default_value("1"),
                ),
        )
}

fn build_tag_command() -> Command {
    Command::new("tag")
        .about("Tag commands")
        .subcommand(
            Command::new("info")
                .about("Get tag information")
                .arg(Arg::new("tag").help("Tag name").required(true)),
        )
        .subcommand(
            Command::new("top-artists")
                .about("Get top artists for a tag")
                .arg(Arg::new("tag").help("Tag name").required(true))
                .arg(
                    Arg::new("limit")
                        .help("Limit")
                        .long("limit")
                        .default_value("50"),
                )
                .arg(
                    Arg::new("page")
                        .help("Page")
                        .long("page")
                        .default_value("1"),
                ),
        )
        .subcommand(
            Command::new("top-albums")
                .about("Get top albums for a tag")
                .arg(Arg::new("tag").help("Tag name").required(true))
                .arg(
                    Arg::new("limit")
                        .help("Limit")
                        .long("limit")
                        .default_value("50"),
                )
                .arg(
                    Arg::new("page")
                        .help("Page")
                        .long("page")
                        .default_value("1"),
                ),
        )
        .subcommand(
            Command::new("top-tracks")
                .about("Get top tracks for a tag")
                .arg(Arg::new("tag").help("Tag name").required(true))
                .arg(
                    Arg::new("limit")
                        .help("Limit")
                        .long("limit")
                        .default_value("50"),
                )
                .arg(
                    Arg::new("page")
                        .help("Page")
                        .long("page")
                        .default_value("1"),
                ),
        )
}

fn build_user_command() -> Command {
    Command::new("user")
        .about("User commands")
        .subcommand(
            Command::new("info")
                .about("Get user information")
                .arg(Arg::new("user").help("Username").required(true)),
        )
        .subcommand(
            Command::new("recent-tracks")
                .about("Get recent tracks")
                .arg(Arg::new("user").help("Username").required(true))
                .arg(
                    Arg::new("extended")
                        .help("Extended info")
                        .long("extended")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("limit")
                        .help("Limit")
                        .long("limit")
                        .default_value("50"),
                )
                .arg(
                    Arg::new("page")
                        .help("Page")
                        .long("page")
                        .default_value("1"),
                ),
        )
        .subcommand(
            Command::new("top-artists")
                .about("Get user's top artists")
                .arg(Arg::new("user").help("Username").required(true))
                .arg(
                    Arg::new("period")
                        .help("Time period")
                        .long("period")
                        .default_value("overall")
                        .value_parser(["overall", "7day", "1month", "3month", "6month", "12month"]),
                )
                .arg(
                    Arg::new("limit")
                        .help("Limit")
                        .long("limit")
                        .default_value("50"),
                )
                .arg(
                    Arg::new("page")
                        .help("Page")
                        .long("page")
                        .default_value("1"),
                ),
        )
}

fn build_library_command() -> Command {
    Command::new("library")
        .about("Library commands")
        .subcommand(
            Command::new("artists")
                .about("Get user's library artists")
                .arg(Arg::new("user").help("Username").required(true)),
        )
}

fn build_auth_command() -> Command {
    Command::new("auth")
        .about("Authentication commands")
        .subcommand(Command::new("login").about("Authenticate with Last.fm"))
        .subcommand(Command::new("status").about("Check authentication status"))
        .subcommand(Command::new("logout").about("Log out and clear session"))
}

fn build_my_command() -> Command {
    Command::new("my")
        .about("Commands for authenticated user (requires login)")
        .subcommand(Command::new("info").about("Get your user information"))
        .subcommand(
            Command::new("recent-tracks")
                .about("Get your recent tracks")
                .arg(
                    Arg::new("extended")
                        .help("Include extended info")
                        .long("extended")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("limit")
                        .help("Limit")
                        .long("limit")
                        .default_value("50"),
                )
                .arg(
                    Arg::new("page")
                        .help("Page")
                        .long("page")
                        .default_value("1"),
                ),
        )
        .subcommand(
            Command::new("top-artists")
                .about("Get your top artists")
                .arg(
                    Arg::new("period")
                        .help("Time period")
                        .long("period")
                        .default_value("overall")
                        .value_parser(["overall", "7day", "1month", "3month", "6month", "12month"]),
                )
                .arg(
                    Arg::new("limit")
                        .help("Limit")
                        .long("limit")
                        .default_value("50"),
                )
                .arg(
                    Arg::new("page")
                        .help("Page")
                        .long("page")
                        .default_value("1"),
                ),
        )
        .subcommand(
            Command::new("top-tracks")
                .about("Get your top tracks")
                .arg(
                    Arg::new("period")
                        .help("Time period")
                        .long("period")
                        .default_value("overall")
                        .value_parser(["overall", "7day", "1month", "3month", "6month", "12month"]),
                )
                .arg(
                    Arg::new("limit")
                        .help("Limit")
                        .long("limit")
                        .default_value("50"),
                )
                .arg(
                    Arg::new("page")
                        .help("Page")
                        .long("page")
                        .default_value("1"),
                ),
        )
        .subcommand(
            Command::new("top-albums")
                .about("Get your top albums")
                .arg(
                    Arg::new("period")
                        .help("Time period")
                        .long("period")
                        .default_value("overall")
                        .value_parser(["overall", "7day", "1month", "3month", "6month", "12month"]),
                )
                .arg(
                    Arg::new("limit")
                        .help("Limit")
                        .long("limit")
                        .default_value("50"),
                )
                .arg(
                    Arg::new("page")
                        .help("Page")
                        .long("page")
                        .default_value("1"),
                ),
        )
}

async fn handle_category_command(
    category: &str,
    matches: &clap::ArgMatches,
    registry: &CommandRegistry,
    config: &CliConfig,
) -> Result<()> {
    let (command_name, args) = match matches.subcommand() {
        Some((subcmd, sub_matches)) => {
            let full_name = format!("{category}.{subcmd}");
            let args = extract_args(sub_matches);
            (full_name, args)
        }
        None => return Err(CliError::invalid_command("No subcommand specified")),
    };

    // Find and execute command
    let command = registry
        .get(&command_name)
        .ok_or_else(|| CliError::invalid_command(format!("Unknown command: {command_name}")))?;

    // Validate arguments
    command.validate_args(&args)?;

    // Execute command
    let output = command.execute(&args).await?;

    // Format and display output
    let formatter = create_formatter(config.output_format, config.color_output);
    println!("{}", formatter.format(&output));

    Ok(())
}

fn extract_args(matches: &clap::ArgMatches) -> CommandArgs {
    let mut args = CommandArgs::default();

    // Known flags to check
    let flag_names = ["autocorrect", "extended"];

    // Extract flags - check if they are present
    for flag in flag_names {
        if matches.try_contains_id(flag).unwrap_or(false) {
            args.flags.insert(flag.to_string(), true);
        }
    }

    // Extract string arguments
    for id in matches.ids() {
        let key = id.as_str();

        // Skip if it's a known flag
        if flag_names.contains(&key) {
            continue;
        }

        // Try to get as string
        if let Some(value) = matches.get_one::<String>(key) {
            args.named.insert(key.to_string(), value.clone());
        }
    }

    args
}
