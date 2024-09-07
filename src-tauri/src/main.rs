#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use events::AppEvent;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tauri::App;
use tauri::CustomMenuItem;
use tauri::Manager;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::SystemTrayMenu;
use tauri::Window;
use tauri::WindowBuilder;

mod config;
mod event_listeners;
mod events;
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
                window
                    .emit(AppEvent::VolumeChange.as_str(), format!("{}:{}", &session.name, new_volume))
                    .unwrap();
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

fn emit_initial_volumes(window: Window) {
    let sessions = volume_manager::get_all_sessions();

    for session in &sessions {
        window
            .emit(AppEvent::VolumeChange.as_str(), format!("{}:{}", session.name, session.volume))
            .unwrap();
    }
}

fn main() {
    logger::init_logger();

    let open_console = CustomMenuItem::new("show_logs".to_string(), "Open Logs");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(open_console).add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu).with_tooltip("Mix Monkey 🍌");

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event({
            move |app, event| match event {
                SystemTrayEvent::LeftClick { .. } => {
                    log::info!("Left click on tray icon");
                    toggle_window(app);
                }
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "show_logs" => {
                        log::info!("Opening log file");
                        logger::open_log_file();
                    }
                    _ => {}
                },
                _ => {}
            }
        })
        .setup(|app: &mut App| {
            log::info!("Tauri app is starting...");

            let window = app.get_window("overlay").unwrap();

            window_manager::center_window_at_top(&window);

            event_listeners::override_media_keys(window.clone());

            read_continuous_serial(window.clone());

            emit_initial_volumes(window.clone());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            volume_manager::get_session_volume,
            volume_manager::set_session_volume,
            volume_manager::toggle_session_mute,
            volume_manager::get_all_sessions,
            window_manager::apply_aero_theme,
            blur_window,
            logger::log,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn toggle_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_window("mixer_window") {
        log::info!("Toggling window");
        let is_visible = window.is_visible().unwrap();
        window.emit(AppEvent::MixerVisibilityChange.as_str(), !is_visible).unwrap();
    } else {
        log::info!("Creating new window");
        create_new_window(app);
    }
}

fn create_new_window(app: &tauri::AppHandle) {
    let mixer_window = WindowBuilder::new(app, "mixer_window", tauri::WindowUrl::App("index-mixer.html".into()))
        .title("Mixer")
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .resizable(true)
        .focused(true)
        .visible(false)
        .build()
        .expect("Failed to create new window");

    let last_focus_time = Arc::new(Mutex::new(Instant::now()));
    let mixer_window_clone = mixer_window.clone();
    let mixer_window_clone2 = mixer_window.clone();

    // in 50ms, show the window
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(50));
        mixer_window_clone2.emit(AppEvent::MixerVisibilityChange.as_str(), true).unwrap();
    });

    mixer_window.on_window_event(move |event| match event {
        tauri::WindowEvent::Focused(is_focused) => {
            if *is_focused {
                *last_focus_time.lock().unwrap() = Instant::now();
            } else {
                let last_time = *last_focus_time.lock().unwrap();
                if last_time.elapsed() > Duration::from_millis(100) {
                    mixer_window_clone.emit(AppEvent::MixerVisibilityChange.as_str(), false).unwrap();
                }
            }
        }
        _ => {}
    });
}
