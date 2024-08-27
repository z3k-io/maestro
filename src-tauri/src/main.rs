// src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(warnings)]

use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::thread;
use tauri::App;
use tauri::AppHandle;
use tauri::Config;
use tauri::SystemTray;
use tauri::SystemTrayMenu;
use tauri::Window;
use tauri::{GlobalShortcutManager, Manager};

use inputbot::{KeySequence, KeybdKey::*, MouseButton::*};

mod config;
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

        let callback = move |data: String| {
            let mut new_volumes = data.split("|");
            // first half will be volues, second half will be mute status

            // loop over the split data, get the session name from config, and set the volume
            for session_name in &config.inputs {
                let current_volume: i32 = *current_volumes.get(session_name).unwrap();
                let new_volume: i32 = new_volumes.next().unwrap_or("-2").parse::<i32>().unwrap();

                // In case the volume is the same, skip
                if current_volume == new_volume {
                    continue;
                }

                current_volumes.insert(session_name.clone(), new_volume);

                // TODO: Handle -1 = mute
                // TODO: Handle session not currently found in the system (i.e. not open in Windows)
                if new_volume == -1 {
                    volume_manager::toggle_session_mute(session_name);
                    continue;
                }

                volume_manager::set_session_volume(session_name, new_volume);

                window.show().unwrap();
                window.emit("volume-change", format!("{}:{}", session_name, new_volume));
            }
        };

        if let Err(e) = serial::read_continuous(callback) {
            eprintln!("Error reading from serial port: {}", e);
        }
    });
}

fn override_media_keys(window: Window) {
    let window = Arc::new(window); // Wrap the window in Arc for shared ownership

    let window_clone_for_up = Arc::clone(&window);
    let window_clone_for_down = Arc::clone(&window);
    let window_clone_for_mute = Arc::clone(&window);

    println!("Initializing media key listners");

    VolumeUpKey.block_bind(move || {
        println!("MEDIA KEY: Volume Up");
        let current_vol = volume_manager::get_session_volume("master");
        let updated_vol = volume_manager::set_session_volume("master", current_vol + 2);

        let payload = format!("{}:{}", "master", updated_vol);
        window_clone_for_up.show();
        window_clone_for_up.emit("volume-change", payload);
    });

    VolumeDownKey.block_bind(move || {
        println!("MEDIA KEY: Volume down");
        let current_vol = volume_manager::get_session_volume("master");
        let updated_vol = volume_manager::set_session_volume("master", current_vol - 2);

        let payload = format!("{}:{}", "master", updated_vol);
        window_clone_for_down.show();
        window_clone_for_down.emit("volume-change", payload);
    });
    VolumeMuteKey.block_bind(move || {
        println!("MEDIA KEY: Mute");
        let mute = volume_manager::toggle_session_mute("master");

        let payload = format!("{}:{}", "master", mute);
        window_clone_for_mute.show();
        window_clone_for_mute.emit("mute-change", payload);
    });

    thread::spawn(move || {
        inputbot::handle_input_events();
    });
}

fn main() {
    let config = config::get_config();

    let tray_menu = SystemTrayMenu::new();
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .setup(|app: &mut App| {
            let window: Window = app.get_window("main").unwrap();
            let handle: AppHandle = app.handle();

            window_manager::center_window_at_top(&window);

            override_media_keys(window.clone());

            read_continuous_serial(window);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            volume_manager::get_session_volume,
            volume_manager::set_session_volume,
            volume_manager::master_volume_up,
            volume_manager::master_volume_down,
            volume_manager::toggle_session_mute,
            window_manager::apply_aero_theme,
        ])
        .system_tray(system_tray)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
