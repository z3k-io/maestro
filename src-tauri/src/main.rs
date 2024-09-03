// src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::thread;
use tauri::App;
use tauri::CustomMenuItem;
use tauri::Manager;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::SystemTrayMenu;
use tauri::Window;

mod config;
mod event_listeners;
mod logger;
mod serial;
mod volume_manager;
mod window_manager;

fn read_continuous_serial(window: Window) -> () {
    thread::spawn(move || {
        let config = config::get_config();

        let mut current_volumes: HashMap<String, i32> = HashMap::new();

        for session in &config.sessions {
            current_volumes.insert(session.name.clone(), 0);
        }

        let on_serial_update_callback = move |data: String| {
            let new_volumes = data.split("|");

            for (index, new_volume) in new_volumes.enumerate() {
                // look up session name from encoder index
                let session = &config.sessions.iter().find(|s| s.encoder == index as u8).unwrap();
                // let session = &config.sessions[index];
                let new_volume: i32 = new_volume.parse::<i32>().unwrap();

                let current_volume: i32 = *current_volumes.get(&session.name).unwrap();

                if current_volume == new_volume {
                    continue;
                }

                // if volume is negative, session is muted
                if new_volume < 0 {
                    volume_manager::set_session_mute(&session.name, true);
                } else {
                    volume_manager::set_session_mute(&session.name, false);
                }

                current_volumes.insert(session.name.clone(), new_volume);

                volume_manager::set_session_volume(&session.name, new_volume.abs());

                window.show().unwrap();
                window.emit("volume-change", format!("{}:{}", &session.name, new_volume)).unwrap();
            }
        };

        if let Err(e) = serial::read_continuous(on_serial_update_callback) {
            log::info!("Error reading from serial port: {}", e);
        }
    });
}

#[tauri::command]
fn blur_window(window: Window) {
    log::info!("Blurring window");
    // window.show().unwrap();
    let focused = window.is_focused().unwrap();
    log::info!("Window focused: {}", focused);

    // TODO: Fix focus state
    // Unfocusing a window seems to be currently unsupported in tauri.
    // Work around might be to create / destroy window on open / close
    // Would still break during open state after focused though

    // window.close().unwrap();
    window.hide().unwrap();
}

fn main() {
    logger::init_logger();

    // Configure system tray menu
    let open_console = CustomMenuItem::new("show_logs".to_string(), "Open Logs");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(open_console).add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(move |app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                if id == "quit" {
                    std::process::exit(0);
                }
                if id == "show_logs" {
                    log::info!("Opening log file");
                    logger::open_log_file();
                }
            }
            _ => {}
        })
        .setup(|app: &mut App| {
            log::info!("Tauri app is starting...");

            let window = app.get_window("main").unwrap();

            window_manager::center_window_at_top(&window);

            event_listeners::override_media_keys(window.clone());

            read_continuous_serial(window);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            volume_manager::get_session_volume,
            volume_manager::set_session_volume,
            volume_manager::toggle_session_mute,
            window_manager::apply_aero_theme,
            blur_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
