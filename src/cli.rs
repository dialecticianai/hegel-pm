use clap::Parser;

/// Hegel Project Manager - Multi-project workflow orchestration
#[derive(Parser, Debug)]
#[command(name = "hegel-pm")]
#[command(about = "Project manager for Hegel projects with web UI", long_about = None)]
pub struct Args {
    /// Run discovery scan and print results (don't start server)
    #[arg(long)]
    pub discover: bool,

    /// Force refresh cache during discovery
    #[arg(long)]
    pub refresh: bool,
}
