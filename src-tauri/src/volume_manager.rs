// src/volume_manager.rs

use std::i32::MIN;

use windows_volume_control::AudioController;

use crate::config;

fn get_audio_controller() -> AudioController {
    unsafe {
        let mut controller = AudioController::init(None);
        controller.get_sessions();
        controller.get_default_audio_enpoint_volume_control();
        controller.get_all_process_sessions();
        return controller;
    }
}

#[tauri::command]
pub fn get_master_volume() -> i32 {
    return get_session_volume("master");
}

#[tauri::command]
pub fn set_master_volume(volume: i32) {
    set_session_volume("master", volume);
}

#[tauri::command]
pub fn master_volume_up() -> i32 {
    log::info!("MEDIA KEY: Volume up - UI");
    let volume = get_master_volume();
    let new_volume = volume + 2;
    set_master_volume(new_volume);
    return new_volume;
}

#[tauri::command]
pub fn master_volume_down() -> i32 {
    log::info!("MEDIA KEY: Volume down - UI");
    let volume = get_master_volume();
    let new_volume = volume - 2;
    set_master_volume(new_volume);
    return new_volume;
}

#[tauri::command]
pub fn get_session_volume(session_name: &str) -> i32 {
    // TODO: *Might* Need special handling for 'other' sessions
    unsafe {
        let controller = get_audio_controller();
        let session = controller.get_session_by_name(session_name.to_string());

        if session.is_none() {
            log::warn!("QSession not found: {}", session_name);
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
            sessions = controller.get_all_sessions_by_name(session_name.to_string());
        }

        if sessions.is_empty() {
            log::warn!("RSession not found: {}", session_name);
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
        let session = controller.get_session_by_name(session_name.to_string());

        if session.is_none() {
            log::error!("ZSession not found: {}", session_name);
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
            sessions = controller.get_all_sessions_by_name(session_name.to_string());
        }

        if sessions.is_empty() {
            log::warn!("Session not found: {}", session_name);
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
