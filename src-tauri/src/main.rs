// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use utils::base::set_current_show_toolbar;

mod implement;
mod utils;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_show_toolbar(show: bool) -> Result<(), String> {
    set_current_show_toolbar(&show);
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app: &mut tauri::App| {
            let mut shared_app = app.handle().clone();

            let result = implement::windows_menu::window_menu(&mut shared_app);
            if let Err(e) = result {
                eprintln!("设置窗口菜单失败: {}", e);
            }
            // let _ = implement::system_tray::system_tray_menu(app);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_show_toolbar])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
