use hegel_pm::discovery::DiscoveryEngine;
use std::error::Error;

/// Run the all command
pub fn run(
    _engine: &DiscoveryEngine,
    _sort_by: &str,
    _benchmark: bool,
    _json: bool,
    _no_cache: bool,
) -> Result<(), Box<dyn Error>> {
    // TODO: Implement in Step 4
    println!("All command - not yet implemented");
    Ok(())
}
