# Lines of Code Report

**Last Updated**: 2025-11-07 12:03
**Tool**: [cloc](https://github.com/AlDanial/cloc) + wc

---

## Overall Summary

| Metric | Rust Code | Documentation (.md) | Total |
|--------|-----------|---------------------|-------|
| **Lines** | 5,353 | 12,004 | 17,357 |
| **Comments** | 419 | - | 419 |
| **Blank Lines** | 928 | - | 928 |
| **Total Lines** | 6,700 | 12,004 | 18,704 |
| **Files** | 37 | 43 | 80 |

**Documentation Ratio**: 2.24 lines of docs per line of code

---

## Rust Code Breakdown

```
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                            37            928            419           5353
Markdown                         7             37              0            165
-------------------------------------------------------------------------------
SUM:                            44            965            419           5518
-------------------------------------------------------------------------------
```

---

## Rust File Details

| File | Total Lines | Impl Lines | Test Lines | Test % | Status |
|------|-------------|------------|------------|--------|--------|
| `api_types.rs` | 511 | 230 | 281 | 55.0% | ⚠️ Large |
| `benchmark_mode.rs` | 387 | 238 | 149 | 38.5% | ⚠️ Large |
| `cli.rs` | 264 | 80 | 184 | 69.7% | ✅ |
| `cli/discover/all.rs` | 400 | 273 | 127 | 31.8% | ⚠️ Large |
| `cli/discover/format.rs` | 128 | 52 | 76 | 59.4% | ✅ |
| `cli/discover/list.rs` | 209 | 118 | 91 | 43.5% | ✅ |
| `cli/discover/mod.rs` | 97 | 68 | 29 | 29.9% | ✅ |
| `cli/discover/show.rs` | 261 | 168 | 93 | 35.6% | ✅ |
| `cli/hegel.rs` | 156 | 95 | 61 | 39.1% | ✅ |
| `client/components/all_projects_view.rs` | 132 | 132 | 0 | 0.0% | ✅ |
| `client/components/mod.rs` | 7 | 7 | 0 | 0.0% | ✅ |
| `client/components/sidebar.rs` | 113 | 113 | 0 | 0.0% | ✅ |
| `client/components/workflow_detail_view.rs` | 327 | 327 | 0 | 0.0% | ⚠️ Large |
| `client/mod.rs` | 37 | 37 | 0 | 0.0% | ✅ |
| `client/types.rs` | 173 | 173 | 0 | 0.0% | ✅ |
| `data_layer/cache.rs` | 140 | 60 | 80 | 57.1% | ✅ |
| `data_layer/messages.rs` | 65 | 42 | 23 | 35.4% | ✅ |
| `data_layer/mod.rs` | 7 | 7 | 0 | 0.0% | ✅ |
| `data_layer/worker.rs` | 615 | 347 | 268 | 43.6% | ⚠️ Large |
| `discovery_mode.rs` | 21 | 21 | 0 | 0.0% | ✅ |
| `discovery/api_types.rs` | 46 | 46 | 0 | 0.0% | ✅ |
| `discovery/cache.rs` | 621 | 233 | 388 | 62.5% | ⚠️ Large |
| `discovery/config.rs` | 234 | 113 | 121 | 51.7% | ✅ |
| `discovery/discover.rs` | 200 | 52 | 148 | 74.0% | ✅ |
| `discovery/engine.rs` | 211 | 77 | 134 | 63.5% | ✅ |
| `discovery/mod.rs` | 35 | 24 | 11 | 31.4% | ✅ |
| `discovery/project.rs` | 240 | 117 | 123 | 51.2% | ✅ |
| `discovery/state.rs` | 113 | 19 | 94 | 83.2% | ✅ |
| `discovery/statistics.rs` | 30 | 3 | 27 | 90.0% | ✅ |
| `discovery/walker.rs` | 172 | 47 | 125 | 72.7% | ✅ |
| `http/axum_backend.rs` | 248 | 248 | 0 | 0.0% | ⚠️ Large |
| `http/mod.rs` | 83 | 57 | 26 | 31.3% | ✅ |
| `http/warp_backend.rs` | 201 | 201 | 0 | 0.0% | ⚠️ Large |
| `lib.rs` | 21 | 15 | 6 | 28.6% | ✅ |
| `main.rs` | 85 | 85 | 0 | 0.0% | ✅ |
| `server_mode.rs` | 49 | 49 | 0 | 0.0% | ✅ |
| `test_helpers.rs` | 61 | 35 | 26 | 42.6% | ✅ |

**⚠️ Warning:** 8 file(s) over 200 impl lines - consider splitting for maintainability

---

## Documentation Files

| File | Lines |
|------|-------|
| `.ddd/feat/benchmark/PLAN.md` | 324 |
| `.ddd/feat/benchmark/SPEC.md` | 418 |
| `.ddd/feat/binary-cache/PLAN.md` | 320 |
| `.ddd/feat/binary-cache/SPEC.md` | 322 |
| `.ddd/feat/cli-discovery/PLAN.md` | 391 |
| `.ddd/feat/cli-discovery/SPEC.md` | 557 |
| `.ddd/feat/metrics-integration/PLAN.md` | 204 |
| `.ddd/feat/metrics-integration/SPEC.md` | 102 |
| `.ddd/feat/project-discovery/PLAN.md` | 372 |
| `.ddd/feat/project-discovery/SPEC.md` | 333 |
| `.ddd/feat/swappable_backend/PLAN.md` | 292 |
| `.ddd/feat/swappable_backend/SPEC.md` | 516 |
| `.ddd/feat/swappable-frontends/PLAN.md` | 286 |
| `.ddd/feat/swappable-frontends/SPEC.md` | 326 |
| `.ddd/feat/ui-v1/PLAN_REFINED_STEPS_7-10.md` | 498 |
| `.ddd/feat/ui-v1/PLAN.md` | 482 |
| `.ddd/feat/ui-v1/SPEC.md` | 346 |
| `ARCHITECTURE.md` | 403 |
| `CLAUDE.md` | 414 |
| `COVERAGE_REPORT.md` | 56 |
| `frontends/ADDING_FRONTENDS.md` | 476 |
| `frontends/alpine/README.md` | 224 |
| `frontends/README.md` | 52 |
| `learnings/.ddd/0_sycamore_foundations_assessment.md` | 335 |
| `learnings/.ddd/1_open_questions.md` | 294 |
| `learnings/LEARNING_SYCAMORE_COMPONENTS.md` | 733 |
| `learnings/LEARNING_SYCAMORE_FOUNDATIONS.md` | 572 |
| `learnings/LEARNING_SYCAMORE_PRACTICES.md` | 530 |
| `LOC_REPORT.md` | 152 |
| `README.md` | 297 |
| `RESEARCH_PLAN.md` | 189 |
| `scripts/README.md` | 101 |
| `src/cli/discover/README.md` | 29 |
| `src/cli/README.md` | 23 |
| `src/client/README.md` | 25 |
| `src/data_layer/README.md` | 20 |
| `src/discovery/README.md` | 53 |
| `src/http/README.md` | 19 |
| `src/README.md` | 33 |
| `static/README.md` | 224 |
| `TESTING.md` | 510 |
| `tests/README.md` | 36 |
| `VISION.md` | 115 |

---

## Documentation Quality Targets

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Docs/Code Ratio | ≥0.3 | 2.24 | ✅ Excellent |
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
