# hegel-pm

Project manager for Hegel projects with web UI. Auto-discovers projects, visualizes workflow states, provides unified dashboard.

## Status

**Current**: Project discovery feature complete (Steps 1-10)
**Next**: Web UI implementation, statistics extraction integration

## Architecture

### Core Components

**Discovery Engine** (`src/discovery/`)
- Filesystem-based project discovery
- Workflow state parsing via hegel-cli
- Cache persistence for fast subsequent loads
- See `src/discovery/README.md` for detailed documentation

**Dependencies**
- `hegel-cli` (path dependency) - State parsing, metrics extraction
- `sycamore` (future) - Reactive web UI framework

## Usage

### Discovery API

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
- **Cache location**: `~/.config/hegel-pm/cache.json`

Custom configuration:
```rust
let config = DiscoveryConfig::new(
    vec![PathBuf::from("/custom/path")],
    5,  // max depth
    vec!["build".to_string()],  // additional exclusions
    PathBuf::from("/tmp/cache.json"),
);
```

## Development

### Testing

```bash
cargo test          # Run all tests
cargo test discovery  # Run discovery module tests only
```

Coverage: 33.96% (will improve with UI implementation)

### Project Structure

```
hegel-pm/
├── src/
│   ├── discovery/     # Project discovery engine
│   │   ├── config.rs      # Configuration with validation
│   │   ├── walker.rs      # Filesystem traversal
│   │   ├── state.rs       # State parsing (delegates to hegel-cli)
│   │   ├── project.rs     # DiscoveredProject model
│   │   ├── statistics.rs  # ProjectStatistics (TODO: metrics extraction)
│   │   ├── cache.rs       # Cache persistence
│   │   ├── discover.rs    # Project discovery integration
│   │   ├── engine.rs      # DiscoveryEngine orchestration
│   │   └── README.md      # Module documentation
│   ├── lib.rs
│   └── main.rs
├── .ddd/
│   └── feat/
│       └── project-discovery/
│           ├── SPEC.md    # Feature specification
│           └── PLAN.md    # Implementation plan
└── README.md
```

## Future Work

1. **Statistics Extraction** - Integrate hegel-cli metrics module to parse hooks.jsonl and states.jsonl
2. **Web UI** - Sycamore-based dashboard with project cards, workflow visualizations
3. **CLI Commands** - `hegel-pm scan`, `hegel-pm serve`
4. **Live Updates** - File watching for real-time state synchronization (optional)
5. **Team Features** - Multi-user support for commercial version (data model ready)

## Error Handling

All errors include context with file paths for debugging:

- **Permission denied**: Logged, scan continues with accessible directories
- **Corrupted state.json**: Project included with error flag, marked for user attention
- **Missing cache**: Triggers fresh scan automatically
- **Invalid configuration**: Caught at engine creation with specific validation errors

## Performance

- **Initial scan**: <2 seconds for typical workspace (10-20 projects)
- **Cache load**: <10ms
- **Memory**: Bounded (lazy loading, streaming JSONL when implemented)
- **Parallel scanning**: Multiple root directories scanned independently

## Integration with hegel-cli

hegel-pm depends on hegel-cli as library:
- Imports `hegel::storage::FileStorage` for state.json parsing
- Imports `hegel::storage::State`, `WorkflowState` types
- Will import `hegel::metrics` for statistics extraction (future)

No duplication - hegel-cli owns .hegel format and parsing logic.

## License

SSPL-1.0
