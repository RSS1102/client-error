use tauri::{
    menu::{CheckMenuItemBuilder, MenuBuilder, MenuItem, MenuItemBuilder, SubmenuBuilder}, App, AppHandle, Emitter
};

/// 窗口菜单
pub fn window_menu(app: &mut AppHandle) -> Result<(), tauri::Error> {
    let lang_str: String = "en".to_string();
    // let is_show_toolbar = get_current_show_toolbar();

    // let language = Arc::new(Mutex::new(Language::new()));
    // let language = language.lock().unwrap();

    let new_file = MenuItem::with_id(app, "new_file", "New File", true, Some("Ctrl+N")).unwrap();
    // let open_file = MenuItem::with_id(
    //     handle,
    //     "open_file",
    //     language.open_file.get_lang(&lang_str),
    //     true,
    //     Some("Ctrl+O"),
    // )
    // .unwrap();
    // let save = MenuItem::with_id(
    //     handle,
    //     "save",
    //     language.save.get_lang(&lang_str),
    //     true,
    //     Some("Ctrl+S"),
    // )
    // .unwrap();
    // let save_as = MenuItem::with_id(
    //     handle,
    //     "save_as",
    //     language.save_as.get_lang(&lang_str),
    //     true,
    //     Some("Ctrl+Shift+S"),
    // )
    // .unwrap();
    // let quit = MenuItem::with_id(
    //     handle,
    //     "quit",
    //     language.quit.get_lang(&lang_str),
    //     true,
    //     Some("Ctrl+Q"),
    // )
    // .unwrap();

    let file_menu = SubmenuBuilder::with_id(app, "file_menu", "File")
        .item(&new_file)
        .build()?;

    let language_sub_en = CheckMenuItemBuilder::with_id("en", "English")
        .checked(&lang_str == "en")
        .build(app)?;

    let language_sub_zh = CheckMenuItemBuilder::new("中文")
        .id("zh")
        .checked(&lang_str == "zh");

    let language_menu = MenuItemBuilder::new("Language")
        .id("language_menu")
        // .item(&language_sub_en)
        // .item(&language_sub_zh.build(handle)?)
        .build(app)?;


    let menu = MenuBuilder::new(app)
        .items(&[&file_menu, &language_menu])
        // .item(&setting_menu.build()?)
        .build()?;

    app.set_menu(menu)?;
    // let handle_clone = handle.clone();
    // let language_clone = language.clone();

    app.on_menu_event(
        move |app_handle: &tauri::AppHandle, event| match event.id().0.as_str() {
            "language_menu" => {
                println!("language_menu {}",event.id().0.as_str());
                file_menu.set_text("1111").expect("set text");
            }
            "toggle_toolbar" => {
                // if let Ok(menu) = create_window_menu(
                //     &handle_clone,
                //     language_clone.clone(),
                //     State {
                //         lang_str: get_current_lang(),
                //         is_show_toolbar: get_current_show_toolbar(),
                //     },
                // ) {
                //     let _ = app_handle.set_menu(menu);
                // }
                let _ = app_handle.emit("toggle_toolbar", "");
            }
            "quit" => {
                app_handle.exit(0);
            }
            _ => {}
        },
    );
    Ok(())
}
