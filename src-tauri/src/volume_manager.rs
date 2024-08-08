// src/volume_manager.rs

use colored::Colorize;
use windows::core::Interface;
use windows::Win32::Foundation::*;
use windows::Win32::Media::Audio::MMDeviceEnumerator;
use windows::Win32::Media::Audio::*;
use windows::Win32::System::Com::*;
use windows::Win32::System::ProcessStatus::*;
use windows::Win32::System::Threading::*;
use windows_volume_control::AudioController;

#[tauri::command]
pub fn get_master_volume() -> f32 {
    return get_session_volume("master");
}

#[tauri::command]
pub fn set_master_volume(volume: f32) {
    set_session_volume("master", volume);
}

#[tauri::command]
pub fn get_session_volume(session_name: &str) -> f32 {
    // TODO: Need special handling for 'other' sessions
    unsafe {
        let mut controller = AudioController::init(None);
        controller.GetSessions();
        controller.GetDefaultAudioEnpointVolumeControl();
        controller.GetAllProcessSessions();
        let test = controller.get_all_session_names();
        let session = controller.get_session_by_name(session_name.to_string());

        // if session doesn't exist, return
        if session.is_none() {
            println!("{}: {}", "Session not found".red(), session_name.red());
            return -1.0;
        }

        return (session.unwrap().getVolume() * 100.0).round() / 100.0;
    }
}

#[tauri::command]
pub fn set_session_volume(session_name: &str, volume: f32) -> f32 {
    // TODO: Need special handling for 'other' sessions
    if volume < 0.0 || volume > 1.0 {
        panic!("{}", "Volume must be between 0.0 and 1.0".red());
    }

    unsafe {
        let mut controller = AudioController::init(None);
        controller.GetSessions();
        controller.GetDefaultAudioEnpointVolumeControl();
        controller.GetAllProcessSessions();
        let test = controller.get_all_session_names();
        let session = controller.get_session_by_name(session_name.to_string());

        // if session doesn't exist, return
        if session.is_none() {
            println!("{}: {}", "Session not found".red(), session_name.red());
            return -1.0;
        }

        session.unwrap().setVolume(volume);

        println!(
            "{} {} {}",
            session_name.green(),
            "volume set to,".green(),
            volume.to_string().green()
        );

        return (volume * 100.0).round() / 100.0;
    }
}
