# HTTP Backend Benchmark Specification

Add benchmarking instrumentation to measure HTTP endpoint performance across discovered projects, enabling backend performance comparison.

---

## Overview

**What it does:** Provides a CLI tool (`hegel-pm --run-benchmarks`) that starts the server, makes HTTP requests to all endpoints, measures response times, and reports per-endpoint and per-project timing statistics.

**Key principles:**
- Measure full HTTP round-trip latency (server startup → HTTP request → response)
- Per-project granularity for `/api/projects/:name/metrics` endpoint
- Backend-agnostic implementation (works with whichever backend binary was compiled with)
- Simple warmup strategy (no distinction between cold/warm cache)
- Human-readable output by default, machine-readable JSON with flag

**Scope:** Add `--run-benchmarks` CLI flag that:
1. Starts server in background
2. Waits for server ready
3. Makes HTTP requests via `reqwest` to all three endpoints
4. Measures and reports timing for each endpoint
5. For project-specific endpoint, reports timing per discovered project
6. Outputs results as table or JSON

**Integration context:**
- Integrates with existing CLI structure (`src/cli.rs::Args`)
- Uses existing server startup logic (`src/server_mode.rs`)
- Measures endpoints exposed by `HttpBackend` implementations
- Works with whichever backend the binary was compiled with (warp or axum)

---

## Data Model

### MODIFIED: Existing Types

**Location:** `src/cli.rs::Command`
```rust
pub enum Command {
    Discover { ... },
    Hegel { ... },

    // NEW: Benchmark command
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
```

### NEW: Benchmark Module Types

**Location:** `src/benchmark_mode.rs` (new file)

```rust
/// Benchmark results for a single endpoint
pub struct EndpointBenchmark {
    /// Endpoint path (e.g., "/api/projects")
    pub path: String,
    /// Average response time in milliseconds
    pub avg_ms: f64,
    /// Number of iterations
    pub iterations: usize,
}

/// Benchmark results for a project-specific endpoint
pub struct ProjectBenchmark {
    /// Project name
    pub project_name: String,
    /// Average response time in milliseconds
    pub avg_ms: f64,
    /// Number of iterations
    pub iterations: usize,
}

/// Complete benchmark results
pub struct BenchmarkResults {
    /// Backend name (warp or axum)
    pub backend: String,
    /// Endpoint: /api/projects
    pub projects_list: EndpointBenchmark,
    /// Endpoint: /api/all-projects
    pub all_projects: EndpointBenchmark,
    /// Endpoint: /api/projects/:name/metrics (one per discovered project)
    pub project_metrics: Vec<ProjectBenchmark>,
}
```

### NEW: Dependencies

**Location:** `Cargo.toml`
```toml
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
# Existing dependencies...
reqwest = "0.11"  # For HTTP client requests
```

---

## Core Operations

### Operation 1: Benchmark Execution

**Syntax:**
```bash
hegel-pm benchmark
hegel-pm benchmark --iterations 50
hegel-pm benchmark --json
hegel-pm benchmark --iterations 200 --json
```

**Parameters:**
- `benchmark`: Command to run benchmarks
- `--iterations N`: Number of iterations per endpoint (default: 100, must be > 0)
- `--json`: Output results as JSON instead of human-readable table

**Behavior:**
1. Parse CLI args, validate benchmark flags
2. Start server in background task (same as normal server mode)
3. Wait for server to be ready (poll `http://127.0.0.1:3030/api/projects` until success)
4. Discover projects to determine benchmark targets
5. Run warmup iteration for each endpoint (single request, discard timing)
6. For each endpoint, run N iterations and measure elapsed time
7. Compute averages across iterations
8. Output results (table or JSON format)
9. Shutdown server and exit

**Example (human-readable output):**
```
Backend: warp
Iterations: 100

Endpoint Benchmarks:
  /api/projects              12.5ms avg
  /api/all-projects          45.3ms avg

Per-Project Metrics:
  hegel-cli                  23.1ms avg
  hegel-pm                   18.7ms avg
  hegel-mirror               31.2ms avg
```

**Example (JSON output):**
```json
{
  "backend": "warp",
  "projects_list": {
    "path": "/api/projects",
    "avg_ms": 12.5,
    "iterations": 100
  },
  "all_projects": {
    "path": "/api/all-projects",
    "avg_ms": 45.3,
    "iterations": 100
  },
  "project_metrics": [
    {
      "project_name": "hegel-cli",
      "avg_ms": 23.1,
      "iterations": 100
    },
    {
      "project_name": "hegel-pm",
      "avg_ms": 18.7,
      "iterations": 100
    }
  ]
}
```

**Validation:**
- `benchmark_iterations` must be > 0
- Server must start successfully
- Server must respond within reasonable timeout (10 seconds for warmup)
- At least one project must be discovered for project metrics benchmarks

### Operation 2: Backend Detection

**Syntax:**
```rust
fn detect_backend() -> &'static str
```

**Behavior:**
Detect which backend the binary was compiled with using compile-time feature flags.

**Example:**
```rust
#[cfg(feature = "warp-backend")]
const BACKEND_NAME: &str = "warp";

#[cfg(feature = "axum-backend")]
const BACKEND_NAME: &str = "axum";
```

**Validation:**
- Exactly one backend feature must be enabled (enforced by existing compile_error!)

### Operation 3: Server Readiness Check

**Syntax:**
```rust
async fn wait_for_server_ready(url: &str, timeout_secs: u64) -> Result<()>
```

**Parameters:**
- `url`: Server URL to poll (e.g., "http://127.0.0.1:3030/api/projects")
- `timeout_secs`: Maximum time to wait for server to become ready

**Behavior:**
1. Make HTTP GET request to URL
2. If successful (status 200), return Ok
3. If connection refused, wait 100ms and retry
4. If timeout exceeded, return error
5. Repeat until success or timeout

**Validation:**
- timeout_secs must be > 0
- Must handle connection refused gracefully (server still starting)
- Must return error if server never becomes ready

### Operation 4: Endpoint Benchmarking

**Syntax:**
```rust
async fn benchmark_endpoint(url: &str, iterations: usize) -> EndpointBenchmark
```

**Parameters:**
- `url`: Full endpoint URL (e.g., "http://127.0.0.1:3030/api/projects")
- `iterations`: Number of timed requests to make

**Behavior:**
1. Make one warmup request (discard timing)
2. For each iteration:
   - Record start time
   - Make HTTP GET request
   - Wait for response
   - Record elapsed time
3. Compute average across all iterations
4. Return EndpointBenchmark with path, average, and iteration count

**Validation:**
- All requests must succeed (status 200)
- Response must be valid (non-empty body)
- iterations must be > 0

---

## Test Scenarios

### Simple: Single Project Benchmark

**Setup:**
- Test workspace with one discovered project
- Start in benchmark mode with default iterations (100)

**Action:**
```bash
hegel-pm --run-benchmarks
```

**Expected:**
- Server starts successfully
- Three endpoint benchmarks complete (projects list, all projects, one project metrics)
- Output shows readable table with timing for each endpoint
- Process exits cleanly with status 0

### Complex: Multiple Projects with JSON Output

**Setup:**
- Test workspace with 5 discovered projects
- Run with custom iterations and JSON output

**Action:**
```bash
hegel-pm --run-benchmarks --benchmark-iterations 50 --benchmark-json
```

**Expected:**
- Server starts and becomes ready within timeout
- Warmup requests complete for all endpoints
- 50 iterations per endpoint (projects list, all projects, 5x project metrics = 7 total benchmark sets)
- JSON output on stdout with structure matching BenchmarkResults
- Valid JSON (parseable with jq)
- Process exits cleanly

### Error: Invalid Iteration Count

**Setup:**
- Attempt to run with zero iterations

**Action:**
```bash
hegel-pm benchmark --iterations 0
```

**Expected:**
- Benchmark runs but validation could catch zero iterations
- Or zero iterations causes immediate completion with no timing
- Process behavior depends on validation implementation

### Error: Server Fails to Start

**Setup:**
- Port 3030 already in use by another process

**Action:**
```bash
hegel-pm benchmark
```

**Expected:**
- Server startup fails with port conflict error
- Readiness check times out or detects startup failure
- Clear error message indicating port conflict
- Process exits with non-zero status
- No partial benchmark results

---

## Success Criteria

**Agent-Verifiable:**

- Build succeeds: `cargo build --features server`
- All existing tests pass: `cargo test --features server`
- CLI accepts `benchmark` command: parsing succeeds
- CLI validates `--iterations` flag with default value
- Benchmark mode can be invoked: `hegel-pm benchmark` runs without panic
- Server starts in background when benchmark mode enabled
- Readiness check completes successfully (server responds)
- Endpoints benchmarked: projects list, all projects, per-project metrics
- Per-project granularity: one ProjectBenchmark per discovered project
- Output format valid: human-readable table printed to stdout
- JSON format valid: `--json` produces parseable JSON matching schema
- Backend detection works: output shows "warp" or "axum" based on compiled features
- Process exits cleanly after benchmarks complete
- Benchmark with warp backend succeeds: `cargo build --features warp-backend && ./target/debug/hegel-pm benchmark`
- Benchmark with axum backend succeeds: `cargo build --features axum-backend && ./target/debug/hegel-pm benchmark`

**Optional Human Testing:**

- Timing results seem reasonable for local loopback requests
- Table output is readable and well-formatted
- JSON output can be piped to analysis tools
- Benchmark runs complete in reasonable time (< 30 seconds for default iterations)

---

## Out of Scope

**Deferred to future work:**
- Percentile metrics (p50, p95, p99) - only average for now
- Min/max timing - only average for now
- Cold vs. warm cache distinction - simple warmup only
- Concurrent request benchmarking - sequential requests only
- Detailed timing breakdown (network vs. processing) - full round-trip only
- Benchmark result history tracking - one-off execution only
- Comparison between backends in single run - requires compiling both
- Statistical analysis (standard deviation, confidence intervals)
- Custom endpoint selection - benchmarks all three endpoints
- Custom port configuration - uses default 3030
- Benchmark result export to file - stdout only

**Explicitly not included:**
- Data layer timing instrumentation (backend comparison only)
- UI for benchmark visualization
- Automated performance regression detection
- Load testing or stress testing
- Response payload validation (assumes correct API contract)

---

## Implementation Notes

**Module structure:**
```
src/
├── benchmark_mode.rs        # NEW: Benchmark execution logic
├── cli.rs                   # MODIFIED: Add benchmark flags
├── main.rs                  # MODIFIED: Dispatch to benchmark_mode
└── server_mode.rs           # Reused as-is for server startup
```

**Entry point:**
- `main.rs` checks `args.run_benchmarks` flag
- If true, call `benchmark_mode::run()` instead of `server_mode::run()`
- Pass benchmark configuration (iterations, json output) to benchmark runner

**Server lifecycle:**
- Spawn server in background tokio task
- Poll for readiness before benchmarking
- Graceful shutdown not required (process exit cleans up)

**HTTP client:**
- Use `reqwest` blocking or async client
- Simple GET requests, no authentication
- Timeout per request (e.g., 30 seconds)
- Handle connection errors gracefully during warmup

**Timing measurement:**
- Use `std::time::Instant::now()` for high-resolution timing
- Measure from request start to response received
- Convert to milliseconds for output (f64 precision)

**Output formatting:**
- Human-readable: simple text table with alignment
- JSON: serialize BenchmarkResults with serde_json
- Write to stdout, errors to stderr
