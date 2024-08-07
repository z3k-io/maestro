// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::AppHandle;
use tauri::PhysicalPosition;
use tauri::SystemTray;
use tauri::SystemTrayMenu;
use tauri::Window;
use tauri::{GlobalShortcutManager, Manager};
use windows::core::Interface;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Dwm::DwmExtendFrameIntoClientArea;
use windows::Win32::Media::Audio::*;
use windows::Win32::System::Com::*;
use windows::Win32::System::ProcessStatus::*;
use windows::Win32::System::Threading::*;
use windows::Win32::UI::Controls::MARGINS;

#[tauri::command]
fn get_process_volume(process_name: &str) -> Result<u32, String> {
    println!("Get {} volume", process_name);

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
                println!("Volume: {}", volume.clone().unwrap());
                return volume;
            }
        }
    }
    Err("Process not found".into())
}

#[tauri::command]
fn set_process_volume(process_name: &str, volume: u32) -> Result<u32, String> {
    println!("Setting {} volume -> {}", process_name, volume);

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

#[tauri::command]
fn apply_aero_theme(window: Window) {
    if let Ok(hwnd) = window.hwnd() {
        let hwnd = HWND(hwnd.0 as *mut _);
        unsafe {
            let margins = MARGINS {
                cxLeftWidth: -1,
                cxRightWidth: -1,
                cyTopHeight: -1,
                cyBottomHeight: -1,
            };
            DwmExtendFrameIntoClientArea(hwnd, &margins).expect("Failed to apply Aero Glass effect");
        }
    }
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

fn center_window_at_top(window: &Window) {
    // Get the primary monitor size
    if let Some(monitor) = window.primary_monitor().expect("Failed to get primary monitor") {
        let screen_size = monitor.size();
        let window_size = window.outer_size().expect("Failed to get window size");

        // Calculate the center position
        let x = (screen_size.width - window_size.width) / 2;
        let y = 30; // px from top of the screen

        window
            .set_position(PhysicalPosition {
                x: x as i32,
                y: y as i32,
            })
            .expect("Failed to set window position");
    }
}

fn main() {
    let tray_menu = SystemTrayMenu::new();
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let handle: AppHandle = app.handle();

            center_window_at_top(&window);

            let running_up = Arc::new(Mutex::new(false));
            let running_down = Arc::new(Mutex::new(false));

            // Clone the window handle for use in both closures
            let window_clone_for_up = window.clone();
            let window_clone_for_down = window.clone();

            // Register global shortcuts for volume up and down
            match handle.global_shortcut_manager().register("VolumeUp", move || {
                println!("VolumeUp Key Pressed");

                let current_vol = get_process_volume("chrome").unwrap();
                let updated_vol = set_process_volume("chrome", current_vol + 2).unwrap();

                window_clone_for_up.show().unwrap();
                window_clone_for_up.emit("volume-change", updated_vol).unwrap();
            }) {
                Ok(_) => println!("VolumeUp shortcut registered successfully"),
                Err(e) => eprintln!("Failed to register VolumeUp shortcut: {}", e),
            }

            match handle.global_shortcut_manager().register("VolumeDown", move || {
                println!("VolumeDown Key Pressed");

                let current_vol = get_process_volume("chrome").unwrap();
                let updated_vol = set_process_volume("chrome", current_vol - 2).unwrap();

                window_clone_for_down.show().unwrap();
                window_clone_for_down.emit("volume-change", updated_vol).unwrap();
            }) {
                Ok(_) => println!("VolumeDown shortcut registered successfully"),
                Err(e) => eprintln!("Failed to register VolumeDown shortcut: {}", e),
            }

            #[cfg(debug_assertions)]
            // app.get_window("main").unwrap().open_devtools();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_process_volume,
            set_process_volume,
            apply_aero_theme
        ])
        .system_tray(system_tray)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
