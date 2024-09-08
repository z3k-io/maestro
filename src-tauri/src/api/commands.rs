use tauri::{AppHandle, Manager, Window};

use crate::{models::audio_session::AudioSession, volume_manager, window_manager};

use super::events;

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
pub fn toggle_session_mute(app_handle: AppHandle, session_name: &str) -> bool {
    let muted = volume_manager::toggle_session_mute(session_name);

    // Get the updated session after toggling mute
    if let Some(updated_session) = volume_manager::get_sessions(session_name).into_iter().next() {
        // Emit an event to all windows
        app_handle.windows().iter().for_each(|(_, window)| {
            events::emit_volume_change_event(&updated_session, window);
        });
    }

    muted
}

// TODO: Unused
#[tauri::command]
pub fn apply_aero_theme(window: Window) {
    window_manager::apply_aero_theme(window);
}
