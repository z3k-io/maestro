use tauri::Manager;
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

#[tauri::command]
pub fn toggle_startup(app_handle: tauri::AppHandle, enable: bool) -> Result<(), String> {
    let exe_dir = app_handle.path().executable_dir().expect("failed to get executable dir");
    let app_exe = exe_dir.join("mix-monkey.exe").to_string_lossy().into_owned();

    log::info!("app_exe: {}", app_exe);

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"Software\Microsoft\Windows\CurrentVersion\Run";
    let (key, _) = hkcu.create_subkey(path).map_err(|e| e.to_string())?;

    if enable {
        key.set_value("Mix Monkey", &app_exe).map_err(|e| e.to_string())?;
    } else {
        key.delete_value("Mix Monkey").map_err(|e| e.to_string())?;
    }

    Ok(())
}
