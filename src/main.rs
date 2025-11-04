mod cli;
mod discovery_mode;
mod server_mode;

use clap::Parser;
use cli::{Args, Command};
use hegel_pm::discovery::{DiscoveryConfig, DiscoveryEngine};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
                eprintln!("⚠️  Warning: --discover flag is deprecated. Use 'discover list' instead.");
                discovery_mode::run(&engine, args.refresh)?;
            } else {
                // Server mode: start web server
                server_mode::run(&engine).await?;
            }
        }
    }

    Ok(())
}
