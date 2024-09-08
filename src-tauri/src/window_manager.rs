use tauri::PhysicalPosition;
use tauri::Window;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Dwm::DwmExtendFrameIntoClientArea;
use windows::Win32::UI::Controls::MARGINS;

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
