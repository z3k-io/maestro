use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;

use tauri::PhysicalPosition;
use tauri::Window;
use tauri::WindowBuilder;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Dwm::DwmExtendFrameIntoClientArea;
use windows::Win32::UI::Controls::MARGINS;

use crate::api;

pub fn apply_aero_theme(window: Window) {
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

pub fn center_window_at_top(window: &Window) {
    // Get the primary monitor size
    if let Some(monitor) = window.primary_monitor().expect("Failed to get primary monitor") {
        let screen_size = monitor.size();
        let window_size = window.outer_size().expect("Failed to get window size");

        // Calculate the center position
        let x = (screen_size.width - window_size.width) / 2;
        let y = 30; // px from top of the screen

        window
            .set_position(PhysicalPosition { x: x as i32, y: y as i32 })
            .expect("Failed to set window position");
    }
}

pub fn create_new_window(app: &tauri::AppHandle) {
    let mixer_window = WindowBuilder::new(app, "mixer_window", tauri::WindowUrl::App("index-mixer.html".into()))
        .title("Mixer")
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .resizable(true)
        .focused(true)
        .visible(false)
        .build()
        .expect("Failed to create new window");

    let last_focus_time = Arc::new(Mutex::new(Instant::now()));
    let mixer_window_clone = mixer_window.clone();
    let mixer_window_clone2 = mixer_window.clone();

    // in 50ms, show the window
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(50));
        api::events::emit_mixer_visibility_change_event(true, &mixer_window_clone2);
    });

    mixer_window.on_window_event(move |event| match event {
        tauri::WindowEvent::Focused(is_focused) => {
            if *is_focused {
                *last_focus_time.lock().unwrap() = Instant::now();
            } else {
                let last_time = *last_focus_time.lock().unwrap();
                if last_time.elapsed() > Duration::from_millis(100) {
                    api::events::emit_mixer_visibility_change_event(false, &mixer_window_clone);
                }
            }
        }
        _ => {}
    });
}
