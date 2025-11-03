# Lines of Code Report

**Last Updated**: 2025-11-03 14:04
**Tool**: [cloc](https://github.com/AlDanial/cloc) + wc

---

## Overall Summary

| Metric | Rust Code | Documentation (.md) | Total |
|--------|-----------|---------------------|-------|
| **Lines** | 1,650 | 5,497 | 7,147 |
| **Comments** | 154 | - | 154 |
| **Blank Lines** | 322 | - | 322 |
| **Total Lines** | 2,126 | 5,497 | 7,623 |
| **Files** | 22 | 21 | 43 |

**Documentation Ratio**: 3.33 lines of docs per line of code

---

## Rust Code Breakdown

```
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                            22            322            154           1650
Markdown                         4             22              0            126
-------------------------------------------------------------------------------
SUM:                            26            344            154           1776
-------------------------------------------------------------------------------
```

---

## Rust File Details

| File | Total Lines | Impl Lines | Test Lines | Test % | Status |
|------|-------------|------------|------------|--------|--------|
| `cli.rs` | 15 | 15 | 0 | 0.0% | ✅ |
| `client/components/metrics_view.rs` | 179 | 179 | 0 | 0.0% | ✅ |
| `client/components/mod.rs` | 5 | 5 | 0 | 0.0% | ✅ |
| `client/components/sidebar.rs` | 78 | 78 | 0 | 0.0% | ✅ |
| `client/mod.rs` | 26 | 26 | 0 | 0.0% | ✅ |
| `client/types.rs` | 86 | 86 | 0 | 0.0% | ✅ |
| `discovery_mode.rs` | 20 | 20 | 0 | 0.0% | ✅ |
| `discovery/api_types.rs` | 32 | 32 | 0 | 0.0% | ✅ |
| `discovery/cache_manager.rs` | 68 | 68 | 0 | 0.0% | ✅ |
| `discovery/cache.rs` | 145 | 51 | 94 | 64.8% | ✅ |
| `discovery/config.rs` | 212 | 105 | 107 | 50.5% | ✅ |
| `discovery/discover.rs` | 200 | 52 | 148 | 74.0% | ✅ |
| `discovery/engine.rs` | 182 | 53 | 129 | 70.9% | ✅ |
| `discovery/mod.rs` | 37 | 26 | 11 | 29.7% | ✅ |
| `discovery/project.rs` | 236 | 113 | 123 | 52.1% | ✅ |
| `discovery/state.rs` | 113 | 19 | 94 | 83.2% | ✅ |
| `discovery/statistics.rs` | 30 | 3 | 27 | 90.0% | ✅ |
| `discovery/walker.rs` | 171 | 46 | 125 | 73.1% | ✅ |
| `lib.rs` | 9 | 3 | 6 | 66.7% | ✅ |
| `main.rs` | 26 | 26 | 0 | 0.0% | ✅ |
| `server_mode.rs` | 195 | 195 | 0 | 0.0% | ✅ |
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
| `CLAUDE.md` | 423 |
| `COVERAGE_REPORT.md` | 56 |
| `learnings/.ddd/0_sycamore_foundations_assessment.md` | 335 |
| `learnings/.ddd/1_open_questions.md` | 294 |
| `learnings/LEARNING_SYCAMORE_COMPONENTS.md` | 733 |
| `learnings/LEARNING_SYCAMORE_FOUNDATIONS.md` | 572 |
| `learnings/LEARNING_SYCAMORE_PRACTICES.md` | 530 |
| `LOC_REPORT.md` | 113 |
| `README.md` | 195 |
| `RESEARCH_PLAN.md` | 189 |
| `src/client/CODE_MAP.md` | 52 |
| `src/CODE_MAP.md` | 22 |
| `src/discovery/CODE_MAP.md` | 43 |
| `src/discovery/README.md` | 31 |
| `TESTING.md` | 510 |
| `VISION.md` | 115 |

---

## Documentation Quality Targets

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Docs/Code Ratio | ≥0.3 | 3.33 | ✅ Excellent |
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
