// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::mpsc;

use hotkeys::register_global_hotkey;
use tauri::{
    CustomMenuItem, Manager, PhysicalPosition, PhysicalSize, Runtime, SystemTray, SystemTrayMenu,
    SystemTrayMenuItem,
};
use window_shadows::set_shadow;
use window_vibrancy::apply_acrylic;

pub mod hotkeys;

#[tauri::command]
async fn search_input<R: Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
    value: String,
) -> Result<(), String> {
    println!("Value: {value}");
    Ok(())
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            tauri::SystemTrayEvent::DoubleClick { .. } => {
                let window = app.get_window("main").unwrap();
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                }
            }
            tauri::SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => std::process::exit(0),
                _ => {}
            },
            _ => {}
        })
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            let screen = window.current_monitor()?.unwrap();
            let screen_size = screen.size();

            // MAX SIZE!!
            // window.set_size(PhysicalSize {
            // height: 500,
            // width: 500,
            // })?;

            window.set_size(PhysicalSize {
                height: 500,
                width: 500,
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

            std::thread::spawn(|| {
                unsafe { register_global_hotkey(window) };
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![search_input])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
