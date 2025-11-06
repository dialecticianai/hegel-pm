# Alpine.js Frontend for Hegel PM

Pure JavaScript single-file frontend implementation using Alpine.js reactive framework.

---

## Overview

This is an alternative frontend for hegel-pm that demonstrates the swappable frontend architecture. Unlike the flagship Sycamore frontend (Rust/WASM), this implementation is:

- **Pure JavaScript** - No TypeScript, no build step beyond copying files
- **Single file** - Entire application in `index.html`
- **CDN-based** - Alpine.js loaded from CDN, no npm dependencies
- **Minimal** - Under 300 lines including CSS and JavaScript
- **Self-documenting** - Clear variable names and structure

---

## How to Build and Serve

### Quick Test (Manual)

Copy the file to the static directory and open in browser:

```bash
cp frontends/alpine/index.html static/
# Then open http://localhost:3030 (with backend running)
```

### Using Build Scripts

```bash
# Build and test
FRONTEND=alpine ./scripts/test.sh

# Restart server with Alpine frontend
FRONTEND=alpine ./scripts/restart-server.sh --frontend
```

### Development Workflow

1. Make changes to `frontends/alpine/index.html`
2. Copy to static: `FRONTEND=alpine ./scripts/test.sh --exclude backend`
3. Refresh browser (no server restart needed)
4. Repeat

---

## Architecture

### Single-File Structure

The entire application lives in `index.html` with three sections:

1. **CSS** - Reused from Sycamore frontend with minimal additions
2. **HTML** - Alpine.js directives (`x-data`, `x-for`, `x-if`, `x-text`)
3. **JavaScript** - Application logic in `hegelApp()` function

### State Management

Alpine.js uses reactive data via the `x-data` directive. The app state includes:

- `projects` - Array of project objects from `/api/projects`
- `selectedProjectName` - Currently selected project name (or null)
- `selectedProject` - Full project object for selected project
- `loading` - Boolean for loading state
- `error` - Error message string (or null)

### Data Flow

```
1. Page loads → init() called
2. init() → loadProjects()
3. loadProjects() → fetch('/api/projects')
4. Response → updates projects array
5. User clicks project → selectProject(name)
6. selectProject() → finds project in array, updates selectedProject
7. Alpine reactivity → UI updates automatically
```

---

## API Integration

### Endpoints Used

**GET /api/projects**
- Returns array of ProjectInfo objects
- Each contains: `project_name`, `summary`, `detail`
- Called once on page load

**Response structure:**
```javascript
[
  {
    project_name: "my-project",
    summary: {
      total_input_tokens: 12345,
      total_output_tokens: 6789,
      total_events: 42,
      phase_count: 3
    },
    detail: {
      current_workflow_state: {
        mode: "execution",
        current_node: "code"
      },
      workflows: [...]
    }
  }
]
```

### Error Handling

- Network errors caught and displayed in sidebar
- Failed requests show error message with HTTP status
- Console logging for debugging

---

## Key Features

### Sidebar

- Lists all projects from API
- Shows workflow state (mode/phase) or "No active workflow"
- Highlights selected project
- Click to view project details

### Main Content Area

**All Projects View** (no selection):
- Aggregate metrics across all projects
- Total project count
- Summed token counts and events

**Project Detail View** (project selected):
- Project name as heading
- Summary metrics in grid
- Workflow list with IDs, modes, and statuses

### Reactive Behavior

Alpine.js automatically updates UI when:
- Projects array populated from API
- Project selected/deselected
- Error state changes

---

## Comparison with Sycamore Frontend

| Aspect | Alpine.js | Sycamore |
|--------|-----------|----------|
| Language | Pure JavaScript | Rust |
| Build step | None (copy files) | Trunk + wasm-pack |
| Size | ~10KB (HTML) | ~400KB (WASM + JS glue) |
| Dependencies | Alpine.js from CDN | Sycamore, wasm-bindgen, etc. |
| Load time | Fast (no WASM init) | Slower (WASM compilation) |
| Type safety | None (dynamic JS) | Full (Rust compiler) |
| Reactivity | Coarse-grained | Fine-grained |

---

## How to Modify

### Add New Metrics to Display

1. Locate the metric grid in the template
2. Add new `metric-item` div with `x-text` binding:
   ```html
   <div class="metric-item">
       <div class="metric-label">My New Metric</div>
       <div class="metric-value" x-text="selectedProject.summary.my_metric"></div>
   </div>
   ```

### Change Styling

Modify the `<style>` section at the top of the file. CSS is standard, no preprocessing.

### Add API Calls

Add methods to the `hegelApp()` function:

```javascript
async fetchSomething() {
    const response = await fetch('/api/some-endpoint');
    const data = await response.json();
    this.someData = data;
}
```

Then call from template or other methods.

### Debug

Open browser console (F12) and check:
- Network tab for API requests
- Console tab for errors
- Alpine DevTools extension (optional)

---

## Limitations

This is a proof-of-concept implementation. It does not:

- Handle all edge cases
- Implement advanced UI features (collapsible workflows, etc.)
- Optimize for large numbers of projects
- Persist state across refreshes
- Implement real-time updates

For production features, use the Sycamore frontend.

---

## Resources

- [Alpine.js Documentation](https://alpinejs.dev/)
- [API Types Reference](../../src/api_types.rs)
- [General Frontend Guide](../ADDING_FRONTENDS.md)
