# Swappable Frontend Implementation Plan

Implementation plan for enabling multiple frontend implementations with pure JavaScript first-class support.

---

## Overview

**Goal**: Enable alternative frontend implementations (Alpine.js, React, Vue, etc.) to be built and served from the same backend, with Alpine.js as proof-of-concept demonstrating the pattern.

**Scope**:
- Create frontends directory structure
- Build Alpine.js single-file frontend consuming existing API
- Update build scripts to support frontend selection via environment variable
- Document the pattern for future frontend additions
- Maintain backward compatibility (default = Sycamore)

**Priorities**:
1. Backward compatibility - existing workflow unchanged
2. Clean separation - each frontend self-contained
3. Simple integration - minimal script changes
4. Clear documentation - adding frontends is repeatable

---

## Methodology

**TDD Approach**: This feature is primarily infrastructure (build scripts, static files, documentation) rather than algorithmic logic. Testing strategy focuses on:
- Integration testing: verify builds complete and serve correctly
- Manual browser testing: verify frontends display and function
- Script behavior testing: verify env var handling and error cases

**What to test**:
- Build script dispatches to correct frontend based on env var
- Static directory contains expected files after build
- Backend serves frontend correctly (existing backend tests cover this)
- Error cases: invalid frontend names, missing directories

**What NOT to test**:
- Alpine.js framework internals (trust the library)
- Browser DOM rendering details (manual verification sufficient)
- CSS styling correctness (subjective, manual testing)
- Individual build tool behavior (Trunk, npm - trust the tools)

---

## Step 1: Create Directory Structure and Alpine.js Frontend

### Goal
Establish the frontends directory and create a working Alpine.js proof-of-concept that demonstrates an alternative frontend consuming the same API as Sycamore.

### Step 1.a: Test Strategy
No automated tests for this step. Manual verification will confirm:
- Directory structure is correct
- Alpine.js HTML is valid and loads in browser
- API calls work (inspect network tab)
- Data renders correctly

### Step 1.b: Implement
Create the directory structure and Alpine.js implementation:
- Create frontends directory at repository root
- Create frontends/alpine subdirectory
- Write single-file Alpine.js application in frontends/alpine/index.html
- Use Alpine.js from CDN (unpkg or esm.sh) to avoid build dependencies
- Reuse CSS styles from existing index.html where applicable
- Implement project list fetching from /api/projects endpoint
- Implement project detail view fetching from /api/projects/{name}/metrics endpoint
- Use Alpine.js reactive data binding for state management
- Render project list in sidebar, detail view in main content area
- Handle loading states and basic error cases

### Success Criteria
- frontends/alpine/index.html exists and contains valid HTML
- Alpine.js loaded from CDN in script tag
- File includes CSS for basic layout (sidebar + main content)
- Fetch calls to both API endpoints present in JavaScript
- Alpine.js x-data and reactive directives used for state
- Manual test: copy file to static/ and open in browser shows UI
- Manual test: project list loads when backend is running
- Manual test: clicking project shows metrics detail view

---

## Step 2: Update test.sh Script for Frontend Selection

### Goal
Modify the test.sh script to support building different frontends based on FRONTEND environment variable while maintaining backward compatibility.

### Step 2.a: Test Strategy
Test by running the script with different inputs:
- No env var (default to Sycamore)
- FRONTEND=sycamore (explicit default)
- FRONTEND=alpine (new frontend)
- FRONTEND=invalid (error case)

Verify the static directory contains correct files after each build.

### Step 2.b: Implement
Modify scripts/test.sh to support frontend selection:
- Read FRONTEND environment variable at script start
- Default to "sycamore" if unset
- Add conditional logic to dispatch to appropriate build command
- For "sycamore": run trunk build --release (existing behavior)
- For "alpine": copy frontends/alpine/* to static/
- For unknown values: print error message and exit with code 1
- Add clear echo statements indicating which frontend is being built
- Ensure backend build still happens after frontend build
- Ensure tests still run after builds complete
- Handle --exclude frontend flag (skip frontend build entirely)

### Success Criteria
- Running test.sh without env var builds Sycamore (backward compatible)
- FRONTEND=sycamore builds Sycamore explicitly
- FRONTEND=alpine copies Alpine files to static/
- FRONTEND=invalid prints error and exits with code 1
- Error message lists valid frontend options
- Backend build and tests still execute after frontend build
- Script output clearly indicates which frontend was selected
- --exclude frontend flag still skips frontend build

---

## Step 3: Update restart-server.sh Script for Frontend Selection

### Goal
Modify restart-server.sh to support frontend selection with same pattern as test.sh, enabling developers to switch frontends when restarting the server.

### Step 3.a: Test Strategy
Test by running the script with different combinations:
- No env var with --frontend flag (Sycamore rebuild)
- FRONTEND=alpine with --frontend flag (Alpine rebuild)
- FRONTEND=sycamore without --frontend flag (backend only, no frontend rebuild)
- Verify server restarts and serves correct frontend

### Step 3.b: Implement
Modify scripts/restart-server.sh with same frontend selection logic:
- Read FRONTEND environment variable (default "sycamore")
- Add conditional logic in frontend build section
- For "sycamore": run trunk build --release
- For "alpine": copy frontends/alpine/* to static/
- For unknown values: print error and exit
- Ensure logic only runs when --frontend flag is present
- Keep backend-only restart path unchanged
- Add clear output indicating frontend selection
- Maintain all existing logging behavior

### Success Criteria
- restart-server.sh without env var and with --frontend builds Sycamore
- FRONTEND=alpine with --frontend copies Alpine files
- FRONTEND=invalid prints error and exits
- Script without --frontend flag skips frontend build (existing behavior)
- Server starts successfully after any valid frontend build
- Log output clearly shows which frontend was built
- Manual test: server serves correct frontend in browser after restart

---

## Step 4: Create Alpine.js Documentation

### Goal
Document the Alpine.js frontend implementation so developers understand how it works and how to modify it.

### Step 4.a: Test Strategy
No automated tests. Manual review of documentation for:
- Completeness (all sections present)
- Clarity (understandable by someone unfamiliar with Alpine.js)
- Accuracy (matches actual implementation)

### Step 4.b: Implement
Create frontends/alpine/README.md with sections:
- Overview of Alpine.js frontend
- How to build and serve (using FRONTEND env var)
- How to develop locally (copy to static/ and refresh)
- API integration details (which endpoints, data flow)
- Architecture explanation (single-file, CDN imports, reactive state)
- How to modify the UI
- Comparison with Sycamore frontend (no build step, pure JS, no WASM)

### Success Criteria
- frontends/alpine/README.md exists
- Contains all planned sections
- Code examples (if any) are descriptive, not literal implementation
- Explains how to run and test the frontend
- Documents API endpoints and data structures consumed
- Readable by someone new to Alpine.js

---

## Step 5: Create General Frontend Addition Guide

### Goal
Provide step-by-step instructions for adding new frontends so the pattern is repeatable and discoverable.

### Step 5.a: Test Strategy
No automated tests. Manual review ensures guide is:
- Complete (all steps to add a frontend)
- Ordered correctly (logical sequence)
- Accurate (reflects actual script structure)

### Step 5.b: Implement
Create frontends/ADDING_FRONTENDS.md with comprehensive guide:
- Purpose and overview of swappable frontend architecture
- Prerequisites (what tools/knowledge needed)
- Step-by-step process for adding a new frontend
- Directory structure requirements
- Build configuration requirements (output to static/)
- API endpoint documentation with example responses
- Script modification instructions (where to add new cases)
- Testing approach (manual browser testing recommended)
- Troubleshooting common issues
- Reference to api_types.rs for data structures
- Examples: Alpine.js (no build), React (npm build), Vue (npm build)

### Success Criteria
- frontends/ADDING_FRONTENDS.md exists
- Contains numbered step-by-step instructions
- Includes directory structure example
- Documents API endpoints with descriptions
- Shows where to modify build scripts (prose, not code)
- Provides troubleshooting section
- References Alpine.js as working example
- Explains build output requirements (static/ directory)

---

## Step 6: Integration Testing and Validation

### Goal
Verify the complete swappable frontend system works end-to-end with multiple scenarios.

### Step 6.a: Test Strategy
Manual integration testing covering:
- Build both frontends successfully
- Switch between frontends
- Error handling for invalid frontends
- Backward compatibility (no env var)
- All combinations of script flags and env vars

### Step 6.b: Implement
Run comprehensive integration test scenarios:
- Clean build with no env var (should use Sycamore)
- Build with FRONTEND=sycamore explicitly
- Build with FRONTEND=alpine
- Attempt build with FRONTEND=nonexistent (should fail)
- Restart server with different frontends and verify UI changes
- Verify both frontends can fetch and display project data
- Check that static/ directory contents change appropriately
- Verify existing workflow (no FRONTEND var) still works
- Test --exclude frontend flag still works with both scripts

Document any issues found and fix before completing step.

### Success Criteria
- Both frontends build without errors
- Switching between frontends produces different UIs in browser
- Both frontends display same data from API
- Invalid frontend name produces clear error message
- Default behavior (no env var) unchanged from pre-feature
- Backend tests still pass: cargo test --features server
- Backend builds successfully: cargo build --release --features server
- Scripts provide clear output about frontend selection
- No manual code changes needed to switch frontends (just env var)

---

## Commit Discipline

After completing each numbered step:
- Stage all changes for that step
- Create commit with format: feat(frontends): complete Step N - description
- Include standard footer (Claude Code attribution)
- Proceed to next step

This maintains clean history and enables step-by-step review.

---

## Success Indicators

Feature is complete when:
- All six steps have passing success criteria
- Documentation is clear and complete
- Manual testing confirms both frontends work
- Backward compatibility verified (existing workflow unchanged)
- Error cases handled gracefully with helpful messages
- Ready for user acceptance testing
