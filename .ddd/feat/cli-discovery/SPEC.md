# CLI Discovery Interface Specification

Command-line interface for discovering and inspecting Hegel projects across the filesystem.

---

## Overview

**What it does:** Provides three CLI commands for discovering Hegel projects, viewing lightweight project listings, inspecting individual project metrics, and comparing aggregate statistics across all projects.

**Key principles:**
- Cache-first by default (fast responses)
- Human-readable output by default, JSON available via flag
- Lightweight list view (no metrics loading), detailed views load full statistics
- Column-sortable aggregate tables for cross-project comparison
- Direct access to discovery module (no web server dependency)

**Scope:** Three commands exposing existing discovery module functionality through CLI. Uses cached discovery results unless explicitly bypassed.

**Integration context:** Consumes `src/discovery` module APIs. Integrates with hegel-pm binary as CLI subcommands. Used by developers, coding agents, and CI/CD pipelines.

---

## Data Model

### Project Listing Entry (Lightweight)

```
hegel-cli                     ~/Code/hegel-cli           412 KB    2025-11-02 14:32
hegel-pm                      ~/Code/hegel-pm            1.2 MB    2025-11-03 09:15
my-app                        ~/Code/my-app              89 KB     2025-10-28 16:45
```

Fields displayed:
- **Name**: Project directory name
- **Path**: Project root path (relative to home or absolute)
- **Size**: `.hegel/` directory size (human-readable: KB, MB, GB)
- **Last Activity**: Most recent file modification timestamp in `.hegel/`

### Project Detail View (Single Project)

```
Project: hegel-pm
Path: /Users/emadum/Code/github.com/dialecticianai/hegel-pm
.hegel size: 1.2 MB
Last activity: 2025-11-03 09:15:23

Workflow State:
  Mode: execution
  Current node: spec
  History: kickoff → spec

Metrics:
  Total tokens: 145,823
  Total events: 89
  Active phases: 3
  [Additional unified metrics fields as available]

Status: Active
```

### Aggregate Table View (All Projects)

```
NAME          PATH                  SIZE      LAST ACTIVITY        TOKENS    EVENTS    PHASES
hegel-pm      ~/Code/hegel-pm       1.2 MB    2025-11-03 09:15    145,823   89        3
hegel-cli     ~/Code/hegel-cli      412 KB    2025-11-02 14:32    89,234    56        5
my-app        ~/Code/my-app         89 KB     2025-10-28 16:45    12,450    23        2
```

Columns:
- **NAME**: Project name
- **PATH**: Project path (abbreviated)
- **SIZE**: `.hegel/` folder size
- **LAST ACTIVITY**: Timestamp
- **TOKENS**: Total token count from metrics
- **EVENTS**: Total event count
- **PHASES**: Active phases count
- Additional columns from `UnifiedMetrics` as relevant

### JSON Output Format

All commands support `--json` flag for machine-readable output.

**List projects JSON**:
```json
{
  "projects": [
    {
      "name": "hegel-pm",
      "project_path": "/Users/emadum/Code/github.com/dialecticianai/hegel-pm",
      "hegel_dir": "/Users/emadum/Code/github.com/dialecticianai/hegel-pm/.hegel",
      "hegel_size_bytes": 1258291,
      "last_activity": "2025-11-03T09:15:23Z",
      "has_state": true,
      "has_error": false
    }
  ],
  "total_count": 1,
  "cache_used": true
}
```

**Show project JSON**:
```json
{
  "name": "hegel-pm",
  "project_path": "/Users/emadum/Code/github.com/dialecticianai/hegel-pm",
  "hegel_dir": "/Users/emadum/Code/github.com/dialecticianai/hegel-pm/.hegel",
  "hegel_size_bytes": 1258291,
  "last_activity": "2025-11-03T09:15:23Z",
  "workflow_state": {
    "mode": "execution",
    "current_node": "spec",
    "history": ["kickoff", "spec"]
  },
  "metrics": {
    "total_tokens": 145823,
    "total_events": 89,
    "active_phases": 3
  },
  "error": null
}
```

---

## Core Operations

### 1. List Projects

**Syntax:**
```
hegel-pm discover list [--json] [--no-cache]
```

**Parameters:**
- `--json`: Output as JSON instead of human-readable table (optional)
- `--no-cache`: Force fresh filesystem scan, bypass cache (optional)

**Behavior:**
- Loads projects from cache (default) or scans filesystem (`--no-cache`)
- Calculates `.hegel/` folder size for each project (disk usage, not recursive sum)
- Sorts by last activity (most recent first)
- Displays lightweight table without loading metrics
- Exits with code 0 if successful, non-zero on error

**Examples:**

Simple usage:
```bash
$ hegel-pm discover list
hegel-pm      ~/Code/hegel-pm       1.2 MB    2025-11-03 09:15
hegel-cli     ~/Code/hegel-cli      412 KB    2025-11-02 14:32
my-app        ~/Code/my-app         89 KB     2025-10-28 16:45

3 projects found
```

Force refresh:
```bash
$ hegel-pm discover list --no-cache
🔄 Scanning filesystem...
💾 Cache updated

hegel-pm      ~/Code/hegel-pm       1.2 MB    2025-11-03 09:15
hegel-cli     ~/Code/hegel-cli      412 KB    2025-11-02 14:32

2 projects found
```

JSON output:
```bash
$ hegel-pm discover list --json
{"projects":[{"name":"hegel-pm","project_path":"..."}],"total_count":1,"cache_used":true}
```

**Validation:**
- Discovery config must be valid (at least one root directory exists)
- Cache location must be writable
- If no projects found, displays "No Hegel projects found" and exits 0

**Error scenarios:**
- Invalid discovery config: Exit code 1, message describes validation failure
- Cache read error (permissions): Fall back to fresh scan, warn user
- Filesystem scan error: Exit code 1, message describes IO error

---

### 2. Show Project

**Syntax:**
```
hegel-pm discover show <project-name> [--json] [--no-cache]
```

**Parameters:**
- `<project-name>`: Project name (required, matches directory name)
- `--json`: Output as JSON instead of human-readable format (optional)
- `--no-cache`: Force fresh scan before lookup (optional)

**Behavior:**
- Loads projects from cache (or scans if `--no-cache`)
- Finds project by exact name match (case-sensitive)
- Loads full `UnifiedMetrics` for the project
- Displays detailed view including workflow state and metrics
- Exits with code 0 if found, code 1 if not found

**Examples:**

Show single project:
```bash
$ hegel-pm discover show hegel-pm
Project: hegel-pm
Path: /Users/emadum/Code/github.com/dialecticianai/hegel-pm
.hegel size: 1.2 MB
Last activity: 2025-11-03 09:15:23

Workflow State:
  Mode: execution
  Current node: spec
  History: kickoff → spec

Metrics:
  Total tokens: 145,823
  Total events: 89
  Active phases: 3

Status: Active
```

Project not found:
```bash
$ hegel-pm discover show nonexistent
Error: Project 'nonexistent' not found

Available projects:
  - hegel-pm
  - hegel-cli
  - my-app
```

JSON output:
```bash
$ hegel-pm discover show hegel-pm --json
{"name":"hegel-pm","project_path":"...","metrics":{...}}
```

**Validation:**
- Project name must be non-empty
- Project must exist in discovered projects list
- If project has corrupted state, display error field but show other data

**Error scenarios:**
- Project not found: Exit code 1, suggest available projects
- Metrics loading failure: Display warning, show partial data, exit code 0
- Discovery failure: Exit code 1, describe error

---

### 3. Show All Projects (Aggregate Table)

**Syntax:**
```
hegel-pm discover all [--sort-by <column>] [--benchmark] [--json] [--no-cache]
```

**Parameters:**
- `--sort-by <column>`: Sort by column name (optional, default: `last-activity`)
  - Valid columns: `name`, `path`, `size`, `last-activity`, `tokens`, `events`, `phases`, `load-time` (when `--benchmark` is used)
- `--benchmark`: Include load time column showing how long metrics took to load for each project (optional)
- `--json`: Output as JSON instead of table (optional)
- `--no-cache`: Force fresh scan (optional)

**Behavior:**
- Loads projects from cache (or scans if `--no-cache`)
- Loads full metrics for all projects
- When `--benchmark` is used, measures time taken to load metrics for each project
- Displays tabular view with all metrics
- When benchmarking, includes load-time column with millisecond precision
- Sorts by specified column (descending for numeric, ascending for text)
- When benchmarking, displays total load time at bottom of table
- Exits with code 0 if successful

**Examples:**

Default view (sorted by last activity):
```bash
$ hegel-pm discover all
NAME          PATH                  SIZE      LAST ACTIVITY        TOKENS    EVENTS    PHASES
hegel-pm      ~/Code/hegel-pm       1.2 MB    2025-11-03 09:15    145,823   89        3
hegel-cli     ~/Code/hegel-cli      412 KB    2025-11-02 14:32    89,234    56        5
my-app        ~/Code/my-app         89 KB     2025-10-28 16:45    12,450    23        2

3 projects found
```

Sort by tokens:
```bash
$ hegel-pm discover all --sort-by tokens
NAME          PATH                  SIZE      LAST ACTIVITY        TOKENS    EVENTS    PHASES
hegel-pm      ~/Code/hegel-pm       1.2 MB    2025-11-03 09:15    145,823   89        3
hegel-cli     ~/Code/hegel-cli      412 KB    2025-11-02 14:32    89,234    56        5
my-app        ~/Code/my-app         89 KB     2025-10-28 16:45    12,450    23        2

3 projects found (sorted by tokens)
```

Benchmark mode:
```bash
$ hegel-pm discover all --benchmark
NAME          PATH                  SIZE      LAST ACTIVITY        TOKENS    EVENTS    PHASES    LOAD TIME
hegel-pm      ~/Code/hegel-pm       1.2 MB    2025-11-03 09:15    145,823   89        3         142ms
hegel-cli     ~/Code/hegel-cli      412 KB    2025-11-02 14:32    89,234    56        5         98ms
my-app        ~/Code/my-app         89 KB     2025-10-28 16:45    12,450    23        2         23ms

3 projects found
Total load time: 263ms
```

Benchmark with sorting by load time:
```bash
$ hegel-pm discover all --benchmark --sort-by load-time
NAME          PATH                  SIZE      LAST ACTIVITY        TOKENS    EVENTS    PHASES    LOAD TIME
hegel-pm      ~/Code/hegel-pm       1.2 MB    2025-11-03 09:15    145,823   89        3         142ms
hegel-cli     ~/Code/hegel-cli      412 KB    2025-11-02 14:32    89,234    56        5         98ms
my-app        ~/Code/my-app         89 KB     2025-10-28 16:45    12,450    23        2         23ms

3 projects found (sorted by load-time)
Total load time: 263ms
```

JSON output:
```bash
$ hegel-pm discover all --json
{"projects":[...],"total_count":3,"sorted_by":"last-activity","cache_used":true}
```

JSON with benchmark data:
```bash
$ hegel-pm discover all --benchmark --json
{"projects":[{"name":"hegel-pm","load_time_ms":142,...}],"total_count":3,"sorted_by":"last-activity","total_load_time_ms":263,"cache_used":true}
```

**Validation:**
- `--sort-by` value must be a valid column name
- All projects must load successfully (partial failures warned but don't fail command)

**Error scenarios:**
- Invalid sort column: Exit code 1, list valid columns
- Metrics loading failure for project: Display "N/A" for that project's metrics, warn user
- No projects found: Display "No Hegel projects found", exit code 0

---

## Test Scenarios

### Simple Scenarios

**List with no projects:**
```bash
$ hegel-pm discover list
No Hegel projects found
```

**Show single project with minimal state:**
```bash
$ hegel-pm discover show minimal-project
Project: minimal-project
Path: /tmp/minimal-project
.hegel size: 4 KB
Last activity: 2025-11-03 10:00:00

Workflow State: None

Metrics: No metrics available

Status: Inactive
```

**All projects with single result:**
```bash
$ hegel-pm discover all
NAME      PATH          SIZE    LAST ACTIVITY        TOKENS    EVENTS    PHASES
my-app    ~/Code/app    89 KB   2025-11-03 10:00    0         0         0

1 project found
```

---

### Complex Scenarios

**List with many projects, cache hit:**
```bash
$ hegel-pm discover list
✅ Loaded 47 projects from cache

project-alpha      ~/Code/project-alpha       2.3 MB    2025-11-03 09:15
project-beta       ~/Code/project-beta        1.8 MB    2025-11-02 14:32
[...45 more projects...]

47 projects found
```

**Show project with full metrics:**
```bash
$ hegel-pm discover show complex-project
Project: complex-project
Path: /Users/dev/complex-project
.hegel size: 15.7 MB
Last activity: 2025-11-03 09:45:12

Workflow State:
  Mode: execution
  Current node: code
  History: kickoff → spec → plan → code

Metrics:
  Total tokens: 2,458,932
  Total events: 1,247
  Active phases: 8
  Files modified: 127
  Tools used: 45
  [Additional metrics...]

Status: Active
```

**All projects with custom sort:**
```bash
$ hegel-pm discover all --sort-by size
NAME          PATH                  SIZE      LAST ACTIVITY        TOKENS    EVENTS    PHASES
large-proj    ~/Code/large          45 MB     2025-11-01 08:00    3,500,000 2,100    12
medium-proj   ~/Code/medium         12 MB     2025-11-02 14:32    890,000   678      7
small-proj    ~/Code/small          245 KB    2025-11-03 09:15    45,000    34       2

3 projects found (sorted by size)
```

---

### Error Scenarios

**Invalid discovery config:**
```bash
$ hegel-pm discover list
Error: Discovery configuration invalid
Root directory does not exist: /nonexistent/path

Please check your hegel-pm configuration.
```

**Project name with corrupted state:**
```bash
$ hegel-pm discover show corrupted-proj
Project: corrupted-proj
Path: /Users/dev/corrupted-proj
.hegel size: 892 KB
Last activity: 2025-10-15 11:23:45

Workflow State: Error loading state
  Error: Failed to parse state.json: unexpected character at line 3

Metrics:
  Total tokens: 34,567
  Total events: 89
  [...]

Status: Error (corrupted state)
```

**Invalid sort column:**
```bash
$ hegel-pm discover all --sort-by invalid
Error: Invalid sort column 'invalid'

Valid columns: name, path, size, last-activity, tokens, events, phases
```

**Metrics loading failure for one project in aggregate view:**
```bash
$ hegel-pm discover all
⚠️  Warning: Failed to load metrics for project 'broken-proj': IO error

NAME          PATH                  SIZE      LAST ACTIVITY        TOKENS    EVENTS    PHASES
good-proj     ~/Code/good           1.2 MB    2025-11-03 09:15    145,823   89        3
broken-proj   ~/Code/broken         456 KB    2025-11-02 10:00    N/A       N/A       N/A

2 projects found
```

**Cache corruption (fallback to scan):**
```bash
$ hegel-pm discover list
⚠️  Cache file corrupted, performing fresh scan...
💾 Cache rebuilt

project-a     ~/Code/project-a      1.1 MB    2025-11-03 09:00

1 project found
```

---

## Success Criteria

### Core Functionality
- [ ] `discover list` displays all projects without loading metrics
- [ ] `discover list` calculates `.hegel/` folder size accurately
- [ ] `discover list` sorts by last activity (most recent first)
- [ ] `discover show <name>` loads and displays full metrics for single project
- [ ] `discover show <name>` handles project-not-found gracefully with suggestions
- [ ] `discover all` loads metrics for all projects and displays in table
- [ ] `discover all --sort-by <col>` sorts by specified column correctly

### Cache Behavior
- [ ] All commands use cache by default
- [ ] `--no-cache` flag forces fresh filesystem scan
- [ ] Cache hit displays confirmation message in non-JSON mode
- [ ] Cache miss triggers scan and rebuild
- [ ] Corrupted cache falls back to scan with warning

### Output Formats
- [ ] Default output is human-readable and well-formatted
- [ ] `--json` flag produces valid JSON for all commands
- [ ] JSON output includes `cache_used` boolean field
- [ ] Human-readable sizes use appropriate units (KB, MB, GB)
- [ ] Timestamps are displayed in readable format (ISO 8601 in JSON)

### Error Handling
- [ ] Invalid discovery config returns exit code 1 with clear message
- [ ] Project not found returns exit code 1 with suggestions
- [ ] Invalid `--sort-by` value returns exit code 1 with valid options
- [ ] Metrics loading failure for individual project warns but continues
- [ ] Corrupted state files display error field but show available data
- [ ] Empty result sets (no projects) return exit code 0

### Performance
- [ ] List command completes in <500ms with cache hit (100 projects)
- [ ] Show command completes in <200ms with cache hit
- [ ] All command completes in <2s for 50 projects (metrics loading)
- [ ] Filesystem scan with exclusions completes reasonably (<10s for typical workspace)

### Integration
- [ ] Commands integrate with existing `DiscoveryEngine` API
- [ ] Discovery config uses same defaults as server mode
- [ ] JSON output schema matches API response types where applicable
- [ ] No duplicate discovery logic (reuses `src/discovery` module)

### Edge Cases
- [ ] Zero projects found displays appropriate message
- [ ] Projects with no workflow state display "None" gracefully
- [ ] Projects with no metrics display "No metrics available"
- [ ] Very large `.hegel/` sizes display correctly (GB, TB)
- [ ] Long project paths are truncated appropriately in table view
- [ ] Special characters in project names handled correctly
