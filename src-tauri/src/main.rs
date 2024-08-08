// src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(warnings)]

use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::thread;
use tauri::Window;

use tauri::AppHandle;
use tauri::SystemTray;
use tauri::SystemTrayMenu;
use tauri::{GlobalShortcutManager, Manager};
mod serial;
mod volume_manager;
mod window_manager;
#[derive(Debug, Deserialize)]
struct Config {
    com_port: String,
    baud_rate: u32,
    inputs: Vec<String>,
}

fn read_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = serde_yaml::from_str(&contents)?;

    Ok(config)
}

fn read_continuous_serial(config: Config, window: Window) -> () {
    thread::spawn(move || {
        println!("Starting serial read on port: {}", config.com_port);

        let callback = move |data: String| {
            let mut split_data = data.split("|");

            println!("Update -> {}", data);

            // loop over the split data, get the session name from config, and set the volume
            for session_name in &config.inputs {
                let new_volume = split_data.next().unwrap_or("-2").parse::<u32>().unwrap() as f32 / 100.0;
                println!("Handling session: {} -> {}", session_name, new_volume);

                // Get the current volume; if found set to new, otherwise skip
                let current_volume = volume_manager::get_session_volume(session_name);
                if current_volume == -1.0 {
                    println!("Session not found, skipping: {}", session_name);
                    continue;
                }

                if (current_volume != new_volume) {
                    println!("Updating {} volume: {} -> {}", session_name, current_volume, new_volume);
                    volume_manager::set_session_volume(session_name, new_volume);

                    window.show().unwrap();
                    window
                        .emit("volume-change", format!("{}:{}", session_name, new_volume))
                        .unwrap();
                }

                // TODO: Handle -1 = mute
            }
        };

        if let Err(e) = serial::read_continuous(&config.com_port, callback) {
            eprintln!("Error reading from serial port: {}", e);
        }
    });
}

fn main() {
    let config_path = "../config.yaml";
    let config = read_config(&config_path).expect("Failed to read config file");

    let tray_menu = SystemTrayMenu::new();
    let system_tray = SystemTray::new().with_menu(tray_menu);

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

                    let current_vol = volume_manager::get_session_volume("master");
                    let updated_vol = volume_manager::set_session_volume("master", current_vol + 0.02);

                    window_clone_for_up.show().unwrap();
                    window_clone_for_up.emit("volume-change", updated_vol).unwrap();
                })
                .unwrap();
            handle
                .global_shortcut_manager()
                .register("VolumeDown", move || {
                    println!("KEYPRESS: VolumeDown");

                    let current_vol = volume_manager::get_session_volume("master");
                    let updated_vol = volume_manager::set_session_volume("master", current_vol - 0.02);

                    window_clone_for_down.show().unwrap();
                    window_clone_for_down.emit("volume-change", updated_vol).unwrap();
                })
                .unwrap();

            read_continuous_serial(config, window);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            volume_manager::get_session_volume,
            volume_manager::set_session_volume,
            window_manager::apply_aero_theme,
        ])
        .system_tray(system_tray)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
