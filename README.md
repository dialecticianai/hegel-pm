# hegel-pm

Project manager for Hegel projects with web UI. Auto-discovers projects, visualizes workflow states, provides unified dashboard.

## Status

**Current**: Sycamore WASM UI working end-to-end with live project display and metrics integration
**Next**: Enhanced UI features (visualizations, graphs, filtering)

## Architecture

### Core Components

**Discovery Engine** (`src/discovery/`)
- Filesystem-based project discovery
- Workflow state and metrics extraction via hegel-cli library
- Cache persistence for fast subsequent loads
- See `src/discovery/README.md` for detailed documentation

**Web Server** (`src/main.rs` with `server` feature)
- Warp-based HTTP server on `localhost:3030`
- Serves Sycamore WASM UI bundle
- JSON API endpoint for project discovery data

**Web UI** (`ui/` - Sycamore + WASM)
- Reactive WASM-based dashboard
- Real-time project list display
- Built with Trunk bundler

**Dependencies**
- `hegel-cli` (path dependency) - All .hegel data access via library API
- `warp` - Async web server framework
- `sycamore` - Reactive web UI framework (WASM-compiled)

## Usage

### CLI

```bash
# Start web server and open browser (default)
hegel-pm

# Run discovery scan only (prints project list)
hegel-pm --discover

# Force cache refresh
hegel-pm --discover --refresh
```

**Build from source:**
```bash
cargo build --release --bin hegel-pm --features server
./target/release/hegel-pm
```

**WASM UI development:**
```bash
cd ui && trunk serve  # Hot reload at localhost:8080
cd ui && trunk build --release  # Production build to static/
```

### Discovery API (Library Usage)

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
│   │   ├── state.rs       # State extraction (delegates to hegel-cli)
│   │   ├── project.rs     # DiscoveredProject model
│   │   ├── statistics.rs  # Type alias to hegel::metrics::UnifiedMetrics
│   │   ├── cache.rs       # Cache persistence
│   │   ├── discover.rs    # Project discovery integration
│   │   ├── engine.rs      # DiscoveryEngine orchestration
│   │   └── README.md      # Module documentation
│   ├── lib.rs
│   └── main.rs
├── ui/                # Sycamore WASM frontend
├── .ddd/
│   └── feat/
│       └── project-discovery/
│           ├── SPEC.md    # Feature specification
│           └── PLAN.md    # Implementation plan
└── README.md
```

## Future Work

1. **Enhanced UI Features**:
   - Workflow state visualizations and phase breakdowns
   - Token usage graphs and trends over time
   - Project health indicators and alerts
   - Search, filtering, and sorting
   - Per-project detail views with full metrics display
2. **CLI Commands** - `hegel-pm scan`, `hegel-pm serve`, `hegel-pm status`
3. **Live Updates** - File watching for real-time state synchronization (optional)
4. **Team Features** - Multi-user support for commercial version (data model ready)

## Error Handling

All errors include context with file paths for debugging:

- **Permission denied**: Logged, scan continues with accessible directories
- **Invalid state data**: Project included with error flag, marked for user attention
- **Missing cache**: Triggers fresh scan automatically
- **Invalid configuration**: Caught at engine creation with specific validation errors

## Performance

- **Initial scan**: <2 seconds for typical workspace (10-20 projects)
- **Cache load**: <10ms
- **Memory**: Bounded (lazy loading via hegel-cli library)
- **Parallel scanning**: Multiple root directories scanned independently

## Integration with hegel-cli

hegel-pm depends on hegel-cli as a library for all .hegel data access:
- `hegel::storage::FileStorage` - State data extraction
- `hegel::storage::{State, WorkflowState}` - Type definitions
- `hegel::metrics::UnifiedMetrics` - Complete metrics aggregation

**Abstraction boundary**: hegel-pm never directly reads .hegel files. All data access goes through hegel-cli's library API. The underlying storage format (JSON, JSONL, SQLite, etc.) is completely opaque to hegel-pm.

## License

SSPL-1.0
