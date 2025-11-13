# hegel-pm

Project discovery library and CLI for finding and tracking Hegel projects. Auto-discovers projects by walking the filesystem, extracts workflow state via hegel-cli, provides library API and CLI commands.

## Status

**Current**: Filesystem discovery with persistent caching, CLI commands for listing/querying projects
**Next**: Parallel discovery optimization, incremental update mechanisms

## Architecture

### Core Components

**Discovery Engine** (`src/discovery/`)
- Filesystem-based project discovery (walks configured roots to find `.hegel/` directories)
- Workflow state and metrics extraction via hegel-cli library
- Persistent binary cache for fast subsequent loads (<100ms vs ~2s first run)
- Configurable roots, max depth, and exclusions
- See `src/discovery/README.md` for detailed documentation

**CLI Commands** (`src/cli/`)
- Subcommand structure: `discover` (list, show, all), `hegel` (xargs-style)
- Thin wrappers around library API
- Human-readable and JSON output formats
- See `src/cli/README.md` for command documentation

**Dependencies**
- `hegel` (hegel-cli library) - All .hegel data access (state parsing, metrics extraction, JSONL handling)
- `walkdir` - Cross-platform recursive directory traversal
- `serde` + `serde_json` - Cache serialization
- `clap` - CLI argument parsing
- `anyhow` - Error handling
- `chrono` - Timestamp parsing

## Usage

### CLI

```bash
# Discovery commands
hegel-pm discover list              # List all projects (name + workflow state)
hegel-pm discover show <name>       # Show single project details
hegel-pm discover all               # Full table with metrics

# Cache management
hegel-pm remove <name>              # Remove project from cache (stop tracking)
hegel-pm refresh                    # Refresh all cached projects
hegel-pm refresh <name> [names...]  # Refresh specific project(s)

# Run hegel commands across all projects (xargs-style)
hegel-pm x status                   # Run 'hegel status' on each project
```

**Build from source:**
```bash
cargo build --release --bin hegel-pm
./target/release/hegel-pm
```

### Library API

```rust
use hegel_pm::discovery::{DiscoveryConfig, DiscoveryEngine};

// Create configuration
let config = DiscoveryConfig::default(); // Scans ~/Code

// Create engine
let engine = DiscoveryEngine::new(config)?;

// Get projects (uses cache if available)
let projects = engine.get_projects(false)?;

// Force refresh (bypass cache)
let projects = engine.get_projects(true)?;
```

### Configuration

Default configuration:
- **Root directories**: `~/Code`
- **Max depth**: 10 levels
- **Exclusions**: `node_modules`, `target`, `.git`, `vendor`
- **Cache location**: `~/.config/hegel-pm/cache.bin` (binary format)

Custom configuration:
```rust
let config = DiscoveryConfig::new(
    vec![PathBuf::from("/custom/path")],
    5,  // max depth
    vec!["build".to_string()],  // additional exclusions
    PathBuf::from("/tmp/cache.bin"),
);
```

## Development

### Scripts

**Build & Test:**
```bash
./scripts/test.sh    # Build + test
```

Use this for:
- Quick iteration during development
- Verifying changes work correctly
- CI/CD pipelines

**Manual Commands:**
```bash
cargo test                  # Run all tests
cargo test discovery        # Run discovery module tests only
cargo build --release       # Build CLI binary
```

Coverage: 31.64% (target: ≥80%, enforced by pre-commit hook)

### Project Structure

```
hegel-pm/
├── src/
│   ├── discovery/     # Project discovery engine
│   │   └── README.md      # Module structure documentation
│   ├── cli/           # CLI commands
│   │   └── README.md      # CLI commands documentation
│   ├── lib.rs         # Library root (exports discovery module)
│   ├── main.rs        # CLI entry point
│   ├── cli.rs         # CLI argument definitions
│   ├── debug.rs       # Debug utilities
│   ├── test_helpers.rs
│   └── README.md      # Source structure overview
├── scripts/           # Build and development scripts
│   └── README.md      # Scripts documentation
└── README.md
```

## Future Work

1. **Performance Optimization**:
   - Parallel filesystem walking for large workspaces
   - Incremental cache updates (only re-scan changed projects)
2. **Discovery Features**:
   - File watching integration for real-time discovery
   - Filtered discovery (by workflow state, last activity, etc.)
   - Project dependency graphing

## Error Handling

All errors include context with file paths for debugging:

- **Permission denied**: Logged, scan continues with accessible directories
- **Invalid state data**: Project included with error flag, marked for user attention
- **Missing cache**: Triggers fresh scan automatically
- **Corrupted cache**: Falls back to fresh scan
- **Invalid configuration**: Caught at engine creation with specific validation errors

## Performance

- **Initial scan**: <2 seconds for typical workspace (10-20 projects)
- **Cached discovery**: <100ms (binary deserialization + validation)
- **Manual refresh**: Use `--no-cache` flag or `refresh` command to update cache
- **Graceful failure**: One corrupted project doesn't block discovery of others
- **Memory**: Bounded (lazy loading via hegel-cli library)

## Integration with hegel-cli

hegel-pm depends on hegel-cli as a library for all .hegel data access:
- `hegel::storage::FileStorage` - State data extraction
- `hegel::storage::{State, WorkflowState}` - Type definitions
- `hegel::metrics::parse_unified_metrics` - Archive-aware metrics aggregation
  - Merges archived workflows with live data
  - Uses pre-computed totals from archive files
  - Includes git metrics backfilled via `hegel analyze --fix-archives`

**Abstraction boundary**: hegel-pm never directly reads .hegel files. All data access goes through hegel-cli's library API. The underlying storage format (JSON, JSONL, archives, SQLite, etc.) is completely opaque to hegel-pm.

## Consumers

**hegel-pm-web**: Web dashboard that visualizes projects discovered by hegel-pm
- Uses `DiscoveryEngine` API for project discovery
- Provides web UI with HTTP API for project data
- See hegel-pm-web repository for details

**Future tools**: Any tool needing Hegel project discovery can depend on hegel-pm library

## License

SSPL-1.0
