#[cfg(not(target_arch = "wasm32"))]
pub mod discovery;

#[cfg(not(target_arch = "wasm32"))]
pub mod api_types;

#[cfg(test)]
mod test_helpers;

// Client WASM module
#[cfg(target_arch = "wasm32")]
pub mod client;
