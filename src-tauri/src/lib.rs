use services::window_service;
use tray::system_tray;
use utils::{logger, macro_listener};

pub mod config;
mod tray {
    pub mod system_tray;
}
mod utils {
    pub mod logger;
    pub mod macro_listener;
    pub mod system_manager;
}
mod api {
    pub mod commands;
    pub mod events;
}
mod services {
    pub mod icon_service;
    pub mod volume_service;
    pub mod window_service;
}
mod models {
    pub mod audio_session;
}

pub fn run() {
    logger::init();

    log::info!("Volare v{}", env!("CARGO_PKG_VERSION"));

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();

            // Initialize autostart plugin
            #[cfg(desktop)]
            {
                use tauri_plugin_autostart::MacosLauncher;
                let _ = app.handle().plugin(tauri_plugin_autostart::init(
                    MacosLauncher::LaunchAgent,
                    Some(vec![]), // No additional arguments needed
                ));
            }

            utils::system_manager::handle_debug_console(handle.clone());

            window_service::create_overlay(handle.clone());
            window_service::create_mixer(handle.clone());

            system_tray::initialize_tray(handle.clone());


            macro_listener::initialize_key_listeners(handle.clone());

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .invoke_handler(tauri::generate_handler![
            api::commands::get_all_sessions,
            api::commands::get_session,
            api::commands::get_session_volume,
            api::commands::set_session_volume,
            api::commands::toggle_session_mute,
            api::commands::log,
            api::commands::get_config,
            api::commands::set_config,
            api::commands::enable_autostart,
            api::commands::disable_autostart,
            api::commands::is_autostart_enabled,
            services::window_service::get_taskbar_height
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
