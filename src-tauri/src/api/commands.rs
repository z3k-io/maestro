use tauri::AppHandle;

use crate::{
    config::{self, Config},
    models::audio_session::AudioSession,
    services::volume_service,
    utils::logger,
};

use super::events;

#[tauri::command]
pub fn log(message: String, level: &str) {
    logger::log(message, level);
}

#[tauri::command]
pub fn get_all_sessions() -> Vec<AudioSession> {
    return volume_service::get_all_sessions();
}

#[tauri::command]
pub fn get_session(session_name: &str) -> Option<AudioSession> {
    return volume_service::get_sessions(session_name).into_iter().next();
}

#[tauri::command]
pub fn get_session_volume(session_name: &str) -> i32 {
    return volume_service::get_session_volume(session_name);
}

#[tauri::command]
pub fn set_session_volume(session_name: &str, volume: i32) -> AudioSession {
    return volume_service::set_session_volume(session_name, volume).expect("Failed to set volume.");
}

#[tauri::command]
pub fn toggle_session_mute(app_handle: AppHandle, session_name: &str) -> bool {
    let session = volume_service::toggle_session_mute(session_name);

    events::emit_volume_change_event(&session, app_handle);

    return session.mute;
}

#[tauri::command]
pub fn get_config() -> Config {
    return config::get_config();
}

#[tauri::command]
pub fn set_config(config: Config) {
    config::set_config(config);
}
