use hegel_pm::discovery::DiscoveryEngine;
use std::error::Error;
use std::process::Command;

/// Commands that should not be run across all projects
const DISALLOWED_COMMANDS: &[&str] = &[
    "top",     // Interactive TUI dashboard
    "reflect", // Interactive GUI review
];

/// Run a hegel command across all discovered projects
pub fn run(engine: &DiscoveryEngine, args: &[String]) -> Result<(), Box<dyn Error>> {
    // Validate we have at least a subcommand
    if args.is_empty() {
        return Err("No hegel command specified. Usage: hegel-pm hegel <command> [args...]".into());
    }

    // Check if the command is disallowed
    let subcommand = &args[0];
    if DISALLOWED_COMMANDS.contains(&subcommand.as_str()) {
        return Err(format!(
            "Command 'hegel {}' cannot be run across all projects (interactive/TUI command)\n\nDisallowed commands: {}",
            subcommand,
            DISALLOWED_COMMANDS.join(", ")
        )
        .into());
    }

    // Discover all projects (use cache)
    let projects = engine.get_projects(false)?;

    if projects.is_empty() {
        println!("No Hegel projects found");
        return Ok(());
    }

    println!(
        "Running 'hegel {}' across {} project(s)...\n",
        args.join(" "),
        projects.len()
    );

    let mut success_count = 0;
    let mut failure_count = 0;

    for project in &projects {
        println!("=== {} ===", project.name);
        println!("Path: {}", project.project_path.display());

        // Run hegel command with --state-dir pointing to this project's .hegel directory
        let mut cmd = Command::new("hegel");
        cmd.args(args);
        cmd.arg("--state-dir");
        cmd.arg(&project.hegel_dir);
        cmd.current_dir(&project.project_path);

        match cmd.output() {
            Ok(output) => {
                // Print stdout
                if !output.stdout.is_empty() {
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }

                // Print stderr
                if !output.stderr.is_empty() {
                    eprint!("{}", String::from_utf8_lossy(&output.stderr));
                }

                if output.status.success() {
                    success_count += 1;
                    println!("✓ Success\n");
                } else {
                    failure_count += 1;
                    println!("✗ Failed with exit code: {:?}\n", output.status.code());
                }
            }
            Err(e) => {
                failure_count += 1;
                eprintln!("✗ Failed to execute command: {}\n", e);
            }
        }
    }

    println!("=== Summary ===");
    println!("Total projects: {}", projects.len());
    println!("Succeeded: {}", success_count);
    println!("Failed: {}", failure_count);

    if failure_count > 0 {
        Err(format!("{} project(s) failed", failure_count).into())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disallowed_command_top() {
        let config = hegel_pm::discovery::DiscoveryConfig::default();
        let engine = DiscoveryEngine::new(config).unwrap();

        let result = run(&engine, &["top".to_string()]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("interactive/TUI"));
    }

    #[test]
    fn test_disallowed_command_reflect() {
        let config = hegel_pm::discovery::DiscoveryConfig::default();
        let engine = DiscoveryEngine::new(config).unwrap();

        let result = run(&engine, &["reflect".to_string(), "SPEC.md".to_string()]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("interactive/TUI"));
    }

    #[test]
    fn test_empty_args() {
        let config = hegel_pm::discovery::DiscoveryConfig::default();
        let engine = DiscoveryEngine::new(config).unwrap();

        let result = run(&engine, &[]);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No hegel command specified"));
    }

    #[test]
    fn test_allowed_commands() {
        // These should not error on validation (though they may fail if no projects exist)
        let allowed = vec![
            vec!["analyze".to_string()],
            vec!["status".to_string()],
            vec!["next".to_string()],
            vec![
                "analyze".to_string(),
                "--fix-archives".to_string(),
                "--dry-run".to_string(),
            ],
        ];

        for args in allowed {
            let subcommand = &args[0];
            assert!(
                !DISALLOWED_COMMANDS.contains(&subcommand.as_str()),
                "Command '{}' should be allowed",
                subcommand
            );
        }
    }
}
