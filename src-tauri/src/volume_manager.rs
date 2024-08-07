use windows::core::Interface;
use windows::Win32::Foundation::*;
use windows::Win32::Media::Audio::*;
use windows::Win32::System::Com::*;
use windows::Win32::System::ProcessStatus::*;
use windows::Win32::System::Threading::*;

#[tauri::command]
pub fn get_process_volume(process_name: &str) -> Result<u32, String> {
    print!("GET {} volume -> ", process_name);

    // If process name doesn't end in .exe, append it
    let process_name = if !process_name.ends_with(".exe") {
        format!("{}.exe", process_name)
    } else {
        process_name.to_string()
    };

    let sessions = enumerate_audio_sessions().map_err(|e| e.to_string())?;
    for session in sessions {
        if let Ok(name) = get_process_name_from_session(&session) {
            if name.to_lowercase() == process_name.to_lowercase() {
                let volume: Result<u32, String> = get_volume_from_session(&session).map_err(|e| e.to_string());
                println!("{}", volume.clone().unwrap());
                return volume;
            }
        }
    }
    Err("Process not found".into())
}

#[tauri::command]
pub fn set_process_volume(process_name: &str, volume: u32) -> Result<u32, String> {
    println!("SET {} volume -> {}", process_name, volume);

    // If process name doesn't end in .exe, append it
    let process_name = if !process_name.ends_with(".exe") {
        format!("{}.exe", process_name)
    } else {
        process_name.to_string()
    };

    let sessions = enumerate_audio_sessions().map_err(|e| e.to_string())?;
    for session in sessions {
        if let Ok(name) = get_process_name_from_session(&session) {
            if name.to_lowercase() == process_name.to_lowercase() {
                return set_volume_for_session(&session, volume).map_err(|e| e.to_string());
            }
        }
    }
    Err("Process not found".into())
}

fn enumerate_audio_sessions() -> windows::core::Result<Vec<IAudioSessionControl2>> {
    let enumerator: IAudioSessionEnumerator = unsafe {
        let device_enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
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
    let process_handle = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, pid) }?;
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

fn get_volume_from_session(session: &IAudioSessionControl2) -> windows::core::Result<u32> {
    let simple_volume: ISimpleAudioVolume = session.cast()?;
    let volume = unsafe { simple_volume.GetMasterVolume()? };
    Ok((volume * 100.0).round() as u32)
}

fn set_volume_for_session(session: &IAudioSessionControl2, volume: u32) -> windows::core::Result<u32> {
    let float_volume: f32 = volume as f32 / 100.0;
    let float_volume = float_volume.max(0.0).min(1.0);

    let simple_volume: ISimpleAudioVolume = session.cast()?;
    unsafe { simple_volume.SetMasterVolume(float_volume, std::ptr::null())? }
    Ok(volume)
}
