use clap::Parser;
use hegel_pm::cli::{Args, Command};
use hegel_pm::discovery::{remove_from_cache, DiscoveryConfig, DiscoveryEngine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize discovery engine with default config
    let config = DiscoveryConfig::default();

    match args.command {
        Some(Command::Discover {
            subcommand,
            json,
            no_cache,
        }) => {
            // Discover subcommand: list, show, or all projects
            let engine = DiscoveryEngine::new(config)?;
            hegel_pm::cli::discover::run(&engine, &subcommand, json, no_cache)?;
        }
        Some(Command::Remove { project_name }) => {
            // Remove project from cache
            let removed = remove_from_cache(&project_name, &config)?;
            if removed {
                println!("✓ Removed '{}' from tracking", project_name);
            } else {
                eprintln!("✗ Project '{}' not found in cache", project_name);
                std::process::exit(1);
            }
        }
        Some(Command::X { args: hegel_args }) => {
            // Run hegel command across all projects
            let engine = DiscoveryEngine::new(config)?;
            hegel_pm::cli::hegel::run(&engine, &hegel_args)?;
        }
        None => {
            // No command specified - show help
            Args::parse_from(&["hegel-pm", "--help"]);
        }
    }

    Ok(())
}
