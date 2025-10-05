use std::sync::Arc;
use std::time::Duration;

use tauri::AppHandle;
use tauri::Event;
use tauri::Listener;
use windows_key_listener::KeyListener;

use crate::api::events;
use crate::api::events::AppEvent;
use crate::config;
use crate::config::Config;
use crate::services::volume_service;
use crate::services::window_service;

const DEBOUNCE_INTERVAL: Duration = Duration::from_millis(25);

pub fn initialize_key_listeners(app_handle: AppHandle) {
    let key_listener = KeyListener::new();

    register_key_listeners(app_handle.clone(), &key_listener, config::get_config());

    app_handle.listen(AppEvent::ConfigChange.as_str(), {
        let app_handle = app_handle.clone();
        move |event: Event| {
            if let Ok(config) = serde_json::from_str::<Config>(event.payload()) {
                log::info!("Config changed, resetting key listeners");
                let new_listener = KeyListener::new();
                register_key_listeners(app_handle.clone(), &new_listener, config);
            }
        }
    });
}

fn register_key_listeners(app_handle: AppHandle, key_listener: &KeyListener, config: Config) {
    if let Some(hotkey) = config.mixer.hotkey {
        log::info!("Registering mixer hotkey: {}", hotkey);
        if let Err(e) = key_listener.listen(
            &hotkey,
            DEBOUNCE_INTERVAL,
            Arc::new({
                let app_handle = app_handle.clone();
                move || {
                    window_service::toggle_mixer(app_handle.clone());
                    true
                }
            }),
        ) {
            log::error!("Failed to register mixer hotkey: {}", e);
        }
    }

    if let Err(e) = key_listener.listen(
        "VolumeUp",
        DEBOUNCE_INTERVAL,
        Arc::new({
            let app_handle = app_handle.clone();
            move || {
                handle_session_up("master", app_handle.clone());
                true
            }
        }),
    ) {
        log::error!("Failed to register VolumeUp hotkey: {}", e);
    }

    if let Err(e) = key_listener.listen(
        "VolumeDown",
        DEBOUNCE_INTERVAL,
        Arc::new({
            let app_handle = app_handle.clone();
            move || {
                handle_session_down("master", app_handle.clone());
                true
            }
        }),
    ) {
        log::error!("Failed to register VolumeDown hotkey: {}", e);
    }

    if let Err(e) = key_listener.listen(
        "VolumeMute",
        DEBOUNCE_INTERVAL,
        Arc::new({
            let app_handle = app_handle.clone();
            move || {
                handle_session_toggle_mute("master", app_handle.clone());
                true
            }
        }),
    ) {
        log::error!("Failed to register VolumeMute hotkey: {}", e);
    }
}

fn handle_session_toggle_mute(session_name: &str, app_handle: AppHandle) {
    let session = volume_service::toggle_session_mute(session_name);

    events::emit_volume_change_event(&session, app_handle);
}

fn handle_session_up(session_name: &str, app_handle: AppHandle) {
    let current_vol = volume_service::get_session_volume(session_name);
    let session = volume_service::set_session_volume(session_name, current_vol + 2).unwrap();

    events::emit_volume_change_event(&session, app_handle);
}

fn handle_session_down(session_name: &str, app_handle: AppHandle) {
    let current_vol = volume_service::get_session_volume(session_name);
    let session = volume_service::set_session_volume(session_name, current_vol - 2).unwrap();

    events::emit_volume_change_event(&session, app_handle);
}
