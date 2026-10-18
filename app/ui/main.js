// `window.__TAURI__` exists because of `"withGlobalTauri": true` in tauri.conf.json.
const { invoke } = window.__TAURI__.core;

// Calls the Rust function `app_version` in src-tauri/src/commands.rs.
invoke("app_version").then((version) => {
  document.getElementById("version").textContent = `v${version}`;
});
