# frontends/

Alternative frontend implementations for hegel-pm dashboard.

---

## Overview

This directory contains alternative frontend implementations that demonstrate the swappable frontend architecture. Each frontend consumes the same backend API (`/api/projects`, `/api/projects/{name}/metrics`) and is served from the `static/` directory.

The flagship Rust/Sycamore frontend lives in `src/client/` and is built with Trunk. Frontends here are alternatives showcasing different tech stacks (pure JS, React, Vue, etc.).

---

## Structure

```
frontends/
├── ADDING_FRONTENDS.md    Step-by-step guide for adding new frontends with API reference
│
└── alpine/                Alpine.js proof-of-concept frontend (pure JavaScript, no build step)
    └── See alpine/README.md
```

---

## Usage

Select frontend via `FRONTEND` environment variable:

```bash
# Build with Alpine.js frontend
FRONTEND=alpine ./scripts/test.sh

# Restart server with Alpine.js frontend
FRONTEND=alpine ./scripts/restart-server.sh --frontend

# Default (no env var) uses Sycamore
./scripts/test.sh
```

---

## Adding Frontends

See `ADDING_FRONTENDS.md` for comprehensive instructions on adding React, Vue, or other frontend implementations.

Key requirements:
- Output to `../../static/` directory
- Include `index.html` at root
- Fetch from `/api/projects` endpoint
- Add build case to `scripts/test.sh` and `scripts/restart-server.sh`
