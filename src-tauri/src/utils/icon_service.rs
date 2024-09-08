pub fn get_icon(pid: u32) -> Option<String> {
    let icon = if pid != 0 {
        Some(windows_icons::get_icon_base64_by_process_id(pid))
    } else {
        None
    };
    return icon;
}
