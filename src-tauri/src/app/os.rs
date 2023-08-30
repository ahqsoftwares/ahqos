#[tauri::command(async)]
pub fn os() -> String {
    #[cfg(windows)]
    return "Win32".into();

    #[cfg(target_os = "linux")]
    return "Linux".into();

    #[cfg(target_os = "macos")]
    return "Macos".into();

    #[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
    return "Unknown".into();
}