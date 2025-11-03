# Lines of Code Report

**Last Updated**: 2025-11-03 10:03
**Tool**: [cloc](https://github.com/AlDanial/cloc) + wc

---

## Overall Summary

| Metric | Rust Code | Documentation (.md) | Total |
|--------|-----------|---------------------|-------|
| **Lines** | 1,174 | 2,226 | 3,400 |
| **Comments** | 111 | - | 111 |
| **Blank Lines** | 262 | - | 262 |
| **Total Lines** | 1,547 | 2,226 | 3,773 |
| **Files** | 13 | 14 | 27 |

**Documentation Ratio**: 1.90 lines of docs per line of code

---

## Rust Code Breakdown

```
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                            13            262            111           1174
Markdown                         3             27              0             63
-------------------------------------------------------------------------------
SUM:                            16            289            111           1237
-------------------------------------------------------------------------------
```

---

## Rust File Details

| File | Total Lines | Impl Lines | Test Lines | Test % | Status |
|------|-------------|------------|------------|--------|--------|
| `client/mod.rs` | 92 | 92 | 0 | 0.0% | ✅ |
| `discovery/cache.rs` | 145 | 51 | 94 | 64.8% | ✅ |
| `discovery/config.rs` | 212 | 105 | 107 | 50.5% | ✅ |
| `discovery/discover.rs` | 200 | 52 | 148 | 74.0% | ✅ |
| `discovery/engine.rs` | 175 | 46 | 129 | 73.7% | ✅ |
| `discovery/mod.rs` | 33 | 22 | 11 | 33.3% | ✅ |
| `discovery/project.rs` | 236 | 113 | 123 | 52.1% | ✅ |
| `discovery/state.rs` | 113 | 19 | 94 | 83.2% | ✅ |
| `discovery/statistics.rs` | 30 | 3 | 27 | 90.0% | ✅ |
| `discovery/walker.rs` | 171 | 46 | 125 | 73.1% | ✅ |
| `lib.rs` | 9 | 3 | 6 | 66.7% | ✅ |
| `main.rs` | 70 | 70 | 0 | 0.0% | ✅ |
| `test_helpers.rs` | 61 | 35 | 26 | 42.6% | ✅ |

---

## Documentation Files

| File | Lines |
|------|-------|
| `.ddd/feat/metrics-integration/PLAN.md` | 204 |
| `.ddd/feat/metrics-integration/SPEC.md` | 102 |
| `.ddd/feat/project-discovery/PLAN.md` | 372 |
| `.ddd/feat/project-discovery/SPEC.md` | 333 |
| `ARCHITECTURE.md` | 273 |
| `CLAUDE.md` | 389 |
| `CODE_MAP.md` | 21 |
| `COVERAGE_REPORT.md` | 56 |
| `LOC_REPORT.md` | 97 |
| `README.md` | 174 |
| `src/CODE_MAP.md` | 19 |
| `src/discovery/CODE_MAP.md` | 40 |
| `src/discovery/README.md` | 31 |
| `VISION.md` | 115 |

---

## Documentation Quality Targets

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Docs/Code Ratio | ≥0.3 | 1.90 | ✅ Excellent |
| README exists | Yes | ✅ | Met |
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
