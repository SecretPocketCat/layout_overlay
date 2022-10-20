#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use event_loop::listen;
use hidapi::HidApi;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use tauri::AppHandle;
use tauri::CustomMenuItem;
use tauri::GlobalShortcutManager;
use tauri::Manager;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::SystemTrayMenu;
use tauri::SystemTrayMenuItem;
use tauri::WindowEvent;
use window_vibrancy::apply_blur;

mod event_loop;

const QUIT_ID: &str = "quit";
const TOGGLE_ID: &str = "toggle";
const LAYER_EV: &str = "layer";
const CAPS_WORD_EV: &str = "capsword";
const CAPS_LOCK_EV: &str = "capslock";
const SHIFT_EV: &str = "shift";
const HID_VENDOR_ID: u16 = 0x3a3c;
const HID_PROD_ID: u16 = 0x0001;
const HID_USAGE: u16 = 0x61;
const HID_USAGE_PAGE: u16 = 0xFF60;

#[derive(Debug)]
enum BoardEvent {
    Connection(bool),
    Layer(u8),
    CapsWord(bool),
    CapsLock(bool),
    Shift(bool),
}

fn main() {
    let (ev_sender, ev_receiver) = crossbeam::channel::unbounded();

    let quit = CustomMenuItem::new(QUIT_ID.to_string(), "Quit");
    let hide = CustomMenuItem::new(TOGGLE_ID.to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);

    let hid_tx = ev_sender.clone();
    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                id if id == QUIT_ID => {
                    app.exit(0);
                }
                id if id == TOGGLE_ID => {
                    toggle_overlay(app, None);
                }
                _ => {}
            },
            _ => {}
        })
        .on_window_event(move |ev| match ev.event() {
            WindowEvent::InputDeviceAdded | WindowEvent::InputDeviceRemoved => {
                println!("device change!");
                hid_tx
                    .send(BoardEvent::Connection(check_board_connection()))
                    .unwrap();
            }
            _ => {}
        })
        .setup(move |app| {
            let window = app.get_window("main").unwrap();
            // Global shortcuts
            let app_handle = app.handle();
            let mut shortcuts = app.global_shortcut_manager();
            shortcuts
                .register("CmdOrCtrl+F24", move || {
                    toggle_overlay(&app_handle.clone(), None)
                })
                .unwrap();

            for i in 0..=7 {
                let tx = ev_sender.clone();
                shortcuts
                    .register(&format!("CmdOrCtrl+F{}", i + 13), move || {
                        tx.send(BoardEvent::Layer(i)).ok();
                    })
                    .unwrap();
            }

            // send events to client
            let app_handle = app.handle();
            thread::spawn(move || loop {
                let app_handle = app_handle.clone();
                match ev_receiver.recv().unwrap() {
                    BoardEvent::Connection(connected) => {
                        toggle_overlay(&app_handle.clone(), Some(connected));
                    }
                    BoardEvent::Layer(layer) => {
                        app_handle.emit_all(LAYER_EV, layer).unwrap();
                    }
                    BoardEvent::CapsWord(active) => {
                        app_handle.emit_all(CAPS_WORD_EV, active).unwrap();
                    }
                    BoardEvent::CapsLock(active) => {
                        app_handle.emit_all(CAPS_LOCK_EV, active).unwrap();
                    }
                    BoardEvent::Shift(active) => {
                        app_handle.emit_all(SHIFT_EV, active).unwrap();
                    }
                };
            });

            window.set_ignore_cursor_events(true).unwrap();

            // window blur
            #[cfg(target_os = "windows")]
            apply_blur(&window, None)
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            listen(window.hwnd().unwrap().0);

            toggle_overlay(&app.handle(), Some(check_board_connection()));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn toggle_overlay(app_handle: &AppHandle, show: Option<bool>) {
    let window = app_handle.get_window("main").unwrap();
    let menu_item = app_handle.tray_handle().get_item(TOGGLE_ID);

    let show = show.unwrap_or(!window.is_visible().unwrap());

    if show {
        window.show().unwrap();
        menu_item.set_title("Hide").unwrap();
    } else {
        window.hide().unwrap();
        menu_item.set_title("Show").unwrap();
    }
}

fn check_board_connection() -> bool {
    let hid_api = HidApi::new().expect("Couldn't create HIDApi");
    let connected = hid_api.device_list().any(|dev| {
        if dev.vendor_id() == HID_VENDOR_ID
            && dev.product_id() == HID_PROD_ID
            && dev.usage() == HID_USAGE
            && dev.usage_page() == HID_USAGE_PAGE
        {
            true
        } else {
            false
        }
    });

    connected
}
