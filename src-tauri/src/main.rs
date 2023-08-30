#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
use app::*;

use tauri::{
    api::notification::Notification,
    Manager
};

use std::{process::{self, Command}, env::current_exe};

#[cfg(windows)]
use is_elevated::is_elevated;

fn main() {
    #[cfg(windows)]
    if !is_elevated() {
        if let Ok(exe) = current_exe() {
            if let Some(exe) = exe.to_str() {
                let _ = Notification::new("com.ahqsoftwares.ahqos")
                    .title("Info")
                    .body("Launching app as administrator")
                    .show();

                let _ = Command::new("powershell")
                    .args([
                        "start-process",
                        &format!("\"{}\"", &exe),
                        "-verb runas"
                    ])
                    .spawn();

                process::exit(0);
            }
        }

        let _ = Notification::new("com.ahqsoftwares.ahqos")
            .title("Error")
            .body("We were unable to launch app as administrator, kindly manually launch it")
            .show();

        process::exit(0);
    }

    tauri::Builder::default()
        .setup(|app| {
            if let Some(window) = app.get_window("main") {
                let _ = window.hide();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            os, 
            init_dirs, 
            get_user_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
