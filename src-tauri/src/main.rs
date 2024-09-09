#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use api::event_listeners;
use global_hotkey::hotkey::HotKey;
use global_hotkey::GlobalHotKeyManager;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use tauri::App;
use tauri::CustomMenuItem;
use tauri::Manager;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::SystemTrayMenu;
use tauri::Window;
use winit::event_loop::{ControlFlow, EventLoop};

mod config;
mod serial;
mod volume_manager;
mod window_manager;
mod api {
    pub mod commands;
    pub mod event_listeners;
    pub mod events;
}

mod utils {
    pub mod icon_service;
    pub mod keyboard;
    pub mod logger;
}

mod models {
    pub mod audio_session;
}

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
                let current_volume: i32 = *current_volumes.get(&session.name).unwrap();
                let new_volume: i32 = new_volume.parse::<i32>().unwrap();

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

                let audio_sessions = volume_manager::get_sessions(&session.name);
                for audio_session in audio_sessions {
                    api::events::emit_volume_change_event(&audio_session, &window);
                }

                window.show().unwrap();
            }
        };

        if let Err(e) = serial::read_continuous(on_serial_update_callback) {
            log::info!("Error reading from serial port: {}", e);
        }
    });
}

fn main() {
    utils::logger::init_logger();

    let (tx, rx) = mpsc::channel();

    // Start tauri on a new thread
    thread::spawn(move || {
        init_tauri(tx);
    });

    let hotkeys_manager = GlobalHotKeyManager::new().unwrap();
    let event_loop = EventLoop::builder().build().unwrap();

    #[allow(deprecated)]
    event_loop
        .run(move |_event, event_loop| {
            event_loop.set_control_flow(ControlFlow::Poll);

            if let Ok(hotkey) = rx.try_recv() {
                hotkeys_manager.register(hotkey).unwrap();
                log::debug!("Registered new hotkey: {:?}", hotkey);
            }
        })
        .unwrap();
}

fn init_tauri(tx: mpsc::Sender<HotKey>) {
    let open_console = CustomMenuItem::new("show_logs".to_string(), "Open Logs");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(open_console).add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu).with_tooltip("Mix Monkey 🍌");

    tauri::Builder::default()
        .any_thread()
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
                        utils::logger::open_log_file();
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

            event_listeners::initialize(tx, window.clone(), app.handle().clone());

            read_continuous_serial(window.clone());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            api::commands::get_session_volume,
            api::commands::set_session_volume,
            api::commands::toggle_session_mute,
            api::commands::get_session,
            api::commands::get_all_sessions,
            api::commands::apply_aero_theme,
            utils::logger::log,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn toggle_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_window("mixer_window") {
        log::info!("Toggling window");
        let is_visible = window.is_visible().unwrap();
        api::events::emit_mixer_visibility_change_event(!is_visible, &window);
    } else {
        log::info!("Creating new window");
        window_manager::create_new_window(app);
    }
}
