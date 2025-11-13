use clap::Parser;
use hegel_pm::cli::{Args, Command};
use hegel_pm::discovery::{DiscoveryConfig, DiscoveryEngine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
            // Discover subcommand: list, show, or all projects
            hegel_pm::cli::discover::run(&engine, &subcommand, json, no_cache)?;
        }
        Some(Command::X { args: hegel_args }) => {
            // Run hegel command across all projects
            hegel_pm::cli::hegel::run(&engine, &hegel_args)?;
        }
        None => {
            // No command specified - show help
            Args::parse_from(&["hegel-pm", "--help"]);
        }
    }

    Ok(())
}
