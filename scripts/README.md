# scripts/

Development and automation scripts for building, testing, and maintaining hegel-pm.

---

## Structure

```
scripts/
├── test.sh                         Build and test without starting server (supports FRONTEND env var)
├── restart-server.sh               Stop, rebuild, and restart server with fresh build (supports FRONTEND env var)
├── generate-coverage-report.sh     Generate code coverage report to COVERAGE_REPORT.md (pre-commit hook)
├── generate-loc-report.sh          Generate lines-of-code report to LOC_REPORT.md (pre-commit hook)
├── doc-audit.pl                    Audit markdown documentation for staleness and completeness
│
└── oneoffs/                        One-time migration and transformation scripts
    ├── 20251104-migrate-to-tracing.pl        Migrate logging from println to tracing crate
    └── 20251105-unmigrate-cli-from-tracing.pl   Reverse tracing migration (stdout fix)
```

---

## Primary Scripts

### test.sh

Build and test hegel-pm without starting the server.

```bash
./scripts/test.sh                      # Build + test (default: Sycamore frontend)
FRONTEND=alpine ./scripts/test.sh      # Build with Alpine.js frontend
./scripts/test.sh --exclude frontend   # Backend only (skip frontend build)
./scripts/test.sh --exclude backend    # Frontend only (skip cargo)
```

Builds frontend (Trunk or copy files), builds backend, runs tests.

### restart-server.sh

Stop existing server, rebuild, and restart with fresh build.

```bash
./scripts/restart-server.sh                      # Backend only (no frontend rebuild)
./scripts/restart-server.sh --frontend           # Backend + frontend (Sycamore)
FRONTEND=alpine ./scripts/restart-server.sh --frontend  # Backend + Alpine.js frontend
```

Logs to `logs/server-YYYYMMDD-HHMMSS.log` for debugging.

---

## Pre-Commit Hook Scripts

These scripts run automatically via git pre-commit hooks:

- **generate-coverage-report.sh**: Runs `cargo tarpaulin` and writes COVERAGE_REPORT.md (temporarily disabled for performance)
- **generate-loc-report.sh**: Counts Rust and documentation lines, writes LOC_REPORT.md

Both auto-stage their output files if changes detected.

---

## Utilities

### doc-audit.pl

Audits markdown documentation for staleness and completeness.

```bash
./scripts/doc-audit.pl                 # Check all .md files for last-modified vs git history
```

Reports files modified but not committed, files older than N days, missing documentation.

---

## Oneoff Scripts

Migration and transformation scripts in `oneoffs/` subdirectory:

- Dated format: `YYYYMMDD-description.pl`
- Typically support `--dry-run` flag for preview
- Committed alongside the changes they produce
- Reference for understanding historical transformations

See `CLAUDE.md` oneoff script patterns section for details.

---

## Frontend Selection

Both `test.sh` and `restart-server.sh` support the `FRONTEND` environment variable:

- `sycamore` (default): Builds Rust/WASM frontend with Trunk
- `alpine`: Copies Alpine.js frontend from frontends/alpine/
- Future: `react`, `vue`, etc.

Default behavior (no env var) unchanged: builds Sycamore frontend.

See `frontends/ADDING_FRONTENDS.md` for adding new frontends and updating these scripts.
