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

    /// [DEPRECATED] Run discovery scan and print results (use 'discover list' instead)
    #[arg(long)]
    pub discover: bool,

    /// [DEPRECATED] Force refresh cache during discovery (use --no-cache instead)
    #[arg(long, requires = "discover")]
    pub refresh: bool,

    /// Run HTTP endpoint benchmarks and exit
    #[arg(long)]
    pub run_benchmarks: bool,

    /// Number of iterations per endpoint (default: 100)
    #[arg(long, requires = "run_benchmarks", default_value = "100")]
    pub benchmark_iterations: usize,

    /// Output benchmark results as JSON
    #[arg(long, requires = "run_benchmarks")]
    pub benchmark_json: bool,
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
    Hegel {
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
    fn test_deprecated_discover_flag() {
        let args = Args::parse_from(["hegel-pm", "--discover"]);
        assert!(args.discover);
        assert!(args.command.is_none());
    }

    #[test]
    fn test_deprecated_refresh_flag() {
        let args = Args::parse_from(["hegel-pm", "--discover", "--refresh"]);
        assert!(args.discover);
        assert!(args.refresh);
    }

    #[test]
    fn test_hegel_command() {
        let args = Args::parse_from(["hegel-pm", "hegel", "status"]);
        match args.command {
            Some(Command::Hegel { args }) => {
                assert_eq!(args, vec!["status"]);
            }
            _ => panic!("Expected Hegel command"),
        }
    }

    #[test]
    fn test_hegel_command_with_multiple_args() {
        let args = Args::parse_from([
            "hegel-pm",
            "hegel",
            "analyze",
            "--fix-archives",
            "--dry-run",
        ]);
        match args.command {
            Some(Command::Hegel { args }) => {
                assert_eq!(args, vec!["analyze", "--fix-archives", "--dry-run"]);
            }
            _ => panic!("Expected Hegel command"),
        }
    }

    #[test]
    fn test_hegel_command_with_flags() {
        let args = Args::parse_from(["hegel-pm", "hegel", "analyze", "--fix-archives", "--json"]);
        match args.command {
            Some(Command::Hegel { args }) => {
                assert_eq!(args, vec!["analyze", "--fix-archives", "--json"]);
            }
            _ => panic!("Expected Hegel command"),
        }
    }

    #[test]
    fn test_benchmark_flag() {
        let args = Args::parse_from(["hegel-pm", "--run-benchmarks"]);
        assert!(args.run_benchmarks);
        assert_eq!(args.benchmark_iterations, 100); // default
        assert!(!args.benchmark_json);
    }

    #[test]
    fn test_benchmark_with_custom_iterations() {
        let args = Args::parse_from([
            "hegel-pm",
            "--run-benchmarks",
            "--benchmark-iterations",
            "50",
        ]);
        assert!(args.run_benchmarks);
        assert_eq!(args.benchmark_iterations, 50);
    }

    #[test]
    fn test_benchmark_with_json() {
        let args = Args::parse_from(["hegel-pm", "--run-benchmarks", "--benchmark-json"]);
        assert!(args.run_benchmarks);
        assert!(args.benchmark_json);
    }

    #[test]
    fn test_benchmark_all_flags() {
        let args = Args::parse_from([
            "hegel-pm",
            "--run-benchmarks",
            "--benchmark-iterations",
            "200",
            "--benchmark-json",
        ]);
        assert!(args.run_benchmarks);
        assert_eq!(args.benchmark_iterations, 200);
        assert!(args.benchmark_json);
    }
}
