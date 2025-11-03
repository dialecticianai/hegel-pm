pub mod discovery;

#[cfg(test)]
mod test_helpers;

// Client WASM module
#[cfg(target_arch = "wasm32")]
pub mod client;
