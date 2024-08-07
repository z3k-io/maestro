// src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;
use tauri::AppHandle;
use tauri::SystemTray;
use tauri::SystemTrayMenu;
use tauri::{GlobalShortcutManager, Manager};

mod serial;
mod volume_manager;
mod window_manager;

fn main() {
    let tray_menu = SystemTrayMenu::new();
    let system_tray = SystemTray::new().with_menu(tray_menu);

    let port_name = "COM3";

    thread::spawn(move || {
        println!("Starting serial read on port: {}", port_name);
        if let Err(e) = serial::read_from_serial_continuous(&port_name) {
            eprintln!("Error reading from serial port: {}", e);
        }
    });

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let handle: AppHandle = app.handle();

            window_manager::center_window_at_top(&window);

            let window_clone_for_up = window.clone();
            let window_clone_for_down = window.clone();

            handle
                .global_shortcut_manager()
                .register("VolumeUp", move || {
                    println!("KEYPRESS: VolumeUp");

                    let current_vol = volume_manager::get_process_volume("chrome").unwrap();
                    let updated_vol = volume_manager::set_process_volume("chrome", current_vol + 2).unwrap();

                    window_clone_for_up.show().unwrap();
                    window_clone_for_up.emit("volume-change", updated_vol).unwrap();
                })
                .unwrap();
            handle
                .global_shortcut_manager()
                .register("VolumeDown", move || {
                    println!("KEYPRESS: VolumeDown");

                    let current_vol = volume_manager::get_process_volume("chrome").unwrap();
                    let updated_vol = volume_manager::set_process_volume("chrome", current_vol - 2).unwrap();

                    window_clone_for_down.show().unwrap();
                    window_clone_for_down.emit("volume-change", updated_vol).unwrap();
                })
                .unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            volume_manager::get_process_volume,
            volume_manager::set_process_volume,
            window_manager::apply_aero_theme,
        ])
        .system_tray(system_tray)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
