use tauri::menu::{MenuBuilder, MenuItem, MenuItemBuilder, SubmenuBuilder};

/// 窗口菜单
pub fn window_menu(app: &mut tauri::App) -> Result<(), tauri::Error> {
    let new_file = MenuItem::with_id(app, "new_file", "New File", true, Some("Ctrl+N")).unwrap();

    let file_menu = SubmenuBuilder::with_id(app, "file_menu", "File")
        .item(&new_file)
        .build()?;

    let change_language_menu = MenuItemBuilder::new("Language")
        .id("change_language_menu")
        .build(app)?;

    let menu = MenuBuilder::new(app)
        .items(&[&file_menu, &change_language_menu])
        .build()?;

    app.set_menu(menu)?;

    app.on_menu_event(
        move |_app_handle: &tauri::AppHandle, event| match event.id().0.as_str() {
            "change_language_menu" => {
                println!("change_language_menu {}", event.id().0.as_str());
                file_menu.set_text("1111").expect("set text");
            }
            _ => {}
        },
    );
    Ok(())
}
