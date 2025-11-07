#[cfg(not(target_arch = "wasm32"))]
mod cli;
#[cfg(not(target_arch = "wasm32"))]
mod server_mode;

// The binary is only compiled for native targets, not WASM
#[cfg(not(target_arch = "wasm32"))]
use clap::Parser;
#[cfg(not(target_arch = "wasm32"))]
use cli::{Args, Command};
#[cfg(not(target_arch = "wasm32"))]
use hegel_pm::discovery::{DiscoveryConfig, DiscoveryEngine};
#[cfg(not(target_arch = "wasm32"))]
use tracing::Level;
#[cfg(not(target_arch = "wasm32"))]
use tracing_subscriber::{fmt, EnvFilter};

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber with env filter
    // Default to INFO level, can be overridden with RUST_LOG env var
    // Examples:
    //   RUST_LOG=debug hegel-pm
    //   RUST_LOG=hegel_pm::server_mode=trace hegel-pm
    fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
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
        Some(Command::X { args: hegel_args }) => {
            // Run hegel command across all projects
            cli::hegel::run(&engine, &hegel_args)?;
        }
        Some(Command::Benchmark { iterations, json }) => {
            // Benchmark mode: start server in background and run benchmarks
            use hegel_pm::benchmark_mode;

            // Start server in background
            let engine_clone = engine.clone();
            tokio::spawn(async move {
                if let Err(e) = server_mode::run(&engine_clone).await {
                    eprintln!("Server error: {}", e);
                }
            });

            // Run benchmarks
            benchmark_mode::run(&engine, iterations, json).await?;
        }
        None => {
            // Server mode: start web server
            server_mode::run(&engine).await?;
        }
    }

    Ok(())
}

// Dummy main for WASM builds (binary is never actually used for WASM)
#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("This binary is not meant to be compiled for WASM");
}
