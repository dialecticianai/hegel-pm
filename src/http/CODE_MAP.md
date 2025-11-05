# src/http/

HTTP backend abstraction layer enabling compile-time selection between warp and axum.

## Structure

```
http/
├── mod.rs              HttpBackend trait, ServerConfig, mutual exclusion enforcement
├── warp_backend.rs     Warp implementation (default backend, feature-gated)
└── axum_backend.rs     Axum implementation (alternative backend, feature-gated)
```

## Key Patterns

**Trait abstraction**: HttpBackend trait provides uniform interface for both backends
**Compile-time selection**: Feature flags (warp-backend, axum-backend) control which backend builds
**Mutual exclusion**: compile_error! prevents enabling both backends simultaneously
**Zero blocking**: All backends delegate I/O to data_layer worker pool via message passing
