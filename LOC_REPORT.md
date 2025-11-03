# Lines of Code Report

**Last Updated**: 2025-11-03 11:58
**Tool**: [cloc](https://github.com/AlDanial/cloc) + wc

---

## Overall Summary

| Metric | Rust Code | Documentation (.md) | Total |
|--------|-----------|---------------------|-------|
| **Lines** | 1,287 | 5,411 | 6,698 |
| **Comments** | 117 | - | 117 |
| **Blank Lines** | 278 | - | 278 |
| **Total Lines** | 1,682 | 5,411 | 7,093 |
| **Files** | 18 | 21 | 39 |

**Documentation Ratio**: 4.20 lines of docs per line of code

---

## Rust Code Breakdown

```
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                            18            278            117           1287
Markdown                         4             20              0            112
-------------------------------------------------------------------------------
SUM:                            22            298            117           1399
-------------------------------------------------------------------------------
```

---

## Rust File Details

| File | Total Lines | Impl Lines | Test Lines | Test % | Status |
|------|-------------|------------|------------|--------|--------|
| `cli.rs` | 15 | 15 | 0 | 0.0% | ✅ |
| `client/components.rs` | 157 | 157 | 0 | 0.0% | ✅ |
| `client/mod.rs` | 23 | 23 | 0 | 0.0% | ✅ |
| `client/types.rs` | 14 | 14 | 0 | 0.0% | ✅ |
| `discovery_mode.rs` | 20 | 20 | 0 | 0.0% | ✅ |
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
| `main.rs` | 26 | 26 | 0 | 0.0% | ✅ |
| `server_mode.rs` | 42 | 42 | 0 | 0.0% | ✅ |
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
| `CLAUDE.md` | 397 |
| `COVERAGE_REPORT.md` | 56 |
| `learnings/.ddd/0_sycamore_foundations_assessment.md` | 335 |
| `learnings/.ddd/1_open_questions.md` | 294 |
| `learnings/LEARNING_SYCAMORE_COMPONENTS.md` | 733 |
| `learnings/LEARNING_SYCAMORE_FOUNDATIONS.md` | 572 |
| `learnings/LEARNING_SYCAMORE_PRACTICES.md` | 514 |
| `LOC_REPORT.md` | 106 |
| `README.md` | 174 |
| `RESEARCH_PLAN.md` | 189 |
| `src/client/CODE_MAP.md` | 36 |
| `src/CODE_MAP.md` | 22 |
| `src/discovery/CODE_MAP.md` | 43 |
| `src/discovery/README.md` | 31 |
| `TESTING.md` | 510 |
| `VISION.md` | 115 |

---

## Documentation Quality Targets

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Docs/Code Ratio | ≥0.3 | 4.20 | ✅ Excellent |
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
