# Lines of Code Report

**Last Updated**: 2025-11-04 21:18
**Tool**: [cloc](https://github.com/AlDanial/cloc) + wc

---

## Overall Summary

| Metric | Rust Code | Documentation (.md) | Total |
|--------|-----------|---------------------|-------|
| **Lines** | 3,961 | 8,056 | 12,017 |
| **Comments** | 269 | - | 269 |
| **Blank Lines** | 641 | - | 641 |
| **Total Lines** | 4,871 | 8,056 | 12,927 |
| **Files** | 31 | 30 | 61 |

**Documentation Ratio**: 2.03 lines of docs per line of code

---

## Rust Code Breakdown

```
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                            31            641            269           3961
Markdown                         6             28              0            132
-------------------------------------------------------------------------------
SUM:                            37            669            269           4093
-------------------------------------------------------------------------------
```

---

## Rust File Details

| File | Total Lines | Impl Lines | Test Lines | Test % | Status |
|------|-------------|------------|------------|--------|--------|
| `api_types.rs` | 509 | 230 | 279 | 54.8% | ⚠️ Large |
| `cli.rs` | 211 | 69 | 142 | 67.3% | ✅ |
| `cli/discover/all.rs` | 401 | 274 | 127 | 31.7% | ⚠️ Large |
| `cli/discover/format.rs` | 128 | 52 | 76 | 59.4% | ✅ |
| `cli/discover/list.rs` | 210 | 119 | 91 | 43.3% | ✅ |
| `cli/discover/mod.rs` | 97 | 68 | 29 | 29.9% | ✅ |
| `cli/discover/show.rs` | 262 | 169 | 93 | 35.5% | ✅ |
| `cli/hegel.rs` | 157 | 96 | 61 | 38.9% | ✅ |
| `client/components/all_projects_view.rs` | 130 | 130 | 0 | 0.0% | ✅ |
| `client/components/metrics_view.rs` | 174 | 174 | 0 | 0.0% | ✅ |
| `client/components/mod.rs` | 9 | 9 | 0 | 0.0% | ✅ |
| `client/components/sidebar.rs` | 105 | 105 | 0 | 0.0% | ✅ |
| `client/components/workflow_detail_view.rs` | 395 | 395 | 0 | 0.0% | ⚠️ Large |
| `client/mod.rs` | 40 | 40 | 0 | 0.0% | ✅ |
| `client/types.rs` | 161 | 161 | 0 | 0.0% | ✅ |
| `discovery_mode.rs` | 21 | 21 | 0 | 0.0% | ✅ |
| `discovery/api_types.rs` | 46 | 46 | 0 | 0.0% | ✅ |
| `discovery/cache_manager.rs` | 69 | 69 | 0 | 0.0% | ✅ |
| `discovery/cache.rs` | 145 | 51 | 94 | 64.8% | ✅ |
| `discovery/config.rs` | 212 | 105 | 107 | 50.5% | ✅ |
| `discovery/discover.rs` | 200 | 52 | 148 | 74.0% | ✅ |
| `discovery/engine.rs` | 183 | 54 | 129 | 70.5% | ✅ |
| `discovery/mod.rs` | 37 | 26 | 11 | 29.7% | ✅ |
| `discovery/project.rs` | 239 | 116 | 123 | 51.5% | ✅ |
| `discovery/state.rs` | 113 | 19 | 94 | 83.2% | ✅ |
| `discovery/statistics.rs` | 30 | 3 | 27 | 90.0% | ✅ |
| `discovery/walker.rs` | 172 | 47 | 125 | 72.7% | ✅ |
| `lib.rs` | 12 | 6 | 6 | 50.0% | ✅ |
| `main.rs` | 57 | 57 | 0 | 0.0% | ✅ |
| `server_mode.rs` | 285 | 285 | 0 | 0.0% | ⚠️ Large |
| `test_helpers.rs` | 61 | 35 | 26 | 42.6% | ✅ |

**⚠️ Warning:** 4 file(s) over 200 impl lines - consider splitting for maintainability

---

## Documentation Files

| File | Lines |
|------|-------|
| `.ddd/feat/cli-discovery/PLAN.md` | 391 |
| `.ddd/feat/cli-discovery/SPEC.md` | 557 |
| `.ddd/feat/metrics-integration/PLAN.md` | 204 |
| `.ddd/feat/metrics-integration/SPEC.md` | 102 |
| `.ddd/feat/project-discovery/PLAN.md` | 372 |
| `.ddd/feat/project-discovery/SPEC.md` | 333 |
| `.ddd/feat/ui-v1/HANDOFF.md` | 137 |
| `.ddd/feat/ui-v1/PLAN_REFINED_STEPS_7-10.md` | 498 |
| `.ddd/feat/ui-v1/PLAN.md` | 482 |
| `.ddd/feat/ui-v1/SPEC.md` | 346 |
| `ARCHITECTURE.md` | 319 |
| `CLAUDE.md` | 414 |
| `COVERAGE_REPORT.md` | 56 |
| `learnings/.ddd/0_sycamore_foundations_assessment.md` | 335 |
| `learnings/.ddd/1_open_questions.md` | 294 |
| `learnings/LEARNING_SYCAMORE_COMPONENTS.md` | 733 |
| `learnings/LEARNING_SYCAMORE_FOUNDATIONS.md` | 572 |
| `learnings/LEARNING_SYCAMORE_PRACTICES.md` | 530 |
| `LOC_REPORT.md` | 132 |
| `README.md` | 239 |
| `RESEARCH_PLAN.md` | 189 |
| `src/cli/CODE_MAP.md` | 23 |
| `src/cli/discover/README.md` | 29 |
| `src/client/CODE_MAP.md` | 26 |
| `src/CODE_MAP.md` | 25 |
| `src/discovery/CODE_MAP.md` | 26 |
| `src/discovery/README.md` | 31 |
| `TESTING.md` | 510 |
| `tests/README.md` | 36 |
| `VISION.md` | 115 |

---

## Documentation Quality Targets

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Docs/Code Ratio | ≥0.3 | 2.03 | ✅ Excellent |
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
