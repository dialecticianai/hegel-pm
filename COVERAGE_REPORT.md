# Test Coverage Report

**Last Updated**: 2025-11-02 17:31
**Tool**: cargo-llvm-cov
**Overall Coverage**: **33.96%** lines | **39.79%** regions | **28.15%** functions

## Summary

```
TOTAL                                        3805              2291    39.79%         270               194    28.15%        2332              1540    33.96%           0                 0         -
```

## Coverage by Module

| Module | Line Coverage | Region Coverage | Functions | Status |
|--------|--------------|-----------------|-----------|--------|

## Coverage Tiers

### 🟢 Excellent (≥90% lines)

### 🟡 Good (70-89% lines)

### 🟠 Moderate (40-69% lines)

### 🔴 Needs Work (<40% lines)

## Coverage Targets

| Tier | Target | Current | Status |
|------|--------|---------|--------|
| Overall | ≥80% | 33.96% | ⏳ In Progress |
| Critical Paths | ≥95% | Check modules above | Policy |
| New Modules | ≥80% | - | Policy |

## How to Update This Report

```bash
# Regenerate coverage report
./scripts/generate-coverage-report.sh
```

## Quick Commands

```bash
# Run tests with coverage
cargo llvm-cov --html      # Detailed HTML
cargo llvm-cov --summary-only  # Terminal summary

# Update this markdown report
./scripts/generate-coverage-report.sh
```

---

*This report is auto-generated from `cargo llvm-cov` output.*
