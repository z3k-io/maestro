// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use windows::core::Interface;
use windows::Win32::Foundation::*;
use windows::Win32::Media::Audio::*;
use windows::Win32::System::Com::*;
use windows::Win32::System::ProcessStatus::*;
use windows::Win32::System::Threading::*;

#[tauri::command]
fn get_process_volume(process_name: &str) -> Result<f32, String> {
    println!("Getting volume for process: {}", process_name);

    // If process name doesn't end in .exe, append it
    let process_name = if !process_name.ends_with(".exe") {
        format!("{}.exe", process_name)
    } else {
        process_name.to_string()
    };

    let sessions = enumerate_audio_sessions().map_err(|e| e.to_string())?;
    for session in sessions {
        if let Ok(name) = get_process_name_from_session(&session) {
            println!("Found process: {}", name);
            if name.to_lowercase() == process_name.to_lowercase() {
                return get_volume_from_session(&session).map_err(|e| e.to_string());
            }
        }
    }
    Err("Process not found".into())
}

#[tauri::command]
fn set_process_volume(process_name: &str, volume: f32) -> Result<(), String> {
    println!("Setting volume for process: {} to {}", process_name, volume);

    let sessions = enumerate_audio_sessions().map_err(|e| e.to_string())?;
    for session in sessions {
        if let Ok(name) = get_process_name_from_session(&session) {
            println!("Found process: {}", name);
            if name.to_lowercase() == process_name.to_lowercase() {
                return set_volume_for_session(&session, volume).map_err(|e| e.to_string());
            }
        }
    }
    Err("Process not found".into())
}

fn enumerate_audio_sessions() -> windows::core::Result<Vec<IAudioSessionControl2>> {
    let enumerator: IAudioSessionEnumerator = unsafe {
        let device_enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        let device = device_enumerator.GetDefaultAudioEndpoint(eRender, eMultimedia)?;
        let session_manager: IAudioSessionManager2 = device.Activate(CLSCTX_ALL, None)?;
        session_manager.GetSessionEnumerator()?
    };
    let count = unsafe { enumerator.GetCount()? };
    let mut sessions = Vec::new();
    for i in 0..count {
        let session: IAudioSessionControl = unsafe { enumerator.GetSession(i)? };
        let session2: IAudioSessionControl2 = session.cast()?;
        sessions.push(session2);
    }
    Ok(sessions)
}

fn get_process_name_from_session(session: &IAudioSessionControl2) -> windows::core::Result<String> {
    let pid = unsafe { session.GetProcessId()? };
    let process_handle =
        unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, pid) }?;
    if process_handle.is_invalid() {
        return Err(windows::core::Error::from_win32());
    }

    let mut name = [0u16; 260];
    let len = unsafe { GetModuleBaseNameW(process_handle, None, &mut name) };
    if len == 0 {
        unsafe {
            let _ = CloseHandle(process_handle);
        };
        return Err(windows::core::Error::from_win32());
    }
    unsafe {
        let _ = CloseHandle(process_handle);
    };

    Ok(String::from_utf16_lossy(&name[..len as usize]))
}

fn get_volume_from_session(session: &IAudioSessionControl2) -> windows::core::Result<f32> {
    let simple_volume: ISimpleAudioVolume = session.cast()?;
    unsafe { simple_volume.GetMasterVolume() }
}

fn set_volume_for_session(
    session: &IAudioSessionControl2,
    volume: f32,
) -> windows::core::Result<()> {
    let simple_volume: ISimpleAudioVolume = session.cast()?;
    unsafe { simple_volume.SetMasterVolume(volume, std::ptr::null()) }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_process_volume,
            set_process_volume
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
