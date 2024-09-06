use std::mem::size_of;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::ProcessStatus::GetProcessImageFileNameW;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use windows::Win32::UI::Shell::{SHGetFileInfoW, SHFILEINFOW, SHGFI_ICON, SHGFI_SMALLICON};
use windows::Win32::UI::WindowsAndMessaging::DestroyIcon;

pub fn get_process_icon(pid: u32) -> Option<String> {
    unsafe {
        let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid);

        if process_handle.is_err() {
            return None;
        }

        let process_handle = process_handle.unwrap();

        let mut buffer = [0u16; 260];
        let path_len = GetProcessImageFileNameW(process_handle, &mut buffer);
        CloseHandle(process_handle);

        if path_len == 0 {
            return None;
        }

        let path = String::from_utf16_lossy(&buffer[..path_len as usize]);
        let mut file_info: SHFILEINFOW = SHFILEINFOW::default();
        let result = SHGetFileInfoW(
            windows::core::PCWSTR::from_raw(path.as_ptr() as *const u16),
            windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES(0),
            Some(&mut file_info),
            size_of::<SHFILEINFOW>() as u32,
            SHGFI_ICON | SHGFI_SMALLICON,
        );

        if result == 0 || file_info.hIcon.is_invalid() {
            return None;
        }

        DestroyIcon(file_info.hIcon);

        // For now, just return the path as a placeholder
        Some(path)
    }
}
