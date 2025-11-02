# Lines of Code Report

**Last Updated**: 2025-11-02 17:28
**Tool**: [cloc](https://github.com/AlDanial/cloc) + wc

---

## Overall Summary

| Metric | Rust Code | Documentation (.md) | Total |
|--------|-----------|---------------------|-------|
| **Lines** | 592 | 1,620 | 2,212 |
| **Comments** | 58 | - | 58 |
| **Blank Lines** | 126 | - | 126 |
| **Total Lines** | 776 | 1,620 | 2,396 |
| **Files** | 7 | 7 | 14 |

**Documentation Ratio**: 2.74 lines of docs per line of code

---

## Rust Code Breakdown

```
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                             7            126             58            592
-------------------------------------------------------------------------------
SUM:                             7            126             58            592
-------------------------------------------------------------------------------
```

---

## Rust File Details

| File | Total Lines | Impl Lines | Test Lines | Test % | Status |
|------|-------------|------------|------------|--------|--------|
| `discovery/config.rs` | 212 | 105 | 107 | 50.5% | ✅ |
| `discovery/mod.rs` | 25 | 14 | 11 | 44.0% | ✅ |
| `discovery/project.rs` | 221 | 98 | 123 | 55.7% | ✅ |
| `discovery/state.rs` | 113 | 19 | 94 | 83.2% | ✅ |
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
| `LOC_REPORT.md` | 82 |
| `VISION.md` | 115 |

---

## Documentation Quality Targets

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Docs/Code Ratio | ≥0.3 | 2.74 | ✅ Excellent |
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
