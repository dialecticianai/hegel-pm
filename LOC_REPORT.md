# Lines of Code Report

**Last Updated**: 2025-11-13 15:04
**Tool**: [cloc](https://github.com/AlDanial/cloc) + wc

---

## Overall Summary

| Metric | Rust Code | Documentation (.md) | Total |
|--------|-----------|---------------------|-------|
| **Lines** | 3,072 | 8,634 | 11,706 |
| **Comments** | 299 | - | 299 |
| **Blank Lines** | 607 | - | 607 |
| **Total Lines** | 3,978 | 8,634 | 12,612 |
| **Files** | 21 | 32 | 53 |

**Documentation Ratio**: 2.81 lines of docs per line of code

---

## Rust Code Breakdown

```
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                            21            607            299           3072
Markdown                         4             23              0            107
-------------------------------------------------------------------------------
SUM:                            25            630            299           3179
-------------------------------------------------------------------------------
```

---

## Rust File Details

| File | Total Lines | Impl Lines | Test Lines | Test % | Status |
|------|-------------|------------|------------|--------|--------|
| `cli.rs` | 239 | 73 | 166 | 69.5% | ✅ |
| `cli/discover/all.rs` | 400 | 273 | 127 | 31.8% | ⚠️ Large |
| `cli/discover/format.rs` | 128 | 52 | 76 | 59.4% | ✅ |
| `cli/discover/list.rs` | 209 | 118 | 91 | 43.5% | ✅ |
| `cli/discover/mod.rs` | 97 | 68 | 29 | 29.9% | ✅ |
| `cli/discover/show.rs` | 261 | 168 | 93 | 35.6% | ✅ |
| `cli/hegel.rs` | 156 | 95 | 61 | 39.1% | ✅ |
| `debug.rs` | 46 | 18 | 28 | 60.9% | ✅ |
| `discovery/api_types.rs` | 46 | 46 | 0 | 0.0% | ✅ |
| `discovery/cache.rs` | 1,006 | 388 | 618 | 61.4% | ⚠️ Large |
| `discovery/config.rs` | 234 | 113 | 121 | 51.7% | ✅ |
| `discovery/discover.rs` | 200 | 52 | 148 | 74.0% | ✅ |
| `discovery/engine.rs` | 211 | 77 | 134 | 63.5% | ✅ |
| `discovery/mod.rs` | 38 | 27 | 11 | 28.9% | ✅ |
| `discovery/project.rs` | 240 | 117 | 123 | 51.2% | ✅ |
| `discovery/state.rs` | 109 | 19 | 90 | 82.6% | ✅ |
| `discovery/statistics.rs` | 30 | 3 | 27 | 90.0% | ✅ |
| `discovery/walker.rs` | 171 | 46 | 125 | 73.1% | ✅ |
| `lib.rs` | 11 | 9 | 2 | 18.2% | ✅ |
| `main.rs` | 85 | 85 | 0 | 0.0% | ✅ |
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
| `ARCHITECTURE.md` | 257 |
| `CLAUDE.md` | 393 |
| `COVERAGE_REPORT.md` | 56 |
| `LOC_REPORT.md` | 125 |
| `README.md` | 190 |
| `scripts/README.md` | 72 |
| `src/cli/discover/README.md` | 29 |
| `src/cli/README.md` | 27 |
| `src/discovery/README.md` | 53 |
| `src/README.md` | 21 |
| `TESTING.md` | 573 |
| `VISION.md` | 109 |

---

## Documentation Quality Targets

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Docs/Code Ratio | ≥0.3 | 2.81 | ✅ Excellent |
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
