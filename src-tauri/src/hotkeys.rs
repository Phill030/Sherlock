use std::ptr;
use tauri::{App, Window};
use winapi::um::{
    errhandlingapi::GetLastError,
    winuser::{
        DispatchMessageW, PeekMessageW, RegisterHotKey, TranslateMessage, MOD_NOREPEAT, MOD_WIN,
        MSG, PM_REMOVE, WM_HOTKEY,
    },
};

pub unsafe fn register_global_hotkey(window: Window) {
    let modifiers: u32 = (MOD_NOREPEAT | MOD_WIN).try_into().unwrap(); // No repeat when key is held + Windows key
    let vk: u32 = 0x59; // S

    let registered = RegisterHotKey(ptr::null_mut(), 1, modifiers, vk);

    if registered != 1 {
        panic!("failed to register hotkey: {}", GetLastError());
    }

    let mut msg = MSG {
        hwnd: ptr::null_mut(),
        message: 0,
        wParam: 0,
        lParam: 0,
        time: 0,
        pt: std::mem::zeroed(),
    };

    loop {
        let peek_value = PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, PM_REMOVE);
        if peek_value != 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);

            if msg.message.eq(&WM_HOTKEY) {
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                    window.emit("open_window", {}).unwrap();
                }
            }
        }
    }
}
