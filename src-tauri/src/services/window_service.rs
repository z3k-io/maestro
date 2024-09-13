use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

use crate::api::events;

pub fn create_overlay(app: AppHandle) -> WebviewWindow {
    let window = WebviewWindowBuilder::new(&app, "overlay", WebviewUrl::App("index-overlay.html".into()))
        .title("Overlay")
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .resizable(false)
        .focused(false)
        .visible(false)
        .transparent(true)
        .build()
        .expect("Failed to create new window");

    window
}

pub fn create_mixer(app: AppHandle) -> WebviewWindow {
    let window = WebviewWindowBuilder::new(&app, "mixer", WebviewUrl::App("index-mixer.html".into()))
        .title("Mixer")
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .resizable(false)
        .focused(true)
        .visible(false)
        .build()
        .expect("Failed to create new window");

    let last_focus_time = Arc::new(Mutex::new(Instant::now()));

    let window_clone = window.clone();
    let last_focus_time_clone = Arc::clone(&last_focus_time);
    let app_handle = app.clone();
    window.on_window_event(move |event| match event {
        tauri::WindowEvent::Focused(is_focused) => {
            if *is_focused {
                *last_focus_time_clone.lock().unwrap() = Instant::now();
            } else {
                let last_time = *last_focus_time_clone.lock().unwrap();
                if last_time.elapsed() > Duration::from_millis(100) {
                    let window_clone = window_clone.clone();
                    let app_handle = app_handle.clone();
                    tauri::async_runtime::spawn(async move {
                        thread::sleep(Duration::from_millis(50));
                        if window_clone.is_visible().unwrap() {
                            log::debug!("Window focus lost -> closing");
                            window_clone.hide().unwrap();
                            // Notify the tray that the window was just hidden
                            app_handle.emit("window_hidden", ()).unwrap();
                        }
                    });
                }
            }
        }
        _ => {}
    });

    return window;
}

pub fn create_config_editor(app: AppHandle) -> WebviewWindow {
    let window = WebviewWindowBuilder::new(&app, "config", WebviewUrl::App("index-config-editor.html".into()))
        .title("Mix Monkey | Config Editor")
        .decorations(true)
        .always_on_top(false)
        .skip_taskbar(false)
        .resizable(true)
        .focused(true)
        .visible(false)
        .build()
        .expect("Failed to create new window");

    return window;
}

pub fn show_config_editor(app: AppHandle) {
    let window = app.get_webview_window("config").expect("Failed to find config editor window");
    window.show().unwrap();
    window.set_focus().unwrap();
}

pub fn hide_config_editor(app: AppHandle) {
    let window = app.get_webview_window("config").expect("Failed to find config editor window");
    window.hide().unwrap();
}

pub fn show_overlay(app: AppHandle) {
    let window = app.get_webview_window("overlay").expect("Failed to find overlay window");

    window.show().unwrap();
    window.set_focus().unwrap();
}

pub fn hide_overlay(app: AppHandle) {
    let window = app.get_webview_window("overlay").expect("Failed to find overlay window");
    window.hide().unwrap();
}

pub fn show_mixer(app: AppHandle) {
    let window = app.get_webview_window("mixer").expect("Failed to find mixer window");
    events::emit_mixer_visibility_change_event(true, app);
    window.show().unwrap();
    window.set_focus().unwrap();
}

pub fn hide_mixer(app: AppHandle) {
    let window = app.get_webview_window("mixer").expect("Failed to find mixer window");
    window.hide().unwrap();
    events::emit_mixer_visibility_change_event(false, app);
}

pub fn toggle_mixer(app: AppHandle) {
    let window = app.get_webview_window("mixer").expect("Failed to find mixer window");
    if window.is_visible().unwrap() {
        hide_mixer(app);
    } else {
        show_mixer(app);
    }
}

pub fn get_window(app: AppHandle, window_label: &str) -> Option<WebviewWindow> {
    app.get_webview_window(window_label)
}
