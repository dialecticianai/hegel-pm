# CLAUDE.md

**hegel-pm**: Project manager for Hegel projects with web UI. Tracks multiple projects, visualizes workflow state, provides unified dashboard.

**Key Context**:
- Language: Rust (edition 2021)
- Framework: Sycamore (reactive web framework)
- Test runner: `./scripts/test.sh` (or `cargo test --features server`)
- Ecosystem role: Manages multiple Hegel projects (cli, mirror, etc.)

---

# Development Scripts

**Available scripts in `scripts/` directory:**

## Build & Test (Preferred)

```bash
./scripts/test.sh                      # Build + test everything (default)
./scripts/test.sh --exclude frontend   # Backend only (skip WASM)
./scripts/test.sh --exclude backend    # Frontend only (skip cargo)
```

**What it does:**
1. Builds frontend with `trunk build --release` (unless excluded)
2. Builds backend with `cargo build --release --features server` (unless excluded)
3. Runs `cargo test --features server` (if backend not excluded)

**When to use:**
- **Default workflow**: Quick iteration during development
- Verifying changes without starting the server
- CI/CD pipelines
- When you just need to know if tests pass

## Server Management

```bash
./scripts/restart-server.sh              # Backend only (fast)
./scripts/restart-server.sh --frontend   # Backend + frontend (full rebuild)
```

**What it does:**
1. Stops any running hegel-pm server process
2. Rebuilds backend with `cargo build --release --features server`
3. Optionally rebuilds frontend with `trunk build --release` (if `--frontend` flag)
4. Starts server with `cargo run --bin hegel-pm --features server --release`
5. Shows server logs including cache status and request timing

**When to use:**
- **When you need to view changes**: After edits requiring browser verification
- To see fresh server logs with timing information
- When server is behaving unexpectedly or UI has stale WASM
- For debugging with live log output

---

# Hegel Ecosystem Integration

**hegel-pm reads Hegel state directories** (`.hegel/`) from managed projects:
- **State format**: JSONL (newline-delimited JSON objects)
- **Key files**: `state.json`, `hooks.jsonl`, `states.jsonl`, `command_log.jsonl`
- **Workflow correlation**: `workflow_id` (ISO 8601 timestamp) links events across files
- **Discovery**: Walk parent directories like git to find `.hegel/` state

**Web UI provides**:
- Multi-project dashboard (all tracked Hegel projects)
- Workflow state visualization (current phase, history, transitions)
- Metrics aggregation (per-project and cross-project views)
- Real-time updates via file watching

---

# Testing Philosophy

**TDD discipline**: Code exists because tests drove its implementation.

**Coverage target**: ≥80% lines (enforced by pre-commit hook)

**What to test**:
- ✅ State parsing and serialization (JSONL format correctness)
- ✅ Multi-project discovery and tracking logic
- ✅ Workflow state interpretation
- ✅ Web UI component behavior (Sycamore reactive state)
- ✅ File watching and live updates

**What NOT to test**:
- ❌ Third-party library behavior (Sycamore internals, serde)
- ❌ File system primitives (trust std::fs)

**Test organization**: Co-located `#[cfg(test)]` modules in implementation files

---

# Code Organization

**Module structure** (TBD during development):
- Standard Rust conventions
- Split files when exceeding ~200 lines
- Submodules for focused concerns

**Auto-enforced** (via pre-commit hooks):
- Formatting: `rustfmt` auto-runs and stages changes
- Coverage reports: Auto-generated and committed
- LOC reports: Auto-generated and committed

---

# Git Workflow

**Commit format**: Conventional commits (`type(scope): subject`)

**Types**: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`

**Example scopes**: `ui`, `discovery`, `state`, `metrics`, `parser`

**Commit footer** (always include):
```
🤖 Generated with [Claude Code](https://claude.com/claude-code)
Co-Authored-By: Claude <noreply@anthropic.com>
```

**Commit frequency**: Wait for user to initiate commits/pushes
- Don't commit aggressively after every change (wastes tokens)
- Let work accumulate into logical units
- User will say "let's commit" or "push this" when ready
- Exception: End of session wrap-up

**Pre-commit hooks** (auto-installed):
- `rustfmt` on all staged `.rs` files (auto-stages formatted output)
- Coverage report generation (`COVERAGE_REPORT.md`)
- LOC report generation (`LOC_REPORT.md`)

---

# Critical Patterns

**Code search**: Use `hegel astq` for AST-aware code search, NOT grep/rg
- Example: `hegel astq -l rust -p 'identifier' src/`
- AST-aware: finds only actual code usage, ignores comments/strings
- Only use grep for non-code searches (logs, markdown)

**Documentation fetching**: Prefer local sources over web search
- ✅ Use `cargo doc --open` for rustdoc (local Rust API documentation)
- ✅ Read source from local dependencies in `~/.cargo/registry/src/`
- ✅ Use `w3m -dump <url>` to fetch and render web pages as clean plain text
- ✅ Optionally cache with `wget` to `.webcache/` directory for offline reference
- ❌ DO NOT use WebFetch tool for external documentation
- Note: Fetch directly with w3m renders better than cached files

**JSONL parsing**: Follow Hegel ecosystem conventions
- One JSON object per line (newline-delimited)
- Timestamps in ISO 8601 format
- `workflow_id` for event correlation

**Scripts over inline commands**: Write reusable scripts to `scripts/` directory
- Check existing scripts before writing new ones
- Reusable infrastructure compounds value

**Oneoff migration scripts**: For large-scale automated transformations (Perl preferred)
- Location: `scripts/oneoffs/YYYYMMDD-description.pl`
- Pattern: Dry-run mode (`--dry-run` flag) to preview changes before applying
- Use cases: Code migrations (logging, imports, renames), JSON fixes (via jq), bulk refactoring
- Examples: `20251104-migrate-to-tracing.pl` (regex), hegel-cli's archive fix (jq)
- Benefits: Auditable, repeatable, testable transformations vs error-prone manual edits
- Workflow: 1) Write script with dry-run, 2) Test dry-run, 3) Apply, 4) Commit script + changes
- Reference: See existing oneoff scripts for patterns (classification rules, backups, summaries)

---

# Using Hegel for Workflow Orchestration

**Hegel** orchestrates Dialectic-Driven Development through state-based workflows. Use it for structured development cycles, command guardrails, AST-aware code transformations, and metrics collection.

**Core principle:** Use when structure helps, skip when it doesn't. The user always knows best.

---

## Command Reference

All commands support `--help` for detailed options. Use `hegel <command> --help` for specifics.

**State directory override:** All commands accept `--state-dir <path>` flag or `HEGEL_STATE_DIR` env var to override default `.hegel/` location. Useful for testing, multi-project workflows, or CI/CD.

### Initialization

```bash
hegel init          # Smart detection: greenfield or retrofit workflow
hegel config list   # View all configuration
hegel config get <key>
hegel config set <key> <value>
```

**Config keys:**
- `code_map_style` - `monolithic` or `hierarchical` (default: hierarchical)
- `use_reflect_gui` - Auto-launch review GUI: `true` or `false` (default: true)

**Init workflows:**
- **Greenfield** (no code): Creates CLAUDE.md, VISION.md, ARCHITECTURE.md, initializes git
- **Retrofit** (existing code): Analyzes structure, creates CODE_MAP.md, integrates DDD patterns

### Meta-Modes & Workflows

```bash
hegel meta <learning|standard>  # Declare meta-mode (required first step)
hegel meta                      # View current meta-mode

hegel start <workflow> [node]   # Load workflow (optionally at specific node)
hegel status                    # Show current state
hegel next                      # Advance to next phase (auto-infers completion claim)
hegel restart                   # Return to SPEC phase (restart cycle, keep same workflow)
hegel repeat                    # Re-display current prompt
hegel abort                     # Abandon workflow entirely (required before starting new one)
hegel reset                     # Clear all state
```

**Meta-modes:**
- `learning` - Research ↔ Discovery loop (starts with research)
- `standard` - Discovery ↔ Execution (starts with discovery)

**Workflows:**
- `init-greenfield` - CUSTOMIZE_CLAUDE → VISION → ARCHITECTURE → GIT_INIT (new projects)
- `init-retrofit` - DETECT_EXISTING → CODE_MAP → CUSTOMIZE_CLAUDE → VISION → ARCHITECTURE → GIT_COMMIT (existing projects)
- `research` - PLAN → STUDY → ASSESS → QUESTIONS (external knowledge gathering)
- `discovery` - SPEC → PLAN → CODE → LEARNINGS → README (toy experiments)
- `execution` - Production-grade rigor with code review phase
- `minimal` - Simplified for quick iterations

**Starting at custom nodes:**
```bash
# Start at default beginning
hegel start discovery           # Starts at 'spec' node

# Start at specific node (skip earlier phases)
hegel start discovery plan      # Start directly at plan phase
hegel start execution code      # Start directly at code phase
```

**Custom start nodes are useful for:**
- Resuming interrupted workflows
- Testing specific workflow phases
- Skipping phases you've already completed manually

**What happens:**
- `hegel start` prints the first phase prompt with embedded guidance
- `hegel start <workflow> <node>` starts at specified node (validates node exists)
- `hegel next` advances and prints the next phase prompt - **follow these instructions**
- `hegel repeat` re-displays current prompt if you need to see it again
- `hegel restart` returns to SPEC phase (same workflow, fresh cycle)
- `hegel abort` abandons workflow entirely (required before starting different workflow)

**Guardrails:**
- Cannot start new workflow while one is active → run `hegel abort` first
- Invalid start node returns error with list of available nodes
- Prevents accidental loss of workflow progress

### Code Operations

```bash
hegel astq [options] [path]     # AST-based search/transform (wraps ast-grep)
```

**Critical:** Use `hegel astq --help` for pattern syntax and examples. ALWAYS prefer astq over grep/rg for code search (AST-aware, ignores comments/strings, explicit "no matches" feedback).

### Document Review

```bash
hegel reflect <file.md> [files...]      # Launch Markdown review GUI
hegel reflect <file.md> --out-dir <dir> # Custom output location
```

Reviews saved to `.ddd/<filename>.review.N` (JSONL format). Read with `cat .ddd/SPEC.review.1 | jq -r '.comment'`.

### Metrics

```bash
hegel top               # Real-time TUI dashboard (4 tabs: Overview, Phases, Events, Files)
hegel analyze           # Static summary (tokens, activity, workflow graph, per-phase metrics)
hegel hook <event>      # Process Claude Code hook events (stdin JSON)
```

Dashboard shortcuts: `q` (quit), `Tab` (switch tabs), `↑↓`/`j`/`k` (scroll), `g`/`G` (top/bottom), `r` (reload).

---

## When to Use Workflows

**Use DDD workflows for:**
- Hard problems requiring novel solutions
- Complex domains where mistakes are expensive
- Learning-dense exploration

**Skip workflows for:**
- Straightforward implementations agents handle autonomously
- Simple CRUD or routine features
- User hasn't requested structured methodology

**When in doubt:** Check `hegel status` at session start. If no active workflow and user hasn't requested structure, proceed without Hegel orchestration.

---

## Integration Patterns

### Session Start

```bash
hegel meta              # Check meta-mode
hegel status            # Check active workflow
# If workflow active and relevant, continue with `hegel next`
# If user requests structure but no workflow, run `hegel meta <mode>`
```

### During Development

```bash
hegel astq -p 'pattern' src/        # AST-aware code search (NOT grep)
hegel top                           # Monitor metrics
```

### Advancing Workflow

```bash
hegel next              # Completed current phase (infers happy-path claim)
hegel restart           # Return to SPEC phase
hegel abort             # Abandon workflow entirely
```

### Document Review

```bash
hegel reflect SPEC.md
# User reviews in GUI, submits
cat .ddd/SPEC.review.1 | jq -r '.comment'  # Read feedback
```

---

## State Files

```
.hegel/
├── state.json          # Current workflow (def, node, history, session metadata)
├── metamode.json       # Meta-mode declaration
├── config.toml         # User configuration
├── hooks.jsonl         # Claude Code events (tool usage, file mods, timestamps)
└── states.jsonl        # Workflow transitions (from/to, mode, workflow_id)
```

**JSONL format:** One JSON object per line (newline-delimited)
**Atomicity:** `state.json` uses atomic writes (write temp, rename)
**Correlation:** `workflow_id` (ISO 8601 timestamp) links hooks/states/transcripts

---

## Error Handling

| Error | Solution |
|-------|----------|
| "No workflow loaded" | `hegel start <workflow>` |
| "Cannot start workflow while one is active" | `hegel abort` then `hegel start <workflow>` |
| "Stayed at current node" (unexpected) | Check `hegel status`, verify not at terminal node, use `hegel restart` |

---

## Best Practices

**DO:**
- ✅ Check `hegel status` at session start
- ✅ Use `hegel astq` for code search (NOT grep/rg - AST-aware)
- ✅ Preview `astq` transformations before applying
- ✅ Read review files after `hegel reflect`
- ✅ Defer to `hegel <command> --help` for detailed syntax

**DON'T:**
- ❌ Start workflow if user hasn't requested structure
- ❌ Use `astq --apply` without previewing
- ❌ Ignore workflow prompts (contain phase-specific guidance)
- ❌ Reset workflow without user confirmation
- ❌ Use grep/rg for code search (use `hegel astq`)

---

## Quick Reference

```bash
# Initialization
hegel init
hegel config list|get|set <key> [<value>]

# Meta-mode (required before workflows)
hegel meta <learning|standard>
hegel meta

# Workflows
hegel start <discovery|execution|research|minimal>
hegel next|restart|abort|repeat|status|reset

# Commands (none for hegel-pm yet)

# Code
hegel astq [options] [path]     # See: hegel astq --help

# Review
hegel reflect <files...>

# Metrics
hegel top
hegel analyze
```

---

**For detailed command syntax, always use:** `hegel <command> --help`

