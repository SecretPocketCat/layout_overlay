use crossbeam::channel::{unbounded, Receiver, Sender};
use once_cell::sync::Lazy;
use std::ffi::c_void;
use std::ptr;
use winapi::shared::hidclass::GUID_DEVINTERFACE_HID;
use winapi::shared::minwindef::{DWORD, LPARAM, UINT, WPARAM};
use winapi::shared::ntdef::LONG;
use winapi::shared::usbiodef::GUID_DEVINTERFACE_USB_DEVICE;
use winapi::shared::windef::{HWINEVENTHOOK, HWND, POINT};
use winapi::um::dbt::{DBT_DEVTYP_DEVICEINTERFACE, DEV_BROADCAST_DEVICEINTERFACE_W};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::{
    CreateWindowExW, DispatchMessageW, GetMessageW, RegisterDeviceNotificationW, SetWinEventHook,
    TranslateMessage, UnhookWinEvent, UnregisterDeviceNotification,
    DEVICE_NOTIFY_ALL_INTERFACE_CLASSES, DEVICE_NOTIFY_WINDOW_HANDLE, EVENT_SYSTEM_FOREGROUND,
    EVENT_SYSTEM_MINIMIZEEND, HWND_MESSAGE, MSG, WINEVENT_OUTOFCONTEXT, WM_DEVICECHANGE,
    WM_INPUT_DEVICE_CHANGE, WM_QUIT,
};

static CHANNEL: Lazy<(Sender<Event>, Receiver<Event>)> = Lazy::new(unbounded);

#[derive(Debug)]
pub enum Event {
    WindowFocus(WindowInfo),
    HidDeviceConnected,
}

#[derive(Debug)]
pub struct WindowInfo {
    exe_path: Option<String>,
    title: Option<String>,
    thread_id: u32,
    process_id: u32,
}

pub fn listen(hwnd_id: isize) {
    std::thread::spawn(|| loop {
        let ev = CHANNEL.1.recv().unwrap();
        println!("{:?}", ev);
    });

    println!("h: {hwnd_id}");

    unsafe {
        std::thread::spawn(move || {
            let hwnd = hwnd_id as *mut isize as HWND;

            loop {
                let mut notif_filter = DEV_BROADCAST_DEVICEINTERFACE_W {
                    dbcc_reserved: 0,
                    dbcc_name: [0],
                    dbcc_size: 0,
                    dbcc_devicetype: DBT_DEVTYP_DEVICEINTERFACE,
                    dbcc_classguid: GUID_DEVINTERFACE_USB_DEVICE,
                };
                notif_filter.dbcc_size = std::mem::size_of_val(&notif_filter) as u32;

                let notif_filter_ptr: *mut c_void = &mut notif_filter as *mut _ as *mut c_void;
                let unregiter_dev_notif = RegisterDeviceNotificationW(
                    hwnd as *mut _,
                    notif_filter_ptr,
                    DEVICE_NOTIFY_ALL_INTERFACE_CLASSES,
                );

                if unregiter_dev_notif.is_null() {
                    panic!("2 fucks");
                }

                let unhook = vec![
                    SetWinEventHook(
                        EVENT_SYSTEM_FOREGROUND,
                        EVENT_SYSTEM_FOREGROUND,
                        ptr::null_mut(),
                        Some(win_event_hook_callback),
                        0,
                        0,
                        WINEVENT_OUTOFCONTEXT,
                    ),
                    SetWinEventHook(
                        WM_DEVICECHANGE,
                        WM_DEVICECHANGE,
                        ptr::null_mut(),
                        Some(win_event_hook_callback),
                        0,
                        0,
                        WINEVENT_OUTOFCONTEXT,
                    ),
                    SetWinEventHook(
                        WM_INPUT_DEVICE_CHANGE,
                        WM_INPUT_DEVICE_CHANGE,
                        ptr::null_mut(),
                        Some(win_event_hook_callback),
                        0,
                        0,
                        WINEVENT_OUTOFCONTEXT,
                    ),
                ];

                let mut msg = MSG {
                    hwnd: 0 as HWND,
                    message: 0 as UINT,
                    wParam: 0 as WPARAM,
                    lParam: 0 as LPARAM,
                    time: 0 as DWORD,
                    pt: POINT { x: 0, y: 0 },
                };

                loop {
                    let pm = GetMessageW(&mut msg, ptr::null_mut(), 0, 0);
                    if pm == 0 {
                        break;
                    }

                    if msg.message == WM_QUIT {
                        break;
                    }

                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }

                UnregisterDeviceNotification(unregiter_dev_notif);

                for u in unhook.into_iter() {
                    UnhookWinEvent(u);
                }
            }
        });
    }
}

extern "system" fn win_event_hook_callback(
    _hook: HWINEVENTHOOK,
    event: DWORD,
    h_wnd: HWND,
    _id_object: LONG,
    _id_child: LONG,
    event_thread: DWORD,
    _event_time: DWORD,
) {
    println!("ev {event}");

    match event {
        EVENT_SYSTEM_FOREGROUND | EVENT_SYSTEM_MINIMIZEEND => {
            let title = windows_win::raw::window::get_text(h_wnd).ok();
            let mut exe_path = None;
            let (process_id, _) = windows_win::raw::window::get_thread_process_id(h_wnd);
            if let Ok(process_handle) = windows_win::Process::open(process_id, 0x0400) {
                exe_path =
                    windows_win::raw::process::get_exe_path(process_handle.into_inner()).ok();
            }

            CHANNEL
                .0
                .send(Event::WindowFocus(WindowInfo {
                    thread_id: event_thread,
                    title,
                    process_id,
                    exe_path,
                }))
                .unwrap();
        }
        _ => unimplemented!("{event}"),
    }
}
