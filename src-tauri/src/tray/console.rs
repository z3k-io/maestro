use std::sync::{Arc, Mutex};
use tauri::{AppHandle, WebviewWindow};

use crate::services::window_service;

pub struct Console {
    window: Arc<Mutex<Option<WebviewWindow>>>,
}

impl Console {
    pub fn new() -> Self {
        Console {
            window: Arc::new(Mutex::new(None)),
        }
    }

    pub fn open(&self, app: &AppHandle) {
        let mut window = self.window.lock().unwrap();

        if window.is_none() {
            let new_window = window_service::create_config_editor(app.clone());
            *window = Some(new_window);
        }

        if let Some(w) = window.as_ref() {
            w.show().unwrap();
            w.set_focus().unwrap();
        }
    }
}
