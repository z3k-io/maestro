use std::sync::Arc;
use std::sync::Mutex;

use tauri::AppHandle;
use tauri::Event;
use tauri::Listener;

use crate::api::events;
use crate::config;
use crate::config::Config;
use crate::services::volume_service;
use crate::services::window_service;

use super::key::Key;
use super::key::KeyChord;
use super::key_listener::KeyListener;

pub fn initialize_key_listeners(app_handle: AppHandle) {
    let key_listener = Arc::new(Mutex::new(KeyListener::new()));

    register_key_listeners(app_handle.clone(), key_listener.clone(), config::get_config());

    app_handle.listen("config_changed", {
        let app_handle = app_handle.clone();
        let key_listener = key_listener.clone();
        move |event: Event| {
            if let Ok(config) = serde_json::from_str::<Config>(event.payload()) {
                log::info!("Config changed, resetting key listeners");
                key_listener.lock().unwrap().dispose();
                key_listener.lock().unwrap().init();
                register_key_listeners(app_handle.clone(), key_listener.clone(), config);
            }
        }
    });
}

fn register_key_listeners(app_handle: AppHandle, key_listener: Arc<Mutex<KeyListener>>, config: Config) {
    if config.mixer.hotkey.is_some() {
        key_listener.lock().unwrap().register(
            KeyChord::from_string(config.mixer.hotkey.unwrap().as_str()),
            true,
            Arc::new({
                let app_handle = app_handle.clone();
                move || {
                    window_service::toggle_mixer(app_handle.clone());
                }
            }),
        );
    }

    key_listener.lock().unwrap().register(
        KeyChord::new(vec![Key::from_name("VolumeUp")]),
        true,
        Arc::new({
            let app_handle = app_handle.clone();
            move || {
                handle_session_up("master", app_handle.clone());
            }
        }),
    );

    key_listener.lock().unwrap().register(
        KeyChord::new(vec![Key::from_name("VolumeDown")]),
        true,
        Arc::new({
            let app_handle = app_handle.clone();
            move || {
                handle_session_down("master", app_handle.clone());
            }
        }),
    );

    key_listener.lock().unwrap().register(
        KeyChord::new(vec![Key::from_name("VolumeMute")]),
        true,
        Arc::new({
            let app_handle = app_handle.clone();
            move || {
                handle_session_toggle_mute("master", app_handle.clone());
            }
        }),
    );
}

fn handle_session_toggle_mute(session_name: &str, app_handle: AppHandle) {
    log::info!("Session toggle mute: {}", session_name);
    let session = volume_service::toggle_session_mute(session_name);

    events::emit_volume_change_event(&session, app_handle);
}

fn handle_session_up(session_name: &str, app_handle: AppHandle) {
    log::info!("Session up: {}", session_name);
    let current_vol = volume_service::get_session_volume(session_name);
    let session = volume_service::set_session_volume(session_name, current_vol + 2).unwrap();

    events::emit_volume_change_event(&session, app_handle);
}

fn handle_session_down(session_name: &str, app_handle: AppHandle) {
    log::info!("Session down: {}", session_name);
    let current_vol = volume_service::get_session_volume(session_name);
    let session = volume_service::set_session_volume(session_name, current_vol - 2).unwrap();

    events::emit_volume_change_event(&session, app_handle);
}
