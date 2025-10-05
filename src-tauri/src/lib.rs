use services::{com_service, window_service};
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
    pub mod com_service;
    pub mod icon_service;
    pub mod volume_service;
    pub mod window_service;
}
mod models {
    pub mod audio_session;
}

pub fn run() {
    logger::init();

    log::info!("Mix Monkey v{}", env!("CARGO_PKG_VERSION"));

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();

            #[allow(dead_code)]
            #[cfg(not(debug_assertions))]
            utils::system_manager::handle_enable_autostart(handle.clone());

            utils::system_manager::handle_debug_console(handle.clone());

            window_service::create_overlay(handle.clone());
            window_service::create_mixer(handle.clone());

            system_tray::initialize_tray(handle.clone());

            com_service::listen_serial_input(handle.clone());

            macro_listener::initialize_key_listeners(handle.clone());

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            api::commands::get_all_sessions,
            api::commands::get_session,
            api::commands::get_session_volume,
            api::commands::set_session_volume,
            api::commands::toggle_session_mute,
            api::commands::log,
            api::commands::get_config,
            api::commands::set_config,
            services::window_service::get_taskbar_height
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
