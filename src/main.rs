use clap::Parser;
use hegel_pm::cli::{Args, Command};
use hegel_pm::discovery::{
    refresh_all_projects, refresh_project, remove_from_cache, DiscoveryConfig, DiscoveryEngine,
};

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
        Some(Command::Refresh { project_names }) => {
            if project_names.is_empty() {
                // Refresh all cached projects
                match refresh_all_projects(&config) {
                    Ok(count) => {
                        println!("✓ Refreshed {} project(s)", count);
                    }
                    Err(e) => {
                        eprintln!("✗ Failed to refresh projects: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                // Refresh specific projects
                let mut success_count = 0;
                let mut failed = Vec::new();

                for project_name in &project_names {
                    match refresh_project(project_name, &config) {
                        Ok(_) => {
                            println!("✓ Refreshed '{}'", project_name);
                            success_count += 1;
                        }
                        Err(e) => {
                            eprintln!("✗ Failed to refresh '{}': {}", project_name, e);
                            failed.push(project_name.clone());
                        }
                    }
                }

                if !failed.is_empty() {
                    eprintln!("\nFailed to refresh {} project(s)", failed.len());
                    std::process::exit(1);
                }

                if success_count > 0 {
                    println!("\n✓ Successfully refreshed {} project(s)", success_count);
                }
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
