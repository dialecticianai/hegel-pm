# scripts/

Development and automation scripts for building, testing, and maintaining hegel-pm.

---

## Structure

```
scripts/
├── test.sh                         Build and test CLI functionality
├── generate-coverage-report.sh     Generate code coverage report to COVERAGE_REPORT.md (pre-commit hook)
├── generate-loc-report.sh          Generate lines-of-code report to LOC_REPORT.md (pre-commit hook)
├── doc-audit.pl                    Audit markdown documentation for staleness and completeness
├── smoke-test-json.pl              Smoke test for JSON output functionality
│
└── oneoffs/                        One-time migration and transformation scripts
    ├── 20251104-migrate-to-tracing.pl        Migrate logging from println to tracing crate
    └── 20251105-unmigrate-cli-from-tracing.pl   Reverse tracing migration (stdout fix)
```

---

## Primary Scripts

### test.sh

Build and test hegel-pm CLI.

```bash
./scripts/test.sh    # Build + test
```

Builds with `cargo build --release` and runs `cargo test`.

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
