// In release builds, don't open a console window next to the island on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod window;

fn main() {
    tauri::Builder::default()
        // Functions the UI can call with `invoke("name")`.
        .invoke_handler(tauri::generate_handler![commands::app_version])
        .setup(|app| {
            window::show_at_top_center(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Quran Island");
}
