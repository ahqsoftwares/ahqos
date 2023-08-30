use std::{fs::create_dir_all, env};

#[tauri::command(async)]
pub fn init_dirs() -> Option<()> {
    let programdata = get_programdata()?;

    create_dir_all(&programdata).ok()?;

    Some(())
}


pub fn get_programdata() -> Option<String> {
    #[cfg(not(windows))]
    return Some("/ahqos".into());

    #[cfg(windows)]
    {
        let system_drive = env::var("SystemDrive").ok()?;

        return Some(
            format!("{}\\ProgramData\\ahqos", &system_drive)
        );
    }
}

