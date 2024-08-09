// src/volume_manager.rs

use std::sync::{Mutex, Once};

use colored::Colorize;
use windows::core::Interface;
use windows::Win32::Foundation::*;
use windows::Win32::Media::Audio::MMDeviceEnumerator;
use windows::Win32::Media::Audio::*;
use windows::Win32::System::Com::*;
use windows::Win32::System::ProcessStatus::*;
use windows::Win32::System::Threading::*;
use windows_volume_control::AudioController;
use windows_volume_control::CoinitMode;

static mut AUDIO_CONTROLLER: Option<Mutex<AudioController>> = None;
static INIT: Once = Once::new();

fn get_audio_controller() -> &'static Mutex<AudioController> {
    unsafe {
        INIT.call_once(|| {
            let controller = AudioController::init(None); // Directly use the returned controller
            AUDIO_CONTROLLER = Some(Mutex::new(controller));
        });
        AUDIO_CONTROLLER.as_ref().expect("AudioController not initialized")
    }
}

// TODO: Handle mute states

#[tauri::command]
pub fn get_master_volume() -> f32 {
    return get_session_volume("master");
}

#[tauri::command]
pub fn set_master_volume(volume: f32) {
    set_session_volume("master", volume);
}

#[tauri::command]
pub fn master_volume_up() -> f32 {
    println!("Volume up");
    let volume = get_master_volume();
    let new_volume = volume + 0.02;
    set_master_volume(new_volume);
    return new_volume;
}

#[tauri::command]
pub fn master_volume_down() -> f32 {
    println!("Volume down");
    let volume = get_master_volume();
    let new_volume = volume - 0.02;
    set_master_volume(new_volume);
    return new_volume;
}

#[tauri::command]
pub fn get_session_volume(session_name: &str) -> f32 {
    // TODO: Need special handling for 'other' sessions
    unsafe {
        let mut controller = get_audio_controller().lock().unwrap();
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
    if volume < 0.0 {
        println!("{}", "Volume must be between 0.0 and 1.0".red());
        return 0.0;
    }
    if volume > 1.0 {
        println!("{}", "Volume must be between 0.0 and 1.0".red());
        return 1.0;
    }

    unsafe {
        let mut controller = get_audio_controller().lock().unwrap();
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

        let message = format!("Setting {} volume -> {}", session_name, volume).green();
        println!("{}", message);
    }

    return (volume * 100.0).round() / 100.0;
}
