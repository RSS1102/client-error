// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod implement;

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

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
