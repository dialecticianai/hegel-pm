// Debug utilities (requires explicit import: use hegel_pm::debug;)
pub mod debug;

#[cfg(not(target_arch = "wasm32"))]
pub mod discovery;

#[cfg(not(target_arch = "wasm32"))]
pub mod api_types;

#[cfg(not(target_arch = "wasm32"))]
pub mod data_layer;

#[cfg(not(target_arch = "wasm32"))]
pub mod http;

#[cfg(not(target_arch = "wasm32"))]
pub mod benchmark_mode;

#[cfg(test)]
mod test_helpers;

// Client WASM module
#[cfg(target_arch = "wasm32")]
pub mod client;
