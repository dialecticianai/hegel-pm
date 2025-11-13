// Debug utilities (requires explicit import: use hegel_pm::debug;)
pub mod debug;

// Core library: project discovery
pub mod discovery;

// CLI commands
pub mod cli;

#[cfg(test)]
mod test_helpers;
