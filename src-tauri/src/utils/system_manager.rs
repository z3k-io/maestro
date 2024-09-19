#![allow(dead_code)]

use crate::config::{get_config, Config};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Listener};
use windows::Win32::Foundation::HWND;
use windows::Win32::System::Console::{AllocConsole, AttachConsole, FreeConsole, GetConsoleWindow, ATTACH_PARENT_PROCESS};

pub fn toggle_startup(enable: bool) -> Result<(), String> {
    log::info!("Set autostart: {}", enable);

    let startup_folder = get_startup_folder()?;
    let shortcut_path = startup_folder.join("Mix Monkey.lnk");

    log::info!("Startup folder: {:?}", startup_folder);
    log::info!("Shortcut path: {:?}", shortcut_path);

    let shortcut_exists = shortcut_path.exists();

    if enable && !shortcut_exists {
        create_shortcut(&shortcut_path)?;
        log::info!("Shortcut created successfully");
    } else if !enable && shortcut_exists {
        fs::remove_file(&shortcut_path).map_err(|e| e.to_string())?;
        log::info!("Shortcut removed successfully");
    } else {
        log::info!("No change needed. Shortcut exists: {}, Enable: {}", shortcut_exists, enable);
    }

    Ok(())
}

fn get_startup_folder() -> Result<PathBuf, String> {
    let startup_folder = dirs::config_dir()
        .ok_or_else(|| "Failed to get config directory".to_string())?
        .join("Microsoft")
        .join("Windows")
        .join("Start Menu")
        .join("Programs")
        .join("Startup");

    Ok(startup_folder)
}

fn create_shortcut(shortcut_path: &PathBuf) -> Result<(), String> {
    let exe_path = std::env::current_exe().map_err(|e| e.to_string())?;
    let working_dir = exe_path.parent().ok_or_else(|| "Failed to get parent directory".to_string())?;

    // Use PowerShell to create a shortcut with the correct working directory
    let script = format!(
        "$WshShell = New-Object -comObject WScript.Shell; \
         $Shortcut = $WshShell.CreateShortcut('{}'); \
         $Shortcut.TargetPath = '{}'; \
         $Shortcut.WorkingDirectory = '{}'; \
         $Shortcut.Save()",
        shortcut_path.to_str().unwrap(),
        exe_path.to_str().unwrap(),
        working_dir.to_str().unwrap()
    );

    log::warn!("Script: {}", script);

    let output = std::process::Command::new("powershell")
        .arg("-Command")
        .arg(&script)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(())
}

pub fn handle_enable_autostart(app_handle: AppHandle) {
    let config = get_config();

    if config.system.autostart {
        toggle_startup(true).unwrap();
    }

    app_handle.listen("config_changed", move |event| {
        if let Ok(config) = serde_json::from_str::<Config>(event.payload()) {
            if config.system.autostart {
                toggle_startup(true).unwrap();
            } else {
                toggle_startup(false).unwrap();
            }
        }
    });
}

pub fn handle_debug_console(app_handle: AppHandle) {
    unsafe fn set_debug_console(config: Config) {
        if config.system.show_console {
            if GetConsoleWindow() == HWND(std::ptr::null_mut()) {
                log::info!("Console not attached, attaching");
                let result = AttachConsole(ATTACH_PARENT_PROCESS);
                if result.is_err() {
                    AllocConsole().expect("Failed to allocate console");
                } else {
                    log::info!("Console attached successfully");
                }
            } else {
                log::info!("Console already attached");
            }
        } else {
            FreeConsole().expect("Failed to free console");
        }
    }

    let config = get_config();

    unsafe {
        set_debug_console(config);
    }

    app_handle.listen("config_changed", move |event| {
        if let Ok(config) = serde_json::from_str::<Config>(event.payload()) {
            unsafe {
                set_debug_console(config);
            }
        }
    });
}
