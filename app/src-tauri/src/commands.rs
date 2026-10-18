//! Rust functions the UI can call.
//!
//! In JavaScript: `await window.__TAURI__.core.invoke("app_version")`.
//! Each one must also be listed in `generate_handler!` in `main.rs`.

/// The app version from `Cargo.toml`, e.g. "0.1.0".
#[tauri::command]
pub fn app_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
