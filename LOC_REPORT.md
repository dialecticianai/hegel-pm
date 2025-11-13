# Lines of Code Report

**Last Updated**: 2025-11-13 00:51
**Tool**: [cloc](https://github.com/AlDanial/cloc) + wc

---

## Overall Summary

| Metric | Rust Code | Documentation (.md) | Total |
|--------|-----------|---------------------|-------|
| **Lines** | 2,717 | 11,627 | 14,344 |
| **Comments** | 246 | - | 246 |
| **Blank Lines** | 530 | - | 530 |
| **Total Lines** | 3,493 | 11,627 | 15,120 |
| **Files** | 21 | 39 | 60 |

**Documentation Ratio**: 4.28 lines of docs per line of code

---

## Rust Code Breakdown

```
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                            21            530            246           2717
Markdown                         4             22              0            116
-------------------------------------------------------------------------------
SUM:                            25            552            246           2833
-------------------------------------------------------------------------------
```

---

## Rust File Details

| File | Total Lines | Impl Lines | Test Lines | Test % | Status |
|------|-------------|------------|------------|--------|--------|
| `cli.rs` | 183 | 61 | 122 | 66.7% | ✅ |
| `cli/discover/all.rs` | 400 | 273 | 127 | 31.8% | ⚠️ Large |
| `cli/discover/format.rs` | 128 | 52 | 76 | 59.4% | ✅ |
| `cli/discover/list.rs` | 209 | 118 | 91 | 43.5% | ✅ |
| `cli/discover/mod.rs` | 97 | 68 | 29 | 29.9% | ✅ |
| `cli/discover/show.rs` | 261 | 168 | 93 | 35.6% | ✅ |
| `cli/hegel.rs` | 156 | 95 | 61 | 39.1% | ✅ |
| `debug.rs` | 46 | 18 | 28 | 60.9% | ✅ |
| `discovery/api_types.rs` | 46 | 46 | 0 | 0.0% | ✅ |
| `discovery/cache.rs` | 633 | 245 | 388 | 61.3% | ⚠️ Large |
| `discovery/config.rs` | 234 | 113 | 121 | 51.7% | ✅ |
| `discovery/discover.rs` | 200 | 52 | 148 | 74.0% | ✅ |
| `discovery/engine.rs` | 211 | 77 | 134 | 63.5% | ✅ |
| `discovery/mod.rs` | 35 | 24 | 11 | 31.4% | ✅ |
| `discovery/project.rs` | 240 | 117 | 123 | 51.2% | ✅ |
| `discovery/state.rs` | 109 | 19 | 90 | 82.6% | ✅ |
| `discovery/statistics.rs` | 30 | 3 | 27 | 90.0% | ✅ |
| `discovery/walker.rs` | 171 | 46 | 125 | 73.1% | ✅ |
| `lib.rs` | 11 | 9 | 2 | 18.2% | ✅ |
| `main.rs` | 32 | 32 | 0 | 0.0% | ✅ |
| `test_helpers.rs` | 61 | 35 | 26 | 42.6% | ✅ |

**⚠️ Warning:** 2 file(s) over 200 impl lines - consider splitting for maintainability

---

## Documentation Files

| File | Lines |
|------|-------|
| `.ddd/feat/benchmark/PLAN.md` | 324 |
| `.ddd/feat/benchmark/SPEC.md` | 418 |
| `.ddd/feat/binary-cache/HANDOFF.md` | 198 |
| `.ddd/feat/binary-cache/PLAN.md` | 320 |
| `.ddd/feat/binary-cache/SPEC.md` | 322 |
| `.ddd/feat/cli-discovery/PLAN.md` | 391 |
| `.ddd/feat/cli-discovery/SPEC.md` | 557 |
| `.ddd/feat/extract-web-ui/PLAN.md` | 360 |
| `.ddd/feat/extract-web-ui/SPEC.md` | 82 |
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
| `CLAUDE.md` | 420 |
| `COVERAGE_REPORT.md` | 56 |
| `learnings/.ddd/0_sycamore_foundations_assessment.md` | 335 |
| `learnings/.ddd/1_open_questions.md` | 294 |
| `learnings/LEARNING_SYCAMORE_COMPONENTS.md` | 733 |
| `learnings/LEARNING_SYCAMORE_FOUNDATIONS.md` | 572 |
| `learnings/LEARNING_SYCAMORE_PRACTICES.md` | 530 |
| `LOC_REPORT.md` | 152 |
| `MANUAL_EDITS.md` | 53 |
| `README.md` | 297 |
| `RESEARCH_PLAN.md` | 189 |
| `scripts/README.md` | 101 |
| `src/cli/discover/README.md` | 29 |
| `src/cli/README.md` | 23 |
| `src/discovery/README.md` | 53 |
| `src/README.md` | 33 |
| `TESTING.md` | 510 |
| `VISION.md` | 115 |

---

## Documentation Quality Targets

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Docs/Code Ratio | ≥0.3 | 4.28 | ✅ Excellent |
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
