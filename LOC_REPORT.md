# Lines of Code Report

**Last Updated**: 2025-11-05 15:51
**Tool**: [cloc](https://github.com/AlDanial/cloc) + wc

---

## Overall Summary

| Metric | Rust Code | Documentation (.md) | Total |
|--------|-----------|---------------------|-------|
| **Lines** | 4,618 | 8,859 | 13,477 |
| **Comments** | 321 | - | 321 |
| **Blank Lines** | 774 | - | 774 |
| **Total Lines** | 5,713 | 8,859 | 14,572 |
| **Files** | 36 | 32 | 68 |

**Documentation Ratio**: 1.92 lines of docs per line of code

---

## Rust Code Breakdown

```
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                            36            774            321           4618
Markdown                         6             28              0            131
-------------------------------------------------------------------------------
SUM:                            42            802            321           4749
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
| `client/components/all_projects_view.rs` | 132 | 132 | 0 | 0.0% | ✅ |
| `client/components/mod.rs` | 7 | 7 | 0 | 0.0% | ✅ |
| `client/components/sidebar.rs` | 113 | 113 | 0 | 0.0% | ✅ |
| `client/components/workflow_detail_view.rs` | 327 | 327 | 0 | 0.0% | ⚠️ Large |
| `client/mod.rs` | 37 | 37 | 0 | 0.0% | ✅ |
| `client/types.rs` | 173 | 173 | 0 | 0.0% | ✅ |
| `data_layer/cache.rs` | 140 | 60 | 80 | 57.1% | ✅ |
| `data_layer/messages.rs` | 65 | 42 | 23 | 35.4% | ✅ |
| `data_layer/mod.rs` | 7 | 7 | 0 | 0.0% | ✅ |
| `data_layer/worker.rs` | 611 | 347 | 264 | 43.2% | ⚠️ Large |
| `discovery_mode.rs` | 21 | 21 | 0 | 0.0% | ✅ |
| `discovery/api_types.rs` | 46 | 46 | 0 | 0.0% | ✅ |
| `discovery/cache.rs` | 145 | 51 | 94 | 64.8% | ✅ |
| `discovery/config.rs` | 212 | 105 | 107 | 50.5% | ✅ |
| `discovery/discover.rs` | 200 | 52 | 148 | 74.0% | ✅ |
| `discovery/engine.rs` | 183 | 54 | 129 | 70.5% | ✅ |
| `discovery/mod.rs` | 35 | 24 | 11 | 31.4% | ✅ |
| `discovery/project.rs` | 239 | 116 | 123 | 51.5% | ✅ |
| `discovery/state.rs` | 113 | 19 | 94 | 83.2% | ✅ |
| `discovery/statistics.rs` | 30 | 3 | 27 | 90.0% | ✅ |
| `discovery/walker.rs` | 172 | 47 | 125 | 72.7% | ✅ |
| `http/axum_backend.rs` | 248 | 248 | 0 | 0.0% | ⚠️ Large |
| `http/mod.rs` | 83 | 57 | 26 | 31.3% | ✅ |
| `http/warp_backend.rs` | 201 | 201 | 0 | 0.0% | ⚠️ Large |
| `lib.rs` | 18 | 12 | 6 | 33.3% | ✅ |
| `main.rs` | 70 | 70 | 0 | 0.0% | ✅ |
| `server_mode.rs` | 49 | 49 | 0 | 0.0% | ✅ |
| `test_helpers.rs` | 61 | 35 | 26 | 42.6% | ✅ |

**⚠️ Warning:** 6 file(s) over 200 impl lines - consider splitting for maintainability

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
| `.ddd/feat/swappable_backend/HANDOFF.md` | 128 |
| `.ddd/feat/swappable_backend/PLAN.md` | 292 |
| `.ddd/feat/swappable_backend/SPEC.md` | 516 |
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
| `LOC_REPORT.md` | 137 |
| `README.md` | 239 |
| `RESEARCH_PLAN.md` | 189 |
| `src/cli/CODE_MAP.md` | 23 |
| `src/cli/discover/README.md` | 29 |
| `src/client/CODE_MAP.md` | 25 |
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
| Docs/Code Ratio | ≥0.3 | 1.92 | ✅ Excellent |
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
