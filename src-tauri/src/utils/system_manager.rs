#![allow(dead_code)]

use crate::api::events::AppEvent;
use crate::config::{get_config, Config};
use tauri::{AppHandle, Listener};
use windows::Win32::Foundation::HWND;
use windows::Win32::System::Console::{AllocConsole, AttachConsole, FreeConsole, GetConsoleWindow, ATTACH_PARENT_PROCESS};


pub fn handle_enable_autostart(app_handle: AppHandle) {
    let config = get_config();

    // Use the official Tauri autostart plugin
    if config.system.autostart {
        use tauri_plugin_autostart::ManagerExt;
        let autostart_manager = app_handle.autolaunch();
        if let Err(e) = autostart_manager.enable() {
            log::error!("Failed to enable autostart: {}", e);
        }
    }

    app_handle.clone().listen(AppEvent::ConfigChange.as_str(), move |event| {
        if let Ok(config) = serde_json::from_str::<Config>(event.payload()) {
            use tauri_plugin_autostart::ManagerExt;
            let autostart_manager = app_handle.autolaunch();
            
            if config.system.autostart {
                if let Err(e) = autostart_manager.enable() {
                    log::error!("Failed to enable autostart: {}", e);
                }
            } else {
                if let Err(e) = autostart_manager.disable() {
                    log::error!("Failed to disable autostart: {}", e);
                }
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
                    if let Err(e) = AllocConsole() {
                        log::warn!("Failed to allocate console: {:?}", e);
                    } else {
                        log::info!("Console allocated successfully");
                    }
                } else {
                    log::info!("Console attached successfully");
                }
            } else {
                log::info!("Console already attached");
            }
        } else {
            if let Err(e) = FreeConsole() {
                log::warn!("Failed to free console: {:?}", e);
            }
        }
    }

    let config = get_config();

    unsafe {
        set_debug_console(config);
    }

    app_handle.listen(AppEvent::ConfigChange.as_str(), move |event| {
        if let Ok(config) = serde_json::from_str::<Config>(event.payload()) {
            unsafe {
                set_debug_console(config);
            }
        }
    });
}
