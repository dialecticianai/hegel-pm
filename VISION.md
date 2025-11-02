# hegel-pm Vision

Unified visual dashboard for tracking multiple Hegel projects across workflows and phases.

---

## Problem Statement

Developers using Hegel for structured development across multiple projects face a visibility gap. Each project's `.hegel/` directory contains rich workflow state (current phase, transition history, metrics, command logs), but there's no unified view.

The current reality:
- **Context switching overhead**: `cd` into each project, run `hegel status` or `hegel top`, mentally track which projects are active
- **No cross-project visibility**: Can't see at a glance which projects are in SPEC phase, which are in CODE, which workflows are stalled
- **State buried in CLI**: Rich JSONL event logs exist but require manual inspection with `cat` and `jq`

For developers juggling 5-10+ concurrent Hegel projects, this creates cognitive load and slows decision-making about where to focus next. The cost: workflow states drift from awareness, projects lose momentum invisibly.

---

## Target Users

**Primary**: Solo developers using Hegel methodology across multiple projects simultaneously (3-10+ active codebases). Developers who value visual clarity and want to see workflow health at a glance.

**Future (commercial)**: Development teams coordinating on shared Hegel workflows, needing team-wide visibility into project states.

**Not for**:
- Developers with only 1-2 Hegel projects (CLI tools suffice)
- Users wanting traditional project management (task assignment, time tracking, gantt charts)
- Teams needing real-time collaboration features in V1

---

## Solution Approach

**Core insight**: The state already exists (`.hegel/` directories), it just needs to be discovered, aggregated, and visualized. Don't create new state, surface existing state.

**Key capabilities**:
- **Auto-discovery**: Walk `~/Code` (configurable) to find all `.hegel/` directories without manual configuration
- **Multi-project dashboard**: Single-page view of all active Hegel projects with workflow states, current phases, recent activity
- **Visual workflow rendering**: See where each project is in its workflow cycle (SPEC → PLAN → CODE → etc.)
- **History and metrics**: Per-project transition logs, event counts, file modification patterns from JSONL logs
- **Live updates**: File watching on `.hegel/` directories for real-time state synchronization

**What we're NOT doing**:
- ❌ Not a code editor or IDE integration
- ❌ Not a replacement for Hegel CLI (CLI remains primary interface for workflow control)
- ❌ Not a traditional PM tool (no task lists, sprints, burndown charts)
- ❌ Not a SaaS/cloud service (local-first, reads local filesystem)

**Architecture approach**: Build with team features in mind but don't implement them yet. Data models and UI components should gracefully extend to multi-user when commercial version arrives (no fundamental rewrites).

---

## Success Criteria

### Qualitative
- [ ] User can open dashboard and immediately identify which projects need attention
- [ ] Switching between project contexts becomes visual (click) instead of mental (remember)
- [ ] Workflow state visibility reduces "what was I working on?" friction
- [ ] Dashboard becomes the default "morning view" for Hegel users

### Quantitative
- [ ] Auto-discovers 10+ projects from `~/Code` in <2 seconds
- [ ] Displays workflow state for all projects in single viewport
- [ ] Updates within 500ms of `.hegel/` state file changes
- [ ] Runs as local web server with <50MB memory footprint
- [ ] Zero manual configuration required (auto-discovery just works)

---

## Guiding Principles

### 1. **Visibility over convenience**
Show all state, even if it's verbose. Transparency beats polish. If `.hegel/state.json` has unexpected structure, surface it rather than hide errors gracefully.

### 2. **Local-first, always**
Read from filesystem, no databases. State lives in `.hegel/` directories, dashboard just renders it. User owns their data.

### 3. **CLI-first, GUI-second**
Hegel CLI remains the source of truth for workflow control. The dashboard is read-mostly with minimal write operations (future: trigger `hegel next` from UI, but CLI always works).

### 4. **Structure IS the state**
Don't invent new state formats. Parse existing JSONL logs (`hooks.jsonl`, `states.jsonl`, `command_log.jsonl`). Filesystem structure reveals project organization.

### 5. **Build for teams, ship for solo**
Data models should assume multi-user (project ownership, session attribution) but UI only renders single-user views in V1. When commercial version arrives, extend don't rewrite.

### 6. **No black boxes**
If the dashboard shows a metric, clicking it reveals the underlying JSONL events. Users can inspect raw data at any depth.

---

## Design Philosophy (from Hegel LEXICON)

**Context is king**: The dashboard's value is making workflow state visible. More context = better decisions.

**Artifacts disposable, clarity durable**: Code can be rewritten, but the insight "I have 3 stalled projects" is what matters.

**Infrastructure compounds**: Build reusable JSONL parsers, state models, Sycamore components that future tools can leverage.

**No black boxes**: All rules visible. If auto-discovery filters a directory, log why. If state parsing fails, show the error and raw JSON.

---

## Non-Goals (This Stage)

- Real-time collaboration (future: team version)
- Workflow control from UI (future: trigger `hegel next`, `hegel restart` from dashboard)
- Code preview or editing (not replacing editors)
- GitHub/GitLab integration (Hegel is local-first)
- Mobile app or responsive design (desktop-first for developers)

---

**In summary**: hegel-pm makes the invisible visible. Developers already have rich workflow state scattered across projects. We surface it in one unified, auto-discovered, visually clear dashboard. Success means never asking "which project was I working on?" again.
