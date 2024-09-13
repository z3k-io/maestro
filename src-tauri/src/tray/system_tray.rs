use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tauri::image::Image;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Listener, Manager, Wry,
};

use crate::{services::window_service, utils};

use super::console::Console;

static WINDOW_LAST_HIDDEN: Lazy<Mutex<Option<Instant>>> = Lazy::new(|| Mutex::new(None));

pub fn initialize_tray(app_handle: AppHandle<Wry>) {
    let open_logs = MenuItem::with_id(&app_handle, "show_logs", "Logs", true, None::<&str>).unwrap();
    let console = MenuItem::with_id(&app_handle, "open_console", "Open Console", true, None::<&str>).unwrap();
    let quit = MenuItem::with_id(&app_handle, "quit", "Quit", true, None::<&str>).unwrap();
    let config = MenuItem::with_id(&app_handle, "config", "Edit Config", true, None::<&str>).unwrap();

    let menu = Menu::with_items(&app_handle, &[&open_logs, &console, &config, &quit]).unwrap();

    app_handle.listen("window_hidden", |_| {
        *WINDOW_LAST_HIDDEN.lock().unwrap() = Some(Instant::now());
    });

    let image = Image::from_path("icons/speaker-32.png").unwrap();

    let _ = TrayIconBuilder::with_id("tray")
        .tooltip("Mix Monkey 🍌")
        .icon(image)
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "show_logs" => {
                log::info!("Opening logs");
                utils::logger::open_log_file();
            }
            "open_console" => {
                log::info!("Opening console");
                let console = Console::new();
                console.open(app);
            }
            "config" => {
                log::info!("Opening config editor");
                let window = window_service::get_window(app.clone(), "config");

                if window.is_none() {
                    log::info!("Creating new config editor window");
                    let _ = window_service::create_config_editor(app.clone());
                }

                window_service::show_config_editor(app.clone());
            }
            "quit" => {
                log::info!("Quitting Mix Monkey");
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
                let should_ignore = WINDOW_LAST_HIDDEN
                    .lock()
                    .unwrap()
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
