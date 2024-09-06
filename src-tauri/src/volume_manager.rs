use std::{collections::HashMap, i32::MIN};

use windows_volume_control::{AudioController, CoinitMode};

use crate::config;

fn get_audio_controller() -> AudioController {
    unsafe {
        let mut controller = AudioController::init(Some(CoinitMode::ApartmentThreaded));
        controller.load_current_sessions();
        return controller;
    }
}

#[tauri::command]
pub fn get_all_sessions() -> HashMap<String, i32> {
    unsafe {
        let controller = get_audio_controller();
        let sessions = controller.get_all_sessions();

        let mut session_volumes: HashMap<String, i32> = HashMap::new();
        for session in sessions {
            let mut volume = (session.get_volume() * 100.0).round() as i32;
            let mute = session.get_mute();
            if mute {
                volume = volume * -1;
            }
            session_volumes.insert(session.get_name().to_string(), volume);
        }

        return session_volumes;
    }
}

#[tauri::command]
pub fn get_session_volume(session_name: &str) -> i32 {
    unsafe {
        let controller = get_audio_controller();
        let session = controller.get_session_with_name(session_name.to_string());

        if session.is_none() {
            log::warn!("Get Volume: No Session Found: {}", session_name);
            return MIN;
        }

        return (session.unwrap().get_volume() * 100.0).round() as i32;
    }
}

#[tauri::command]
pub fn set_session_volume(session_name: &str, volume: i32) -> i32 {
    if volume < 0 {
        log::error!("Volume must be between 0 and 100");
        return 0;
    }
    if volume > 100 {
        log::error!("Volume must be between 0 and 100");
        return 100;
    }

    let new_volume = volume as f32 / 100.0;

    unsafe {
        let controller = get_audio_controller();
        let mut sessions;

        if session_name.to_lowercase() == "other" {
            sessions = controller.get_all_sessions();
            sessions.retain(|session| !config::get_defined_session_names().contains(&session.get_name().to_lowercase()));
        } else {
            sessions = controller.get_all_sessions_with_name(session_name.to_string());
        }

        if sessions.is_empty() {
            log::warn!("Set Volume: No Session Found: {}", session_name);
            return MIN;
        }

        for session in sessions {
            log::info!("Setting {} volume -> {}", session.get_name(), volume);
            session.set_volume(new_volume);
        }
    }

    return volume;
}

#[tauri::command]
pub fn get_session_mute(session_name: &str) -> bool {
    unsafe {
        let controller = get_audio_controller();
        let session = controller.get_session_with_name(session_name.to_string());

        if session.is_none() {
            log::error!("Get Mute: No Session Found: {}", session_name);
            return false;
        }

        return session.unwrap().get_mute();
    }
}

#[tauri::command]
pub fn set_session_mute(session_name: &str, mute: bool) -> bool {
    unsafe {
        let controller = get_audio_controller();
        let mut sessions;

        if session_name.to_lowercase() == "other" {
            sessions = controller.get_all_sessions();
            sessions.retain(|session| !config::get_defined_session_names().contains(&session.get_name().to_lowercase()));
        } else {
            sessions = controller.get_all_sessions_with_name(session_name.to_string());
        }

        if sessions.is_empty() {
            log::warn!("Set Mute: No Session Found: {}", session_name);
            return false;
        }

        for session in sessions {
            session.set_mute(mute);
            log::info!("Setting {} mute -> {}", session.get_name(), mute);
        }

        return mute;
    }
}

#[tauri::command]
pub fn toggle_session_mute(session_name: &str) -> bool {
    log::info!("TOGGLE MUTE: {}", session_name);
    let mute = get_session_mute(session_name);
    return set_session_mute(session_name, !mute);
}
