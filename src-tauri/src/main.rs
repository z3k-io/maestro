// src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use tauri::App;
use tauri::CustomMenuItem;
use tauri::Manager;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::SystemTrayMenu;
use tauri::Window;

use inputbot::KeybdKey::*;

mod config;
mod logger;
mod serial;
mod volume_manager;
mod window_manager;

fn read_continuous_serial(window: Window) -> () {
    thread::spawn(move || {
        let config = config::get_config();

        let mut current_volumes: HashMap<String, i32> = HashMap::new();

        for session_name in &config.inputs {
            current_volumes.insert(session_name.clone(), 0);
        }

        let on_serial_update_callback = move |data: String| {
            let mut new_volumes = data.split("|");

            // loop over the split data, get the session name from config, and set the volume / mute status
            for session_name in &config.inputs {
                let current_volume: i32 = *current_volumes.get(session_name).unwrap();
                let new_volume: i32 = new_volumes
                    .next()
                    .unwrap_or_else(|| {
                        log::error!("Error parsing serial data: {}", data);
                        "0"
                    })
                    .parse::<i32>()
                    .unwrap();

                if current_volume == new_volume {
                    continue;
                }

                // if volume is negative, session is muted
                if new_volume < 0 {
                    volume_manager::set_session_mute(session_name, true);
                } else {
                    volume_manager::set_session_mute(session_name, false);
                }

                current_volumes.insert(session_name.clone(), new_volume);

                volume_manager::set_session_volume(session_name, new_volume.abs());

                window.show().unwrap();
                window.emit("volume-change", format!("{}:{}", session_name, new_volume)).unwrap();
            }
        };

        if let Err(e) = serial::read_continuous(on_serial_update_callback) {
            log::info!("Error reading from serial port: {}", e);
        }
    });
}

fn override_media_keys(window: Window) {
    let window = Arc::new(window); // Wrap the window in Arc for shared ownership

    let window_clone_for_up = Arc::clone(&window);
    let window_clone_for_down = Arc::clone(&window);
    let window_clone_for_mute = Arc::clone(&window);

    log::info!("Initializing media key listners");

    VolumeUpKey.block_bind(move || {
        log::debug!("[MEDIA KEY] Volume Up");
        let current_vol = volume_manager::get_session_volume("master");
        let updated_vol = volume_manager::set_session_volume("master", current_vol + 2);

        let payload = format!("{}:{}", "master", updated_vol);
        window_clone_for_up.show().unwrap();
        window_clone_for_up.emit("volume-change", payload).unwrap();
    });

    VolumeDownKey.block_bind(move || {
        log::debug!("[MEDIA KEY] Volume down");
        let current_vol = volume_manager::get_session_volume("master");
        let updated_vol = volume_manager::set_session_volume("master", current_vol - 2);

        let payload = format!("{}:{}", "master", updated_vol);
        window_clone_for_down.show().unwrap();
        window_clone_for_down.emit("volume-change", payload).unwrap();
    });
    VolumeMuteKey.block_bind(move || {
        log::debug!("[MEDIA KEY] Mute");
        let mute = volume_manager::toggle_session_mute("master");

        let payload = format!("{}:{}", "master", mute);
        window_clone_for_mute.show().unwrap();
        window_clone_for_mute.emit("mute-change", payload).unwrap();
    });

    thread::spawn(move || {
        inputbot::handle_input_events();
    });
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

            override_media_keys(window.clone());

            read_continuous_serial(window);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            volume_manager::get_session_volume,
            volume_manager::set_session_volume,
            volume_manager::toggle_session_mute,
            window_manager::apply_aero_theme,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
