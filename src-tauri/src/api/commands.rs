use tauri::Window;

use crate::{models::audio_session::AudioSession, volume_manager, window_manager};

#[tauri::command]
pub fn get_all_sessions() -> Vec<AudioSession> {
    return volume_manager::get_all_sessions();
}

// #[tauri::command]
// pub fn get_sessions(session_name: &str) -> Option<AudioSession> {
//     return volume_manager::get_sessions(session_name);
// }

#[tauri::command]
pub fn get_session(session_name: &str) -> Option<AudioSession> {
    return volume_manager::get_sessions(session_name).into_iter().next();
}

#[tauri::command]
pub fn get_session_volume(session_name: &str) -> i32 {
    return volume_manager::get_session_volume(session_name);
}

#[tauri::command]
pub fn set_session_volume(session_name: &str, volume: i32) -> i32 {
    return volume_manager::set_session_volume(session_name, volume);
}

#[tauri::command]
pub fn toggle_session_mute(session_name: &str) -> bool {
    return volume_manager::toggle_session_mute(session_name);
}

// TODO: Unused
#[tauri::command]
pub fn apply_aero_theme(window: Window) {
    window_manager::apply_aero_theme(window);
}
