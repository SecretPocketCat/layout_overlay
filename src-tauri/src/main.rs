#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::AppHandle;
use tauri::CustomMenuItem;
use tauri::Manager;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::SystemTrayMenu;
use tauri::SystemTrayMenuItem;
use window_vibrancy::apply_blur;

const QUIT_ID: &str = "quit";
const TOGGLE_ID: &str = "toggle";

fn main() {
    let quit = CustomMenuItem::new(QUIT_ID.to_string(), "Quit");
    let hide = CustomMenuItem::new(TOGGLE_ID.to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                id if id == QUIT_ID => {
                    app.exit(0);
                }
                id if id == TOGGLE_ID => {
                    toggle_overlay(app);
                }
                _ => {}
            },
            _ => {}
        })
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            // events
            let handle = app.handle();
            app.listen_global("toggle", move |_| {
                toggle_overlay(&handle);
            });
            // todo:
            // tao
            // window.set_ignore_cursor_events(true);

            // window blur
            #[cfg(target_os = "windows")]
            apply_blur(&window, None)
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn toggle_overlay(app_handle: &AppHandle) {
    let window = app_handle.get_window("main").unwrap();
    let menu_item = app_handle.tray_handle().get_item(TOGGLE_ID);

    if window.is_visible().unwrap() {
        // todo: this crashes
        window.hide().unwrap();
        menu_item.set_title("Show").unwrap();
    } else {
        // todo: fix - messes up pos
        window.show().unwrap();
        menu_item.set_title("Hide").unwrap();
    }
}
