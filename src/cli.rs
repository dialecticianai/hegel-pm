pub mod discover;
pub mod hegel;

use clap::{Parser, Subcommand};

/// Hegel Project Manager - Multi-project workflow orchestration
#[derive(Parser, Debug)]
#[command(name = "hegel-pm")]
#[command(about = "Project manager for Hegel projects with web UI", long_about = None)]
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

    /// Run a hegel command across all discovered projects
    X {
        /// Arguments to pass to hegel command
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Run HTTP endpoint benchmarks
    Benchmark {
        /// Number of iterations per endpoint
        #[arg(long, default_value = "100")]
        iterations: usize,

        /// Output results as JSON
        #[arg(long)]
        json: bool,
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
    fn test_benchmark_command() {
        let args = Args::parse_from(["hegel-pm", "benchmark"]);
        match args.command {
            Some(Command::Benchmark { iterations, json }) => {
                assert_eq!(iterations, 100); // default
                assert!(!json);
            }
            _ => panic!("Expected Benchmark command"),
        }
    }

    #[test]
    fn test_benchmark_with_custom_iterations() {
        let args = Args::parse_from(["hegel-pm", "benchmark", "--iterations", "50"]);
        match args.command {
            Some(Command::Benchmark { iterations, json }) => {
                assert_eq!(iterations, 50);
                assert!(!json);
            }
            _ => panic!("Expected Benchmark command"),
        }
    }

    #[test]
    fn test_benchmark_with_json() {
        let args = Args::parse_from(["hegel-pm", "benchmark", "--json"]);
        match args.command {
            Some(Command::Benchmark { iterations, json }) => {
                assert_eq!(iterations, 100);
                assert!(json);
            }
            _ => panic!("Expected Benchmark command"),
        }
    }

    #[test]
    fn test_benchmark_all_options() {
        let args = Args::parse_from(["hegel-pm", "benchmark", "--iterations", "200", "--json"]);
        match args.command {
            Some(Command::Benchmark { iterations, json }) => {
                assert_eq!(iterations, 200);
                assert!(json);
            }
            _ => panic!("Expected Benchmark command"),
        }
    }
}
