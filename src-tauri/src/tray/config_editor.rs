use tauri::AppHandle;

use crate::services::window_service;

pub struct ConfigEditor {}

impl ConfigEditor {
    pub fn new() -> Self {
        ConfigEditor {}
    }

    pub async fn open(&self, app: &AppHandle) {
        let window = window_service::get_window(app.clone(), "config");

        if window.is_none() {
            log::info!("Creating new config editor window");
            let _ = window_service::create_config_editor(app.clone());
        }

        window_service::show_config_editor(app.clone());
    }
}
