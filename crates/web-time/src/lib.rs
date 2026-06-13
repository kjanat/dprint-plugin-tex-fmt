// On non-wasm targets mirror std::time exactly.
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub use std::time::*;

// On wasm32-unknown-unknown provide a browser-free stub.
// tex-fmt uses Instant only for log-entry timestamps which this plugin
// discards entirely, so correctness of the counter does not matter.
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
mod wasm;
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub use wasm::*;
