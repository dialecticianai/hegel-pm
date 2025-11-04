mod cli;
mod discovery_mode;
mod server_mode;

use clap::Parser;
use cli::{Args, Command};
use hegel_pm::discovery::{DiscoveryConfig, DiscoveryEngine};
use tracing::{Level, warn};
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber with env filter
    // Default to INFO level, can be overridden with RUST_LOG env var
    // Examples:
    //   RUST_LOG=debug hegel-pm
    //   RUST_LOG=hegel_pm::server_mode=trace hegel-pm
    fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive(Level::INFO.into())
        )
        .init();

    let args = Args::parse();

    // Initialize discovery engine with default config
    let config = DiscoveryConfig::default();
    let engine = DiscoveryEngine::new(config)?;

    match args.command {
        Some(Command::Discover {
            subcommand,
            json,
            no_cache,
        }) => {
            // New discover subcommand
            cli::discover::run(&engine, &subcommand, json, no_cache)?;
        }
        Some(Command::Hegel { args: hegel_args }) => {
            // Run hegel command across all projects
            cli::hegel::run(&engine, &hegel_args)?;
        }
        None => {
            if args.discover {
                // Deprecated --discover flag
                warn!("⚠️  Warning: --discover flag is deprecated. Use 'discover list' instead.");
                discovery_mode::run(&engine, args.refresh)?;
            } else {
                // Server mode: start web server
                server_mode::run(&engine).await?;
            }
        }
    }

    Ok(())
}
