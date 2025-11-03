# src/discovery/

Auto-discovery and state management for Hegel projects across the filesystem.

## Module Root

### **mod.rs**
Module exports and public API surface for discovery engine.

## Configuration

### **config.rs**
Discovery configuration with search roots, exclusions, and cache settings.

## Discovery Engine

### **engine.rs**
Orchestrates project discovery with caching and background refresh.

### **walker.rs**
Filesystem walker to locate `.hegel/` directories across search roots.

### **discover.rs**
Core discovery logic: scans directories, loads state, constructs project objects.

## Data Models

### **project.rs**
DiscoveredProject model with workflow state, statistics, and metadata. Implements lazy-loading for metrics via `load_statistics()`.

### **state.rs**
Loads workflow state from `.hegel/state.json` using hegel-cli's FileStorage.

### **statistics.rs**
Type alias to hegel-cli's UnifiedMetrics for comprehensive workflow analytics.

## Caching

### **cache.rs**
Persistent cache for discovered projects with atomic writes and expiration.
