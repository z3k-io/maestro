use tauri::{
    menu::{Menu, MenuItem}, tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent}, AppHandle, Listener, Manager, Wry
};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::time::{Instant, Duration};

use crate::{services::window_service, utils};

static WINDOW_LAST_HIDDEN: Lazy<Mutex<Option<Instant>>> = Lazy::new(|| Mutex::new(None));

pub fn initialize_tray(app_handle: AppHandle<Wry>) {
    let open_logs = MenuItem::with_id(&app_handle, "show_logs", "Open Logs", true, None::<&str>).unwrap();
    let quit = MenuItem::with_id(&app_handle, "quit", "Quit", true, None::<&str>).unwrap();
    let menu = Menu::with_items(&app_handle, &[&open_logs, &quit]).unwrap();

    app_handle.listen("window_hidden", |_| {
        *WINDOW_LAST_HIDDEN.lock().unwrap() = Some(Instant::now());
    });

    let _ = TrayIconBuilder::with_id("tray")
        .tooltip("Mix Monkey 🍌")
        .icon(app_handle.default_window_icon().unwrap().clone())
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "show_logs" => {
                utils::logger::open_log_file();
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Down,
                ..
            } = event
            {
                let app = tray.app_handle();
                let window = app.get_webview_window("mixer").unwrap();
                
                let grace_period = Duration::from_millis(300);
                let should_ignore = WINDOW_LAST_HIDDEN.lock().unwrap()
                    .map_or(false, |last_hidden| last_hidden.elapsed() < grace_period);

                if should_ignore {
                    log::debug!("Ignoring tray click due to recent window hide");
                    return;
                } 
                if window.is_visible().unwrap() {
                    log::debug!("Hiding Volume Mixer window");
                    window_service::hide_mixer(app.clone());
                } else {
                    log::debug!("Showing Volume Mixer window");
                    window_service::hide_overlay(app.clone());
                    window_service::show_mixer(app.clone());
                }
            }
        })
        .build(&app_handle);
}
