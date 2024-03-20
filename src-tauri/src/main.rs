// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rdev::{listen, Event, EventType, Key};
use tauri::{
    CustomMenuItem, Manager, PhysicalPosition, PhysicalSize, SystemTray, SystemTrayMenu,
    SystemTrayMenuItem,
};
use window_shadows::set_shadow;
use window_vibrancy::apply_acrylic;

static mut WINDOWS_PRESSED: bool = false;

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(key) => match key {
            Key::MetaLeft => {
                println!("TEST");
                unsafe { WINDOWS_PRESSED = true }
            }
            Key::KeyY => {
                if unsafe { WINDOWS_PRESSED } {
                    print!("WINDOWS IS PRESSED AND SPACE!!");
                }
            }
            _ => {}
        },
        EventType::KeyRelease(key) => match key {
            Key::MetaLeft => unsafe { WINDOWS_PRESSED = false },
            _ => {}
        },

        _ => {}
    }
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    // This will block.
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }

    tauri::Builder::default()
        .system_tray(system_tray)
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            let screen = window.current_monitor()?.unwrap();
            let screen_size = screen.size();

            window.set_size(PhysicalSize {
                height: 500,
                width: 600,
            })?;

            let window_size = window.outer_size().unwrap();

            window.set_position(PhysicalPosition {
                x: (screen_size.width / 2) - (window_size.width / 2),
                y: (screen_size.height / 2) - (window_size.height / 2),
            })?;

            #[cfg(target_os = "windows")]
            apply_acrylic(&window, Some((0, 0, 0, 10)))?;

            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&window, true).unwrap();

            Ok(())
        })
        // .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
