mod commands;
mod db;
mod migration;
mod models;
mod notification;
mod tray;

use notification::NotificationState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .manage(NotificationState::new())
        .setup(|app| {
            // 初始化数据库
            let db = tauri::async_runtime::block_on(db::init_db(app.handle()))
                 .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn std::error::Error>)?;
            
            // 检查并迁移数据
            tauri::async_runtime::block_on(migration::check_and_migrate_from_json(app.handle(), &db))
                 .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn std::error::Error>)?;

            app.manage(db);

            // 创建系统托盘
            tray::create_tray(app.handle())?;

            // Apply vibrancy (MacOS only) - 仅对主窗口应用，悬浮窗使用 CSS 模糊以支持异形（箭头）
            if let Some(window) = app.get_webview_window("main") {
                #[cfg(target_os = "macos")]
                {
                    use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
                    let _ = apply_vibrancy(
                        &window,
                        NSVisualEffectMaterial::UnderWindowBackground,
                        None,
                        Some(16.0),
                    );
                }
            }

            // Apply vibrancy to popup window

            // 启动提醒检查任务
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                notification::check_reminders(app_handle).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_todos,
            commands::save_todo,
            commands::delete_todo,
            commands::get_notes,
            commands::save_note,
            commands::delete_note,
            commands::get_settings,
            commands::save_settings,
            commands::apply_vibrancy,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| {
            #[cfg(target_os = "macos")]
            if let tauri::RunEvent::Reopen {
                has_visible_windows: _,
                ..
            } = event
            {
                if let Some(window) = app.get_webview_window("main") {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
        });
}
