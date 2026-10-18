//! Placement of the island window on screen.

use tauri::{App, Manager, PhysicalPosition};

/// Gap between the top of the screen and the island, in physical pixels.
const TOP_MARGIN: i32 = 8;

/// Moves the island to the top-centre of the primary monitor, then shows it.
///
/// The window starts hidden (`"visible": false` in `tauri.conf.json`) so it
/// doesn't flash in the middle of the screen before being moved.
pub fn show_at_top_center(app: &App) -> tauri::Result<()> {
    let window = app
        .get_webview_window("island")
        .expect("the island window is declared in tauri.conf.json");

    if let Some(monitor) = window.primary_monitor()? {
        let screen = monitor.size();
        let origin = monitor.position();
        let size = window.outer_size()?;
        let x = origin.x + (screen.width as i32 - size.width as i32) / 2;
        window.set_position(PhysicalPosition::new(x, origin.y + TOP_MARGIN))?;
    }

    window.show()
}
