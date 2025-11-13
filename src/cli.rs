pub mod discover;
pub mod hegel;

use clap::{Parser, Subcommand};

/// Hegel Project Manager - CLI for discovering and managing Hegel projects
#[derive(Parser, Debug)]
#[command(name = "hegel-pm")]
#[command(about = "CLI and library for discovering and managing Hegel projects", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Discover and inspect Hegel projects
    Discover {
        #[command(subcommand)]
        subcommand: DiscoverCommand,

        /// Output as JSON instead of human-readable format
        #[arg(long, global = true)]
        json: bool,

        /// Force fresh filesystem scan, bypass cache
        #[arg(long, global = true)]
        no_cache: bool,
    },

    /// Remove a project from tracking (clears from cache)
    Remove {
        /// Name of the project to remove
        project_name: String,
    },

    /// Refresh cached data for project(s)
    Refresh {
        /// Names of projects to refresh (omit to refresh all cached projects)
        project_names: Vec<String>,
    },

    /// Run a hegel command across all discovered projects
    X {
        /// Arguments to pass to hegel command
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum DiscoverCommand {
    /// List all discovered projects (lightweight, no metrics)
    List,

    /// Show detailed information for a specific project
    Show {
        /// Name of the project to show
        project_name: String,
    },

    /// Show aggregate metrics for all projects in a table
    All {
        /// Column to sort by (name, path, size, last-activity, tokens, events, phases, load-time)
        #[arg(long, default_value = "last-activity")]
        sort_by: String,

        /// Include load time column for performance profiling
        #[arg(long)]
        benchmark: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_subcommand() {
        let args = Args::parse_from(["hegel-pm", "discover", "list"]);
        assert!(matches!(
            args.command,
            Some(Command::Discover {
                subcommand: DiscoverCommand::List,
                ..
            })
        ));
    }

    #[test]
    fn test_show_subcommand() {
        let args = Args::parse_from(["hegel-pm", "discover", "show", "my-project"]);
        match args.command {
            Some(Command::Discover {
                subcommand: DiscoverCommand::Show { project_name },
                ..
            }) => {
                assert_eq!(project_name, "my-project");
            }
            _ => panic!("Expected Show subcommand"),
        }
    }

    #[test]
    fn test_all_subcommand_defaults() {
        let args = Args::parse_from(["hegel-pm", "discover", "all"]);
        match args.command {
            Some(Command::Discover {
                subcommand: DiscoverCommand::All { sort_by, benchmark },
                ..
            }) => {
                assert_eq!(sort_by, "last-activity");
                assert!(!benchmark);
            }
            _ => panic!("Expected All subcommand"),
        }
    }

    #[test]
    fn test_all_subcommand_with_options() {
        let args = Args::parse_from([
            "hegel-pm",
            "discover",
            "all",
            "--sort-by",
            "tokens",
            "--benchmark",
        ]);
        match args.command {
            Some(Command::Discover {
                subcommand: DiscoverCommand::All { sort_by, benchmark },
                ..
            }) => {
                assert_eq!(sort_by, "tokens");
                assert!(benchmark);
            }
            _ => panic!("Expected All subcommand"),
        }
    }

    #[test]
    fn test_global_json_flag() {
        let args = Args::parse_from(["hegel-pm", "discover", "--json", "list"]);
        match args.command {
            Some(Command::Discover { json, .. }) => {
                assert!(json);
            }
            _ => panic!("Expected Discover command"),
        }
    }

    #[test]
    fn test_global_no_cache_flag() {
        let args = Args::parse_from(["hegel-pm", "discover", "--no-cache", "list"]);
        match args.command {
            Some(Command::Discover { no_cache, .. }) => {
                assert!(no_cache);
            }
            _ => panic!("Expected Discover command"),
        }
    }

    #[test]
    fn test_hegel_command() {
        let args = Args::parse_from(["hegel-pm", "x", "status"]);
        match args.command {
            Some(Command::X { args }) => {
                assert_eq!(args, vec!["status"]);
            }
            _ => panic!("Expected X command"),
        }
    }

    #[test]
    fn test_hegel_command_with_multiple_args() {
        let args = Args::parse_from(["hegel-pm", "x", "analyze", "--fix-archives", "--dry-run"]);
        match args.command {
            Some(Command::X { args }) => {
                assert_eq!(args, vec!["analyze", "--fix-archives", "--dry-run"]);
            }
            _ => panic!("Expected X command"),
        }
    }

    #[test]
    fn test_hegel_command_with_flags() {
        let args = Args::parse_from(["hegel-pm", "x", "analyze", "--fix-archives", "--json"]);
        match args.command {
            Some(Command::X { args }) => {
                assert_eq!(args, vec!["analyze", "--fix-archives", "--json"]);
            }
            _ => panic!("Expected X command"),
        }
    }

    #[test]
    fn test_remove_command() {
        let args = Args::parse_from(["hegel-pm", "remove", "my-project"]);
        match args.command {
            Some(Command::Remove { project_name }) => {
                assert_eq!(project_name, "my-project");
            }
            _ => panic!("Expected Remove command"),
        }
    }

    #[test]
    fn test_refresh_command_single() {
        let args = Args::parse_from(["hegel-pm", "refresh", "my-project"]);
        match args.command {
            Some(Command::Refresh { project_names }) => {
                assert_eq!(project_names, vec!["my-project"]);
            }
            _ => panic!("Expected Refresh command"),
        }
    }

    #[test]
    fn test_refresh_command_multiple() {
        let args = Args::parse_from(["hegel-pm", "refresh", "project1", "project2", "project3"]);
        match args.command {
            Some(Command::Refresh { project_names }) => {
                assert_eq!(project_names, vec!["project1", "project2", "project3"]);
            }
            _ => panic!("Expected Refresh command"),
        }
    }

    #[test]
    fn test_refresh_command_no_args() {
        let args = Args::parse_from(["hegel-pm", "refresh"]);
        match args.command {
            Some(Command::Refresh { project_names }) => {
                assert!(project_names.is_empty());
            }
            _ => panic!("Expected Refresh command"),
        }
    }
}
