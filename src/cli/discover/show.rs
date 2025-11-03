use hegel_pm::discovery::DiscoveryEngine;
use std::error::Error;

/// Run the show command
pub fn run(
    _engine: &DiscoveryEngine,
    _project_name: &str,
    _json: bool,
    _no_cache: bool,
) -> Result<(), Box<dyn Error>> {
    // TODO: Implement in Step 3
    println!("Show command - not yet implemented");
    Ok(())
}
