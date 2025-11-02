# Lines of Code Report

**Last Updated**: 2025-11-02 17:24
**Tool**: [cloc](https://github.com/AlDanial/cloc) + wc

---

## Overall Summary

| Metric | Rust Code | Documentation (.md) | Total |
|--------|-----------|---------------------|-------|
| **Lines** | 188 | 1,616 | 1,804 |
| **Comments** | 15 | - | 15 |
| **Blank Lines** | 32 | - | 32 |
| **Total Lines** | 235 | 1,616 | 1,851 |
| **Files** | 4 | 7 | 11 |

**Documentation Ratio**: 8.60 lines of docs per line of code

---

## Rust Code Breakdown

```
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                             4             32             15            188
-------------------------------------------------------------------------------
SUM:                             4             32             15            188
-------------------------------------------------------------------------------
```

---

## Rust File Details

| File | Total Lines | Impl Lines | Test Lines | Test % | Status |
|------|-------------|------------|------------|--------|--------|
| `discovery/config.rs` | 212 | 105 | 107 | 50.5% | ✅ |
| `discovery/mod.rs` | 19 | 8 | 11 | 57.9% | ✅ |
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
| `LOC_REPORT.md` | 78 |
| `VISION.md` | 115 |

---

## Documentation Quality Targets

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Docs/Code Ratio | ≥0.3 | 8.60 | ✅ Excellent |
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
