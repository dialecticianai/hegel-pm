mod cli;
mod discovery_mode;
mod server_mode;

use clap::Parser;
use cli::Args;
use hegel_pm::discovery::{DiscoveryConfig, DiscoveryEngine};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize discovery engine with default config
    let config = DiscoveryConfig::default();
    let engine = DiscoveryEngine::new(config)?;

    if args.discover {
        // Discovery mode: scan and print results
        discovery_mode::run(&engine, args.refresh)?;
    } else {
        // Server mode: start web server
        server_mode::run(&engine).await?;
    }

    Ok(())
}
