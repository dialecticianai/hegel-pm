# HTTP Backend Benchmark Implementation Plan

High-level plan for adding benchmark instrumentation to measure HTTP endpoint performance with TDD discipline.

---

## Overview

**Goal**: Add `--run-benchmarks` CLI flag that starts the server, makes HTTP requests to all endpoints, measures response times, and reports per-endpoint and per-project timing statistics.

**Scope**: Create new benchmark module, extend CLI with benchmark flags, implement HTTP benchmarking with reqwest client, output results as table or JSON.

**Priorities**:
1. Core benchmark execution - measure endpoints and report results
2. Per-project granularity - individual timing for each discovered project
3. Clean output formatting - human-readable table and JSON support
4. Backend detection - report which backend binary was compiled with

**Methodology**: TDD where it drives development. Write tests for CLI parsing and benchmark result structures first. Implementation-first for HTTP client code (straightforward reqwest usage). Focus on working functionality, skip edge cases.

---

## Step 1: CLI Flag Extension

### Goal
Add benchmark flags to CLI argument parser, validate argument combinations.

### Step 1.a: Write Tests

Describe CLI parsing test strategy:
- Test benchmark flag parsing succeeds
- Test benchmark iterations flag with custom value
- Test benchmark JSON flag parsing
- Test iteration count validation rejects zero
- Test benchmark flags work independently of existing commands
- Test invalid combinations (e.g., benchmarks with discover subcommand) if needed

### Step 1.b: Implement

Extend CLI argument structure:
- Add `run_benchmarks` boolean flag to Args struct
- Add `benchmark_iterations` with default value of one hundred
- Add `benchmark_json` boolean flag for JSON output
- Configure flag requirements (iterations and json require run_benchmarks)
- Add validation for iteration count greater than zero
- Update CLI help text to document new flags

### Success Criteria

- CLI accepts benchmark flags without errors
- Default iteration count is one hundred
- Zero iterations rejected with clear error
- Help text shows new benchmark options
- Existing CLI tests still pass
- New CLI tests verify benchmark flag parsing

---

## Step 2: Benchmark Module Foundation

### Goal
Create benchmark module with data structures for results, establish testing patterns.

### Step 2.a: Write Tests

Describe benchmark types test strategy:
- Test EndpointBenchmark struct creation
- Test ProjectBenchmark struct creation
- Test BenchmarkResults aggregation structure
- Test JSON serialization of benchmark results matches expected schema
- Validate field types and names match spec

### Step 2.b: Implement

Create new benchmark module:
- Create benchmark_mode.rs file in src directory
- Define EndpointBenchmark struct with path, average milliseconds, iterations
- Define ProjectBenchmark struct with project name, average milliseconds, iterations
- Define BenchmarkResults struct with backend name and all endpoint results
- Derive serde Serialize for JSON output support
- Add module export to lib.rs gated on native compilation
- Create stub run function that takes benchmark config

### Success Criteria

- Benchmark module compiles cleanly
- Structs serialize to JSON correctly
- Module exported from lib.rs
- Tests validate struct creation and serialization
- No compilation warnings

---

## Step 3: Server Lifecycle Management

### Goal
Implement server startup in background task and readiness polling.

### Step 3.a: Write Tests

Describe server lifecycle test strategy:
- Test server readiness check succeeds when server responds
- Test readiness check retries on connection refused
- Test readiness check times out after configured duration
- Mock or stub server responses for testing
- Validate timeout behavior without waiting full duration

### Step 3.b: Implement

Add server startup and readiness logic:
- Create function to spawn server in background tokio task
- Implement readiness check that polls health endpoint
- Add retry loop with small delay between attempts
- Implement timeout logic using tokio time utilities
- Return error if server never becomes ready
- Handle connection refused during startup gracefully
- Add reqwest dependency to Cargo.toml

### Success Criteria

- Server starts in background without blocking
- Readiness check completes when server responds
- Connection refused handled gracefully with retries
- Timeout prevents infinite waiting
- Tests validate retry and timeout behavior
- Integration test confirms server becomes ready

---

## Step 4: Endpoint Benchmarking Logic

### Goal
Implement HTTP request timing for individual endpoints with warmup.

### Step 4.a: Write Tests

Describe endpoint benchmarking test strategy:
- Test warmup request executes before timing
- Test multiple iterations accumulate timing correctly
- Test average calculation is accurate
- Test successful responses required for valid benchmark
- Mock HTTP server for controlled testing
- Validate timing measurement granularity

### Step 4.b: Implement

Create endpoint benchmarking function:
- Implement warmup request that discards timing
- Create iteration loop for configured count
- Use Instant for high-resolution timing measurement
- Make HTTP GET requests with reqwest
- Accumulate elapsed times across iterations
- Compute average response time in milliseconds
- Return EndpointBenchmark with results
- Handle HTTP errors by failing benchmark
- Add timeout per request

### Success Criteria

- Warmup request executes before timing loop
- Average calculation is mathematically correct
- Timing granularity sufficient for millisecond precision
- HTTP failures cause benchmark to fail
- Tests validate timing and averaging logic
- Integration test with real HTTP confirms timing works

---

## Step 5: Per-Project Benchmarking

### Goal
Benchmark project metrics endpoint for each discovered project individually.

### Step 5.a: Write Tests

Describe per-project benchmarking strategy:
- Test project discovery returns project list
- Test benchmark executed for each project
- Test project name included in results
- Validate all projects benchmarked when multiple exist
- Test error handling when project not found

### Step 5.b: Implement

Add per-project benchmark iteration:
- Get discovered projects from engine
- Extract project names from discovery results
- Iterate over each project name
- Construct metrics endpoint URL for each project
- Call endpoint benchmarking function for each
- Collect ProjectBenchmark results into vector
- Handle errors for individual projects gracefully
- Log progress during benchmarking

### Success Criteria

- Each discovered project gets individual benchmark
- Project names correctly extracted
- Results include one ProjectBenchmark per project
- Multiple projects benchmarked sequentially
- Tests validate iteration and result collection
- Integration test confirms per-project timing

---

## Step 6: Backend Detection

### Goal
Detect which HTTP backend the binary was compiled with using feature flags.

### Step 6.a: Write Tests

Describe backend detection test strategy:
- Test warp backend detected when warp feature enabled
- Test axum backend detected when axum feature enabled
- Validate compile-time detection using cfg attributes
- No runtime detection needed

### Step 6.b: Implement

Add backend detection logic:
- Use cfg feature attributes to detect backend
- Define constant for warp backend name
- Define constant for axum backend name
- Return appropriate backend name string
- Include backend name in BenchmarkResults
- Document that detection is compile-time only

### Success Criteria

- Warp backend correctly identified
- Axum backend correctly identified
- Backend name included in results
- Detection works at compile time
- Tests validate correct backend reported
- No runtime overhead for detection

---

## Step 7: Output Formatting

### Goal
Format benchmark results as human-readable table or JSON based on flag.

### Step 7.a: Write Tests

Describe output formatting test strategy:
- Test table format produces readable output
- Test JSON format produces valid parseable JSON
- Test JSON schema matches BenchmarkResults structure
- Validate table alignment and spacing
- Test both formats with multiple projects

### Step 7.b: Implement

Create output formatting functions:
- Implement table formatter that prints to stdout
- Calculate column widths for alignment
- Format endpoint benchmarks in table rows
- Format per-project benchmarks in separate section
- Implement JSON formatter using serde_json
- Add function to choose formatter based on flag
- Handle empty project list gracefully
- Add headers and labels for readability

### Success Criteria

- Table output is aligned and readable
- JSON output is valid and parseable
- JSON schema matches specification
- Both formats include all benchmark data
- Tests validate formatting correctness
- Output can be piped to jq for JSON

---

## Step 8: Main Integration

### Goal
Wire benchmark mode into main entry point, dispatch based on CLI flags.

### Step 8.a: Write Tests

Describe main integration test strategy:
- Test benchmark mode selected when flag present
- Test normal server mode when flag absent
- Validate argument passing to benchmark runner
- Integration test runs full benchmark end-to-end
- No tests needed for main dispatch (thin layer)

### Step 8.b: Implement

Update main entry point:
- Check for run_benchmarks flag in args
- If true, call benchmark_mode run function
- Pass iterations and JSON flag to benchmark runner
- Otherwise proceed to normal server or discover mode
- Ensure clean process exit after benchmarks complete
- Propagate errors from benchmark runner
- Add appropriate logging for benchmark mode

### Success Criteria

- Benchmark mode triggered by flag
- Normal modes still work without flag
- Arguments passed correctly to benchmark runner
- Process exits cleanly after benchmarks
- Errors propagate with clear messages
- Integration test confirms end-to-end flow
- Both warp and axum backends can be benchmarked

---

## Out of Scope

Explicitly not implementing in this plan:
- Percentile metrics (only average)
- Min/max timing (only average)
- Cold vs warm cache distinction (simple warmup)
- Concurrent request benchmarking (sequential only)
- Benchmark result history or comparison
- Statistical analysis (standard deviation, confidence intervals)
- Custom endpoint selection (all endpoints always benchmarked)
- Response payload validation (assumes correct API)
