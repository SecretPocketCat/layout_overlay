use crossbeam::channel::{unbounded, Receiver, Sender};
use once_cell::sync::Lazy;
use std::ptr;
use winapi::shared::minwindef::{DWORD, LPARAM, UINT, WPARAM};
use winapi::shared::ntdef::LONG;
use winapi::shared::windef::{HWINEVENTHOOK, HWND, POINT};
use winapi::um::winuser::{
    DispatchMessageW, GetMessageW, SetWinEventHook, TranslateMessage, UnhookWinEvent,
    EVENT_SYSTEM_FOREGROUND, MSG, WINEVENT_OUTOFCONTEXT, WM_QUIT,
};

static CHANNEL: Lazy<(Sender<Event>, Receiver<Event>)> = Lazy::new(unbounded);

#[derive(Debug)]
pub enum Event {
    WindowFocus(WindowInfo),
}

#[derive(Debug)]
pub struct WindowInfo {
    exe_path: Option<String>,
    title: Option<String>,
}

pub fn listen() {
    std::thread::spawn(|| loop {
        let ev = CHANNEL.1.recv().unwrap();
    });

    unsafe {
        std::thread::spawn(move || {
            let unhook = vec![SetWinEventHook(
                EVENT_SYSTEM_FOREGROUND,
                EVENT_SYSTEM_FOREGROUND,
                ptr::null_mut(),
                Some(win_event_hook_callback),
                0,
                0,
                WINEVENT_OUTOFCONTEXT,
            )];

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

            for u in unhook.into_iter() {
                UnhookWinEvent(u);
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
    _event_thread: DWORD,
    _event_time: DWORD,
) {
    println!("ev {event}");

    match event {
        EVENT_SYSTEM_FOREGROUND => {
            let title = windows_win::raw::window::get_text(h_wnd).ok();
            let mut exe_path = None;
            let (process_id, _) = windows_win::raw::window::get_thread_process_id(h_wnd);
            if let Ok(process_handle) = windows_win::Process::open(process_id, 0x0400) {
                exe_path =
                    windows_win::raw::process::get_exe_path(process_handle.into_inner()).ok();
            }

            CHANNEL
                .0
                .send(Event::WindowFocus(WindowInfo { title, exe_path }))
                .unwrap();
        }
        _ => unimplemented!("{event}"),
    }
}
