// src/volume_manager.rs

use colored::Colorize;
use windows_volume_control::AudioController;

fn get_audio_controller() -> AudioController {
    unsafe {
        let mut controller = AudioController::init(None);
        controller.GetSessions();
        controller.GetDefaultAudioEnpointVolumeControl();
        controller.GetAllProcessSessions();
        return controller;
    }
}

fn get_case_sensitive_session_name(session_name: &str) -> String {
    unsafe {
        let controller = get_audio_controller();
        let session_names = controller.get_all_session_names();

        // find session by name case insensitive
        for case_sensitive_session_name in session_names {
            if session_name.to_lowercase() == case_sensitive_session_name.to_lowercase() {
                return case_sensitive_session_name;
            }
        }

        return session_name.to_string();
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
    // TODO: Need special handling for 'other' sessions
    unsafe {
        let controller = get_audio_controller();
        let session_name_cased = get_case_sensitive_session_name(session_name);
        let session = controller.get_session_by_name(session_name_cased.to_string());

        // if session doesn't exist, return // TODO: Need to handle this better
        if session.is_none() {
            log::info!("{}: {}", "Session not found".red(), session_name_cased.red());
            return -2;
        }

        return (session.unwrap().getVolume() * 100.0).round() as i32;
    }
}

#[tauri::command]
pub fn set_session_volume(session_name: &str, volume: i32) -> i32 {
    // TODO: Need special handling for 'other' sessions
    if volume < 0 {
        log::info!("{}", "Volume must be between 0 and 100".red());
        return 0;
    }
    if volume > 100 {
        log::info!("{}", "Volume must be between 0 and 100".red());
        return 100;
    }

    let new_volume = volume as f32 / 100.0;

    unsafe {
        let controller = get_audio_controller();
        let session_name_cased = get_case_sensitive_session_name(session_name);
        let session = controller.get_session_by_name(session_name_cased.to_string());

        if session.is_none() {
            log::info!("{}: {}", "Session not found".red(), session_name.red());
            return -2;
        }

        let current_volume = session.unwrap().getVolume();

        // If increasing volume, disable mute
        if new_volume > current_volume {
            session.unwrap().setMute(false);
        }

        session.unwrap().setVolume(new_volume);

        let message = format!("Setting {} volume -> {}", session_name, volume).green();
        log::info!("{}", message);
    }

    return volume;
}

#[tauri::command]
pub fn get_session_mute(session_name: &str) -> bool {
    unsafe {
        let controller = get_audio_controller();
        let session = controller.get_session_by_name(session_name.to_string());

        return session.unwrap().getMute();
    }
}

#[tauri::command]
pub fn set_session_mute(session_name: &str, mute: bool) -> bool {
    unsafe {
        let controller = get_audio_controller();
        let session = controller.get_session_by_name(session_name.to_string());

        session.unwrap().setMute(mute);

        let message = format!("Setting {} mute -> {}", session_name, mute).green();
        log::info!("{}", message);

        return session.unwrap().getMute();
    }
}

#[tauri::command]
pub fn toggle_session_mute(session_name: &str) -> bool {
    log::info!("TOGGLE MUTE: {}", session_name);
    let mute = get_session_mute(session_name);
    return set_session_mute(session_name, !mute);
}
