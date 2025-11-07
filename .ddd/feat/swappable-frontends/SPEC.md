# Swappable Frontend Architecture Specification

Enable multiple frontend implementations to be built and served from the same backend, with pure JavaScript frontends as first-class citizens.

---

## Overview

**What it does:** Establishes infrastructure for building and serving alternative frontend implementations (React, Vue, Alpine.js, etc.) while maintaining the flagship Rust/Sycamore frontend. Any frontend can be built and served via the existing backend with no code changes to the backend itself.

**Key principles:**
- Backend remains frontend-agnostic (serves static files from `static/`, doesn't care what's in it)
- Each frontend is self-contained with its own tooling and dependencies
- Build scripts support frontend selection via environment variable
- Pure JavaScript frontends are first-class citizens (no TypeScript requirement)
- Adding a new frontend is a documented, repeatable process

**Scope:**
- Create `frontends/` directory structure with Alpine.js as proof-of-concept
- Update `scripts/test.sh` and `scripts/restart-server.sh` to support frontend selection via `FRONTEND` env var
- **Default behavior unchanged**: No env var = Sycamore frontend (backward compatible)
- Keep existing Sycamore frontend in `src/client/` as flagship implementation
- Document the pattern for adding future frontends

**Integration context:**
- Backend: No changes needed (src/http/ already serves from configurable `static_dir`)
- API: Existing endpoints (`/api/projects`, `/api/projects/{name}/metrics`) work with any frontend
- Build: Scripts modified to dispatch to appropriate frontend build tool

**Out of scope:**
- TypeScript support (explicitly forbidden by project requirements)
- Automated frontend testing (manual browser testing for now)
- JSDoc generation from Rust types (code is self-documenting)
- Moving Sycamore frontend out of `src/client/` (may happen in future)

---

## Data Model

### NEW: Directory Structure

```
hegel-pm/
├── frontends/                         # NEW - Alternative frontend implementations
│   ├── alpine/                        # NEW - Alpine.js proof-of-concept
│   │   ├── index.html                 # NEW - Single-file Alpine.js app
│   │   └── README.md                  # NEW - Alpine.js frontend docs
│   └── ADDING_FRONTENDS.md            # NEW - Guide for adding new frontends
│
├── src/client/                        # UNCHANGED - Flagship Sycamore frontend
├── static/                            # MODIFIED - Output from whichever frontend was built
└── scripts/
    ├── test.sh                        # MODIFIED - Frontend selection support
    └── restart-server.sh              # MODIFIED - Frontend selection support
```

### MODIFIED: Build Scripts

Both `scripts/test.sh` and `scripts/restart-server.sh` gain frontend selection capability:

**Environment variable:**
- `FRONTEND` - Frontend to build (default: `sycamore`)
- Valid values: `sycamore`, `alpine`, (future: `react`, `vue`, etc.)

**Behavior:**
- `FRONTEND=sycamore` - Runs `trunk build --release` (current behavior, default)
- `FRONTEND=alpine` - Copies `frontends/alpine/*` to `static/`
- Invalid value - Prints error and lists valid frontends

### NEW: Alpine.js Frontend

**Location:** `frontends/alpine/index.html`

**Structure:**
- Single HTML file with embedded Alpine.js
- Uses CDN imports (unpkg.com or esm.sh) for Alpine.js
- Consumes same API as Sycamore frontend
- Reuses CSS from current `index.html` where applicable
- Self-documenting JavaScript with clear variable names and comments

**API Integration:**
- Fetches `/api/projects` on load
- Fetches `/api/projects/{name}/metrics` when project clicked
- Renders same data as Sycamore frontend (project list, metrics, workflows)

### NEW: Documentation

**`frontends/ADDING_FRONTENDS.md`:**
- Step-by-step guide for adding a new frontend
- Required files and structure
- How to integrate with build scripts
- API endpoint documentation
- Example script modifications

**`frontends/alpine/README.md`:**
- Alpine.js implementation overview
- How to run/develop
- API data flow
- Modification guide

---

## Core Operations

### Operation 1: Build with Specific Frontend

**Syntax:**
```bash
FRONTEND=<name> ./scripts/test.sh
FRONTEND=<name> ./scripts/restart-server.sh [--frontend]
```

**Parameters:**
- `FRONTEND` (optional) - Frontend name (default: `sycamore`)

**Examples:**
```bash
# Build with default (Sycamore)
./scripts/test.sh

# Build with Alpine.js
FRONTEND=alpine ./scripts/test.sh

# Restart server with Alpine.js frontend rebuild
FRONTEND=alpine ./scripts/restart-server.sh --frontend
```

**Behavior:**
1. Script reads `FRONTEND` environment variable (defaults to `sycamore`)
2. Dispatches to appropriate build command:
   - `sycamore` → `trunk build --release`
   - `alpine` → `cp -r frontends/alpine/* static/`
   - Unknown → Error with list of valid frontends
3. Continues with backend build (unchanged)
4. For `restart-server.sh`: starts server (unchanged)

**Validation:**
- If `FRONTEND` is set but invalid, script exits with error
- If frontend directory doesn't exist, script exits with error
- Output clearly indicates which frontend was built

**Error cases:**
```bash
FRONTEND=react ./scripts/test.sh
# Output: Error: Unknown frontend 'react'
#         Valid frontends: sycamore, alpine

FRONTEND=alpine ./scripts/test.sh
# (but frontends/alpine/ doesn't exist)
# Output: Error: Frontend directory not found: frontends/alpine/
```

### Operation 2: Add New Frontend

**Process** (documented in `frontends/ADDING_FRONTENDS.md`):

1. **Create frontend directory:**
   ```bash
   mkdir -p frontends/<name>
   cd frontends/<name>
   ```

2. **Set up build tooling:**
   - No build step (HTML/JS): Just create `index.html`
   - npm-based (React/Vue): Run `npm init`, add build scripts
   - Other: Document build process in `frontends/<name>/README.md`

3. **Configure build output:**
   - All frontends must output to `../../static/` directory
   - Must include `index.html` at root
   - WASM/JS bundles can have any naming scheme

4. **Implement API integration:**
   - Fetch `/api/projects` for project list
   - Fetch `/api/projects/{name}/metrics` for project details
   - Refer to `src/api_types.rs` for response structure

5. **Update build scripts:**

   Add case to both `scripts/test.sh` and `scripts/restart-server.sh`:

   ```bash
   # In the frontend build section:
   elif [ "$FRONTEND" = "yourname" ]; then
       echo "🎨 Building yourname frontend..."
       cd frontends/yourname && <your-build-command>
   ```

6. **Test:**
   ```bash
   FRONTEND=yourname ./scripts/test.sh
   FRONTEND=yourname ./scripts/restart-server.sh --frontend
   ```

7. **Document:**
   - Create `frontends/<name>/README.md`
   - Update `frontends/ADDING_FRONTENDS.md` if pattern is new

### Operation 3: Switch Between Frontends

**Syntax:**
```bash
FRONTEND=<name> ./scripts/restart-server.sh --frontend
```

**Behavior:**
1. Stops existing server
2. Rebuilds frontend (overwrites `static/`)
3. Rebuilds backend
4. Starts server with new frontend

**Example:**
```bash
# Currently running with Sycamore
FRONTEND=alpine ./scripts/restart-server.sh --frontend
# Server restarts with Alpine.js frontend
```

---

## Test Scenarios

### Simple: Build and Serve Alpine.js Frontend

**Setup:**
- Fresh clone of hegel-pm
- `frontends/alpine/index.html` exists

**Steps:**
1. `FRONTEND=alpine ./scripts/test.sh`
2. `FRONTEND=alpine ./scripts/restart-server.sh --frontend`
3. Open browser to `http://localhost:3030`

**Expected:**
- Alpine.js frontend builds (files copied to `static/`)
- Backend builds successfully
- Server starts on port 3030
- Browser shows Alpine.js UI
- Project list loads from `/api/projects`
- Clicking project loads metrics from `/api/projects/{name}/metrics`

### Complex: Switch Between Frontends

**Setup:**
- Server running with Sycamore frontend
- Both `src/client/` and `frontends/alpine/` exist

**Steps:**
1. Verify Sycamore frontend is active (browser shows Rust/WASM UI)
2. `FRONTEND=alpine ./scripts/restart-server.sh --frontend`
3. Refresh browser
4. `FRONTEND=sycamore ./scripts/restart-server.sh --frontend`
5. Refresh browser again

**Expected:**
- First restart: Alpine.js frontend loads, pure JS, no WASM
- Second restart: Sycamore frontend loads, WASM bundle present
- Both frontends display same data (same API calls)
- No backend code changes needed
- `static/` directory contents change between builds

### Error: Invalid Frontend Name

**Setup:**
- Clean repository

**Steps:**
1. `FRONTEND=nonexistent ./scripts/test.sh`

**Expected:**
- Script exits with error code 1
- Error message: "Error: Unknown frontend 'nonexistent'"
- Lists valid frontends: "Valid frontends: sycamore, alpine"
- No build occurs
- `static/` directory unchanged

### Error: Missing Frontend Directory

**Setup:**
- `frontends/react/` does not exist
- Build script has `react` case but directory is missing

**Steps:**
1. `FRONTEND=react ./scripts/test.sh`

**Expected:**
- Script detects missing directory
- Error message: "Error: Frontend directory not found: frontends/react/"
- Script exits with error code 1
- Suggests checking `frontends/ADDING_FRONTENDS.md`

---

## Success Criteria

Agent-verifiable criteria:

- `frontends/alpine/index.html` exists and contains valid HTML
- `frontends/alpine/README.md` exists and documents the frontend
- `frontends/ADDING_FRONTENDS.md` exists and provides step-by-step guide
- `scripts/test.sh` modified to support `FRONTEND` environment variable
- `scripts/restart-server.sh` modified to support `FRONTEND` environment variable
- `./scripts/test.sh` (no env var) builds Sycamore frontend (backward compatible)
- `FRONTEND=sycamore ./scripts/test.sh` succeeds (explicit default)
- `FRONTEND=alpine ./scripts/test.sh` succeeds (copies files to `static/`)
- `FRONTEND=invalid ./scripts/test.sh` fails with error message
- `static/index.html` exists after either frontend build
- Backend tests still pass: `cargo test --features server`
- Backend builds successfully: `cargo build --release --features server`
- No changes to `src/http/` directory (backend unchanged)
- Scripts provide clear output indicating which frontend was built

---

## Optional Human Testing

Subjective QA concerns requiring manual verification:

- Alpine.js UI displays correctly in browser
- Project list is readable and navigable
- Clicking projects shows detail view
- Metrics display accurately
- UI layout is reasonable (doesn't need to match Sycamore exactly)
- Switching frontends shows visually different UIs
- Both frontends feel responsive
- No console errors in browser developer tools
