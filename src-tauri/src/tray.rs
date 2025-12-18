use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager,
};

pub fn create_tray(app: &AppHandle) -> Result<TrayIcon, tauri::Error> {
    // 创建托盘菜单
    let add_todo = MenuItem::with_id(app, "add_todo", "添加 Todo", true, None::<&str>)?;
    let add_note = MenuItem::with_id(app, "add_note", "添加便签", true, None::<&str>)?;
    let separator = MenuItem::with_id(app, "separator", "---", false, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&add_todo, &add_note, &separator, &settings, &quit])?;

    // 加载图标
    let icon_bytes = include_bytes!("../icons/icon-menubar.png");
    let icon_image = image::load_from_memory(icon_bytes)
        .map_err(|e| tauri::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?
        .to_rgba8();
    let (width, height) = icon_image.dimensions();
    let icon = Image::new(icon_image.as_raw(), width, height);

    // 创建托盘图标
    let tray = TrayIconBuilder::new()
        .icon(icon)
        .icon_as_template(true)
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "add_todo" => {
                app.emit("tray-add-todo", ()).unwrap();
                show_main_window(app);
            }
            "add_note" => {
                app.emit("tray-add-note", ()).unwrap();
                show_main_window(app);
            }
            "settings" => {
                app.emit("tray-settings", ()).unwrap();
                show_main_window(app);
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click { button, .. } = event {
                match button {
                    tauri::tray::MouseButton::Left => {
                        let app = tray.app_handle();
                        show_main_window(app);
                    }
                    _ => {}
                }
            }
        })
        .build(app)?;

    Ok(tray)
}

fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}
