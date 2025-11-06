# Adding New Frontends to Hegel PM

Step-by-step guide for creating alternative frontend implementations.

---

## Overview

Hegel PM supports swappable frontends through a simple file-based architecture. Any framework or approach can be used as long as it:

1. Outputs to the `static/` directory
2. Includes an `index.html` at the root
3. Consumes the same HTTP API endpoints

The backend is completely frontend-agnostic - it just serves static files and provides JSON API endpoints.

---

## Prerequisites

**Required knowledge:**
- Basic HTTP/REST API concepts
- Your chosen frontend framework
- How your framework's build tool works (if applicable)

**Required tools:**
- Backend running on `localhost:3030`
- Build tools for your frontend (npm, etc., or none for simple HTML/JS)

**Recommended:**
- Review existing frontends in this directory for examples
- Read API type definitions in `src/api_types.rs`

---

## Quick Reference

**Existing frontends:**
- **Sycamore** (src/client/) - Rust/WASM, Trunk build, flagship implementation
- **Alpine.js** (frontends/alpine/) - Pure JS, no build step, proof-of-concept

**API endpoints:**
- `GET /api/projects` - List all projects with summary + detail
- `GET /api/projects/{name}/metrics` - (Currently same data, may differ in future)

**Build script locations:**
- `scripts/test.sh` - Build + test
- `scripts/restart-server.sh` - Build + restart server

---

## Step-by-Step Process

### Step 1: Create Frontend Directory

Create a directory under `frontends/` with your frontend name:

```bash
mkdir -p frontends/my-frontend
cd frontends/my-frontend
```

**Naming conventions:**
- Lowercase, hyphenated (e.g., `alpine`, `react-spa`, `vue3`)
- Short and descriptive
- Must match what you'll use in `FRONTEND` environment variable

### Step 2: Set Up Your Framework

Initialize your frontend project with whatever tools your framework requires.

**Examples:**

**No build step (HTML/JS):**
```bash
# Just create index.html - that's it!
```

**npm-based (React, Vue, etc.):**
```bash
npm init -y
npm install react react-dom  # or vue, etc.
npm install -D vite  # or webpack, parcel, etc.
```

**Other:**
- Deno, Bun, etc. - follow their conventions
- Just ensure build output goes to correct location (next step)

### Step 3: Configure Build Output

**Critical requirement:** Your build must output to `../../static/` directory.

**For build tools:**

Edit your build config to output to the static directory:

```javascript
// vite.config.js example
export default {
  build: {
    outDir: '../../static',
    emptyOutDir: true  // Clear old files
  }
}
```

```javascript
// webpack.config.js example
module.exports = {
  output: {
    path: path.resolve(__dirname, '../../static'),
    filename: 'bundle.js'
  }
}
```

**For no-build frontends:**

Your files will be copied directly, so ensure `index.html` is at the root of your frontend directory.

### Step 4: Implement API Integration

Your frontend must fetch data from these endpoints:

#### GET /api/projects

Returns array of all projects:

```javascript
const response = await fetch('/api/projects');
const projects = await response.json();
// projects is array of ProjectInfo objects
```

**Response structure (see src/api_types.rs for authoritative types):**

```javascript
[
  {
    project_name: "string",
    summary: {
      total_input_tokens: number,
      total_output_tokens: number,
      total_cache_creation_tokens: number,
      total_cache_read_tokens: number,
      total_all_tokens: number,
      total_events: number,
      bash_command_count: number,
      file_modification_count: number,
      git_commit_count: number,
      phase_count: number
    },
    detail: {
      current_workflow_state: {
        mode: "string",
        current_node: "string"
      } | null,
      workflows: [
        {
          workflow_id: "ISO-8601-timestamp",
          mode: "string",
          status: "Active" | "Completed" | "Aborted",
          current_phase: "string" | null,
          phases: [
            {
              phase_name: "string",
              status: "InProgress" | "Completed",
              start_time: "ISO-8601",
              end_time: "ISO-8601" | null,
              duration_seconds: number,
              metrics: {
                total_input_tokens: number,
                total_output_tokens: number,
                total_cache_creation_tokens: number,
                total_cache_read_tokens: number,
                total_all_tokens: number,
                event_count: number,
                bash_command_count: number,
                file_modification_count: number,
                git_commit_count: number
              }
            }
          ],
          total_metrics: { /* same structure as phase metrics */ }
        }
      ]
    }
  }
]
```

**Error handling:**

```javascript
try {
  const response = await fetch('/api/projects');
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  const data = await response.json();
} catch (error) {
  console.error('Failed to load projects:', error);
  // Display error to user
}
```

### Step 5: Build and Test Locally

**Manual testing (no build step):**

```bash
# Copy files to static
cp -r frontends/my-frontend/* static/

# Start backend (if not running)
cargo run --bin hegel-pm --features server --release

# Open browser to http://localhost:3030
```

**With build step:**

```bash
# Run your build command
cd frontends/my-frontend
npm run build  # or whatever your build command is

# Verify output in ../../static/
ls ../../static/

# Start backend and test in browser
```

**Development mode (if your framework supports it):**

Many frameworks have dev servers with hot reload. Configure them to proxy API requests:

```javascript
// vite.config.js example
export default {
  server: {
    proxy: {
      '/api': 'http://localhost:3030'
    }
  }
}
```

Then run dev server:
```bash
npm run dev
# Visit http://localhost:5173 (or whatever port your dev server uses)
```

### Step 6: Update Build Scripts

Add your frontend to both build scripts.

**Edit `scripts/test.sh`:**

Find the `case "$FRONTEND" in` block and add your frontend:

```bash
case "$FRONTEND" in
    sycamore)
        trunk build --release
        ;;
    alpine)
        # ... existing alpine case
        ;;
    my-frontend)  # ADD THIS
        if [ ! -d "frontends/my-frontend" ]; then
            echo "Error: Frontend directory not found: frontends/my-frontend/"
            echo "See frontends/ADDING_FRONTENDS.md for setup instructions"
            exit 1
        fi
        # For no-build frontends:
        cp -r frontends/my-frontend/* static/
        # OR for npm-based frontends:
        # cd frontends/my-frontend && npm run build
        ;;
    *)
        echo "Error: Unknown frontend '$FRONTEND'"
        echo "Valid frontends: sycamore, alpine, my-frontend"  # UPDATE THIS
        exit 1
        ;;
esac
```

**Edit `scripts/restart-server.sh`:**

Add the same case block to the `if [ "$BUILD_FRONTEND" = true ];` section.

**Update usage docs:**

Update the header comments in both scripts to list your new frontend.

### Step 7: Create Documentation

Create `frontends/my-frontend/README.md` documenting:

- Overview of your frontend
- How to build and run it
- Development workflow
- Architecture/structure explanation
- How to modify
- Comparison with other frontends (optional)

See `frontends/alpine/README.md` for a good template.

### Step 8: Test All Scenarios

Verify your frontend works in all modes:

```bash
# Build with your frontend
FRONTEND=my-frontend ./scripts/test.sh

# Restart server with your frontend
FRONTEND=my-frontend ./scripts/restart-server.sh --frontend

# Verify default still works (Sycamore)
./scripts/test.sh

# Test error case
FRONTEND=typo ./scripts/test.sh
# Should show error message with valid frontend list
```

**Manual browser testing:**

- [ ] Projects load in sidebar
- [ ] Workflow states display correctly
- [ ] Clicking project shows detail view
- [ ] Metrics are accurate
- [ ] No console errors
- [ ] Layout is reasonable

---

## API Reference

### Data Types

See `src/api_types.rs` for authoritative type definitions. Here's a quick reference:

**ProjectInfo:**
- `project_name` - String
- `summary` - ProjectMetricsSummary (aggregate counts)
- `detail` - ProjectWorkflowDetail (current state + workflows)

**ProjectMetricsSummary:**
- Token counts (input, output, cache creation, cache read, all)
- Event counts (total, bash commands, file mods, git commits)
- Phase count

**WorkflowSummary:**
- `workflow_id` - ISO 8601 timestamp
- `mode` - String (e.g., "execution", "discovery")
- `status` - "Active" | "Completed" | "Aborted"
- `current_phase` - String or null
- `phases` - Array of PhaseSummary
- `total_metrics` - Aggregated metrics across all phases

**PhaseSummary:**
- `phase_name` - String
- `status` - "InProgress" | "Completed"
- `start_time`, `end_time` - ISO 8601 timestamps
- `duration_seconds` - Number
- `metrics` - PhaseMetricsSummary

### Development Tips

**No TypeScript requirement:**

This project explicitly avoids TypeScript. Use pure JavaScript with clear variable names and JSDoc comments if you want type hints.

**Self-documenting code:**

Write code that's readable without extensive comments. Good variable names and clear structure are preferred.

**Testing:**

Manual browser testing is sufficient for frontends. The backend has comprehensive tests covering the API.

---

## Troubleshooting

### Build script doesn't find my frontend

- Check directory name matches exactly (case-sensitive)
- Ensure you added case to both test.sh and restart-server.sh
- Verify directory exists: `ls -la frontends/`

### API requests fail with 404

- Backend must be running on `localhost:3030`
- Check API endpoint paths match exactly: `/api/projects`
- Verify in Network tab (browser DevTools)

### Build output in wrong location

- Check your build config `outDir` / `output.path`
- Should be absolute or relative to frontend directory: `../../static/`
- Verify with `ls -la ../../static/` after build

### Static files not served

- Ensure `index.html` is at root of static directory
- Check for build errors in script output
- Restart server to pick up new files

### Data not displaying

- Open browser console (F12), check for errors
- Verify API response in Network tab
- Check variable names match response structure
- Confirm async/await or promises used correctly

---

## Examples

### No-Build Frontend (HTML + Alpine.js)

See `frontends/alpine/` - single HTML file, CDN imports, copy files to static.

### npm-Based Frontend (React + Vite)

```bash
mkdir frontends/react-spa && cd frontends/react-spa
npm init -y
npm install react react-dom
npm install -D vite @vitejs/plugin-react
```

**vite.config.js:**
```javascript
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  build: {
    outDir: '../../static',
    emptyOutDir: true
  },
  server: {
    proxy: {
      '/api': 'http://localhost:3030'
    }
  }
})
```

**Build script case:**
```bash
react-spa)
    if [ ! -d "frontends/react-spa" ]; then
        echo "Error: Frontend directory not found: frontends/react-spa/"
        exit 1
    fi
    cd frontends/react-spa && npm run build
    ;;
```

---

## Questions?

- Check existing frontend implementations in this directory
- Review API types in `src/api_types.rs`
- Look at backend tests in `src/http/` for API behavior
- See `ARCHITECTURE.md` for backend design decisions
