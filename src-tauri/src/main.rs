#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use hidapi::HidApi;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
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
const LAYER_EV: &str = "layer";
const CAPS_WORD_EV: &str = "capsword";
const HID_VENDOR_ID: u16 = 0x3a3c;
const HID_PROD_ID: u16 = 0x0001;
const HID_USAGE: u16 = 0x61;
const HID_USAGE_PAGE: u16 = 0xFF60;

#[derive(Debug)]
enum BoardEvent {
    Connection(bool),
    Layer(u8),
    CapsWord(bool),
}

fn main() {
    let (ev_sender, ev_receiver) = channel::<BoardEvent>();

    thread::spawn(move || {
        run_hid_loop(ev_sender.clone());
    });

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
                    toggle_overlay(app, None);
                }
                _ => {}
            },
            _ => {}
        })
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            // events
            let app_handle = app.handle();
            app.listen_global("toggle", move |_| {
                toggle_overlay(&app_handle.clone(), None);
            });

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
                    BoardEvent::CapsWord(on) => {
                        app_handle.emit_all(CAPS_WORD_EV, on).unwrap();
                    }
                };
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

fn run_hid_loop(sender: Sender<BoardEvent>) {
    let mut buff = [0; 2];
    loop {
        let mut hid_api = HidApi::new().expect("Couldn't create HIDApi");
        let mut hid_device = None;
        while hid_device.is_none() {
            hid_device = hid_api.device_list().find_map(|dev| {
                if dev.vendor_id() == HID_VENDOR_ID
                    && dev.product_id() == HID_PROD_ID
                    && dev.usage() == HID_USAGE
                    && dev.usage_page() == HID_USAGE_PAGE
                {
                    let dev = Some(
                        hid_api
                            .open_path(dev.path())
                            .expect("Couldn't connect to HID device"),
                    );
                    sender.send(BoardEvent::Connection(true)).unwrap();
                    dev
                } else {
                    None
                }
            });

            if hid_device.is_none() {
                hid_api.refresh_devices().unwrap();
                sleep(Duration::from_millis(1000));
            }
        }

        let hid_device = hid_device.unwrap();

        loop {
            match hid_device.read(&mut buff) {
                Ok(_) => {
                    println!("{:?}", buff);
                    let ev_type = buff[0];
                    let ev_val = buff[1];
                    match ev_type {
                        1 => sender.send(BoardEvent::Layer(ev_val)).unwrap(),
                        2 => sender.send(BoardEvent::CapsWord(ev_val != 0)).unwrap(),
                        _ => unimplemented!("Board event '{}' not implemended", ev_type),
                    };
                }
                Err(e) => {
                    eprintln!("HID read error: {}", e);
                    break;
                }
            }
        }

        sender.send(BoardEvent::Connection(false)).unwrap();
    }
}
