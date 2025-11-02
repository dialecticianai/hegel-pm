# Lines of Code Report

**Last Updated**: 2025-11-02 17:25
**Tool**: [cloc](https://github.com/AlDanial/cloc) + wc

---

## Overall Summary

| Metric | Rust Code | Documentation (.md) | Total |
|--------|-----------|---------------------|-------|
| **Lines** | 359 | 1,618 | 1,977 |
| **Comments** | 34 | - | 34 |
| **Blank Lines** | 65 | - | 65 |
| **Total Lines** | 458 | 1,618 | 2,076 |
| **Files** | 5 | 7 | 12 |

**Documentation Ratio**: 4.51 lines of docs per line of code

---

## Rust Code Breakdown

```
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                             5             65             34            359
-------------------------------------------------------------------------------
SUM:                             5             65             34            359
-------------------------------------------------------------------------------
```

---

## Rust File Details

| File | Total Lines | Impl Lines | Test Lines | Test % | Status |
|------|-------------|------------|------------|--------|--------|
| `discovery/config.rs` | 212 | 105 | 107 | 50.5% | ✅ |
| `discovery/mod.rs` | 21 | 10 | 11 | 52.4% | ✅ |
| `discovery/project.rs` | 221 | 98 | 123 | 55.7% | ✅ |
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
| `LOC_REPORT.md` | 80 |
| `VISION.md` | 115 |

---

## Documentation Quality Targets

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Docs/Code Ratio | ≥0.3 | 4.51 | ✅ Excellent |
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
