use base64::{engine::general_purpose, Engine as _};
use log::{error, warn};
use std::mem::size_of;
use windows::core::PWSTR;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::Graphics::Gdi::{
    BitBlt, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, GetDIBits, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB,
    DIB_RGB_COLORS, SRCCOPY,
};
use windows::Win32::System::Threading::{OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_FORMAT, PROCESS_QUERY_LIMITED_INFORMATION};
use windows::Win32::UI::Shell::{SHGetFileInfoW, SHFILEINFOW, SHGFI_ICON, SHGFI_SMALLICON};
use windows::Win32::UI::WindowsAndMessaging::{DestroyIcon, GetIconInfo, ICONINFO};

pub fn get_process_icon(pid: u32) -> Option<String> {
    if pid == 0 {
        // This is the 'master' volume, return None for now
        return None;
    }

    unsafe {
        let process_handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid);

        if let Err(e) = process_handle {
            error!("Failed to open process (PID: {}): {:?}", pid, e);
            return None;
        }

        let process_handle = process_handle.unwrap();

        let mut buffer = [0u16; 260];
        let mut size = buffer.len() as u32;
        let result = QueryFullProcessImageNameW(
            process_handle,
            PROCESS_NAME_FORMAT(0),
            PWSTR::from_raw(buffer.as_mut_ptr()),
            &mut size,
        );
        CloseHandle(process_handle).unwrap();

        if !result.is_ok() {
            error!("Failed to get process image file name for PID: {}", pid);
            return None;
        }

        let path = String::from_utf16_lossy(&buffer[..size as usize]);
        warn!("Process path for PID {}: {}", pid, path);

        let mut file_info: SHFILEINFOW = SHFILEINFOW::default();
        let result = SHGetFileInfoW(
            windows::core::PCWSTR::from_raw(path.as_ptr() as *const u16),
            windows::Win32::Storage::FileSystem::FILE_ATTRIBUTE_NORMAL,
            Some(&mut file_info),
            std::mem::size_of::<SHFILEINFOW>() as u32,
            SHGFI_ICON | SHGFI_SMALLICON,
        );

        if result == 0 || file_info.hIcon.is_invalid() {
            error!("Failed to get file info or invalid icon handle for path: {}", path);
            return None;
        }

        let icon_data = extract_icon_data(file_info.hIcon);
        DestroyIcon(file_info.hIcon).unwrap();

        icon_data.map(|data| general_purpose::STANDARD.encode(data))
    }
}

unsafe fn extract_icon_data(h_icon: windows::Win32::UI::WindowsAndMessaging::HICON) -> Option<Vec<u8>> {
    let mut icon_info: ICONINFO = ICONINFO::default();
    if GetIconInfo(h_icon, &mut icon_info).is_err() {
        error!("Failed to get icon info");
        return None;
    }

    let hdc = GetDC(None);
    let hdc_mem = CreateCompatibleDC(hdc);

    let old_obj = SelectObject(hdc_mem, icon_info.hbmColor);

    let mut bmi: BITMAPINFO = BITMAPINFO::default();
    bmi.bmiHeader.biSize = size_of::<BITMAPINFOHEADER>() as u32;
    bmi.bmiHeader.biWidth = 16;
    bmi.bmiHeader.biHeight = -16;
    bmi.bmiHeader.biPlanes = 1;
    bmi.bmiHeader.biBitCount = 32;
    bmi.bmiHeader.biCompression = BI_RGB.0;

    let mut bits: Vec<u8> = vec![0; 16 * 16 * 4];

    if BitBlt(hdc_mem, 0, 0, 16, 16, hdc, 0, 0, SRCCOPY).is_ok() {
        let result = GetDIBits(
            hdc_mem,
            icon_info.hbmColor,
            0,
            16,
            Some(bits.as_mut_ptr() as *mut std::ffi::c_void),
            &mut bmi,
            DIB_RGB_COLORS,
        );
        if result == 0 {
            error!("Failed to get DIB bits");
            return None;
        }
    } else {
        error!("Failed to perform BitBlt");
        return None;
    }

    SelectObject(hdc_mem, old_obj);
    DeleteDC(hdc_mem).unwrap();
    DeleteObject(icon_info.hbmColor).unwrap();
    DeleteObject(icon_info.hbmMask).unwrap();

    Some(bits)
}
