// 抑制 macOS cocoa crate 的弃用警告（功能正常，未来可迁移到 objc2）
#![cfg_attr(target_os = "macos", allow(deprecated))]
#![cfg_attr(target_os = "macos", allow(unexpected_cfgs))]

use tauri::{
    image::Image,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, PhysicalPosition,
};

// 抑制 cocoa crate 的弃用警告（功能正常，未来可迁移到 objc2）
#[cfg(target_os = "macos")]
use cocoa::base::id;
#[cfg(target_os = "macos")]
use cocoa::foundation::NSRect;
#[cfg(target_os = "macos")]
use objc::{class, msg_send, sel, sel_impl};

pub fn create_tray(app: &AppHandle) -> Result<TrayIcon, tauri::Error> {
    // 创建托盘菜单
    let open_main = MenuItem::with_id(app, "open_main", "打开主窗口", true, None::<&str>)?;
    let separator1 = PredefinedMenuItem::separator(app)?;
    let add_todo = MenuItem::with_id(app, "add_todo", "添加 Todo", true, None::<&str>)?;
    let add_note = MenuItem::with_id(app, "add_note", "添加便签", true, None::<&str>)?;
    let separator2 = PredefinedMenuItem::separator(app)?;
    let settings = MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[
            &open_main,
            &separator1,
            &add_todo,
            &add_note,
            &separator2,
            &settings,
            &quit,
        ],
    )?;

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
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "open_main" => {
                show_main_window(app);
            }
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
            if let TrayIconEvent::Click {
                button,
                button_state,
                position,
                ..
            } = event
            {
                match button {
                    tauri::tray::MouseButton::Left => {
                        // 只在鼠标松开时处理，避免按下和松开都触发
                        if button_state == tauri::tray::MouseButtonState::Up {
                            let app = tray.app_handle();
                            show_popup_window(app, position);
                        }
                    }
                    _ => {}
                }
            }
        })
        .build(app)?;

    Ok(tray)
}

fn show_popup_window(app: &AppHandle, click_pos: tauri::PhysicalPosition<f64>) {
    if let Some(window) = app.get_webview_window("popup") {
        let is_visible = window.is_visible().unwrap_or(false);

        if is_visible {
            let _ = window.hide();
        } else {
            // Position window near tray icon using click position
            position_popup_near_tray(&window, click_pos);
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

#[cfg(target_os = "macos")]
fn position_popup_near_tray(
    window: &tauri::WebviewWindow,
    click_pos: tauri::PhysicalPosition<f64>,
) {
    unsafe {
        // 获取主屏幕
        let screen: id = msg_send![class!(NSScreen), mainScreen];
        let screen_frame: NSRect = msg_send![screen, frame];
        let visible_frame: NSRect = msg_send![screen, visibleFrame];

        // 获取缩放因子
        let scale_factor = window.scale_factor().unwrap_or(1.0);

        // 获取窗口大小 (Physical Pixels)
        if let Ok(size) = window.outer_size() {
            let window_width = size.width as f64;
            // let window_height = size.height as f64;

            // 计算菜单栏高度 (Points)
            // Cocoa 坐标系 (Bottom-Left Origin):
            // screen top = screen_frame.size.height
            // visible top = visible_frame.origin.y + visible_frame.size.height
            let menu_bar_height_points =
                screen_frame.size.height - (visible_frame.origin.y + visible_frame.size.height);
            let menu_bar_height_pixels = menu_bar_height_points * scale_factor;

            // X 坐标计算 (Pixels)
            // 箭头在 CSS 中定义为居中 (left: 50%)
            // 所以我们希望窗口的中心对齐到点击位置 (Tray Icon)

            // 窗口中心 X = 点击位置 X
            // 窗口左上角 X = 点击位置 X - (窗口宽度 / 2)
            let desired_x = click_pos.x - (window_width / 2.0);

            // 确保窗口不超出屏幕边界
            // 减小边距限制，让窗口能更靠边，解决箭头对齐偏差问题
            let screen_width_pixels = screen_frame.size.width * scale_factor;
            let min_x = 2.0 * scale_factor;
            let max_x = screen_width_pixels - window_width - (2.0 * scale_factor);
            let x = desired_x.clamp(min_x, max_x);

            // Y 坐标计算 (Pixels, Top-Left Origin)
            // 只需要放在菜单栏下方即可
            let y = menu_bar_height_pixels; // 紧贴菜单栏，CSS 中有箭头处理视觉连接

            let position = PhysicalPosition::new(x as i32, y as i32);
            let _ = window.set_position(position);
        }
    }
}

#[cfg(not(target_os = "macos"))]
fn position_popup_near_tray(
    _window: &tauri::WebviewWindow,
    _click_pos: tauri::PhysicalPosition<f64>,
) {
    // 非 macOS 平台暂不实现定位
}

fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}
