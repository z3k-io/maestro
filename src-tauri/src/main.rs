// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::AppHandle;
use tauri::PhysicalPosition;
use tauri::SystemTray;
use tauri::SystemTrayMenu;
use tauri::Window;
use tauri::{GlobalShortcutManager, Manager};
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Dwm::DwmExtendFrameIntoClientArea;
use windows::Win32::UI::Controls::MARGINS;

mod volume_manager;

#[tauri::command]
fn apply_aero_theme(window: Window) {
    if let Ok(hwnd) = window.hwnd() {
        let hwnd = HWND(hwnd.0 as *mut _);
        unsafe {
            let margins = MARGINS {
                cxLeftWidth: -1,
                cxRightWidth: -1,
                cyTopHeight: -1,
                cyBottomHeight: -1,
            };
            DwmExtendFrameIntoClientArea(hwnd, &margins).expect("Failed to apply Aero Glass effect");
        }
    }
}

fn center_window_at_top(window: &Window) {
    // Get the primary monitor size
    if let Some(monitor) = window.primary_monitor().expect("Failed to get primary monitor") {
        let screen_size = monitor.size();
        let window_size = window.outer_size().expect("Failed to get window size");

        // Calculate the center position
        let x = (screen_size.width - window_size.width) / 2;
        let y = 30; // px from top of the screen

        window
            .set_position(PhysicalPosition {
                x: x as i32,
                y: y as i32,
            })
            .expect("Failed to set window position");
    }
}

fn main() {
    let tray_menu = SystemTrayMenu::new();
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let handle: AppHandle = app.handle();

            center_window_at_top(&window);

            // Clone the window handle for use in both closures
            let window_clone_for_up = window.clone();
            let window_clone_for_down = window.clone();

            // Register global shortcuts for volume up and down
            match handle.global_shortcut_manager().register("VolumeUp", move || {
                println!("VolumeUp Key Pressed");

                let current_vol = volume_manager::get_process_volume("chrome").unwrap();
                let updated_vol = volume_manager::set_process_volume("chrome", current_vol + 2).unwrap();

                window_clone_for_up.show().unwrap();
                window_clone_for_up.emit("volume-change", updated_vol).unwrap();
            }) {
                Ok(_) => println!("VolumeUp shortcut registered successfully"),
                Err(e) => eprintln!("Failed to register VolumeUp shortcut: {}", e),
            }

            match handle.global_shortcut_manager().register("VolumeDown", move || {
                println!("VolumeDown Key Pressed");

                let current_vol = volume_manager::get_process_volume("chrome").unwrap();
                let updated_vol = volume_manager::set_process_volume("chrome", current_vol - 2).unwrap();

                window_clone_for_down.show().unwrap();
                window_clone_for_down.emit("volume-change", updated_vol).unwrap();
            }) {
                Ok(_) => println!("VolumeDown shortcut registered successfully"),
                Err(e) => eprintln!("Failed to register VolumeDown shortcut: {}", e),
            }

            #[cfg(debug_assertions)]
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            volume_manager::get_process_volume,
            volume_manager::set_process_volume,
            apply_aero_theme
        ])
        .system_tray(system_tray)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
