# Lines of Code Report

**Last Updated**: 2025-11-02 17:31
**Tool**: [cloc](https://github.com/AlDanial/cloc) + wc

---

## Overall Summary

| Metric | Rust Code | Documentation (.md) | Total |
|--------|-----------|---------------------|-------|
| **Lines** | 1,049 | 1,621 | 2,670 |
| **Comments** | 103 | - | 103 |
| **Blank Lines** | 236 | - | 236 |
| **Total Lines** | 1,388 | 1,621 | 3,009 |
| **Files** | 11 | 7 | 18 |

**Documentation Ratio**: 1.55 lines of docs per line of code

---

## Rust Code Breakdown

```
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                            11            236            103           1049
-------------------------------------------------------------------------------
SUM:                            11            236            103           1049
-------------------------------------------------------------------------------
```

---

## Rust File Details

| File | Total Lines | Impl Lines | Test Lines | Test % | Status |
|------|-------------|------------|------------|--------|--------|
| `discovery/cache.rs` | 145 | 51 | 94 | 64.8% | ✅ |
| `discovery/config.rs` | 212 | 105 | 107 | 50.5% | ✅ |
| `discovery/discover.rs` | 200 | 52 | 148 | 74.0% | ✅ |
| `discovery/engine.rs` | 175 | 46 | 129 | 73.7% | ✅ |
| `discovery/mod.rs` | 33 | 22 | 11 | 33.3% | ✅ |
| `discovery/project.rs` | 238 | 115 | 123 | 51.7% | ✅ |
| `discovery/state.rs` | 113 | 19 | 94 | 83.2% | ✅ |
| `discovery/statistics.rs` | 67 | 30 | 37 | 55.2% | ✅ |
| `discovery/walker.rs` | 201 | 46 | 155 | 77.1% | ✅ |
| `lib.rs` | 1 | 1 | 0 | 0.0% | ✅ |
| `main.rs` | 3 | 3 | 0 | 0.0% | ✅ |

---

## Documentation Files

| File | Lines |
|------|-------|
| `.ddd/feat/project-discovery/PLAN.md` | 372 |
| `.ddd/feat/project-discovery/SPEC.md` | 333 |
| `ARCHITECTURE.md` | 273 |
| `CLAUDE.md` | 389 |
| `COVERAGE_REPORT.md` | 56 |
| `LOC_REPORT.md` | 83 |
| `VISION.md` | 115 |

---

## Documentation Quality Targets

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Docs/Code Ratio | ≥0.3 | 1.55 | ✅ Excellent |
| README exists | Yes | ❌ | Missing |
| Architecture docs | Yes | ✅ | Met |

---

## How to Update This Report

```bash
# Regenerate LOC report
./scripts/generate-loc-report.sh
```

---

*This report is auto-generated from `cloc` and `wc` output.*
*Updated automatically by pre-commit hook when source files change.*
