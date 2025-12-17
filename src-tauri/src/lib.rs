mod commands;
mod models;
mod notification;
mod tray;

use notification::NotificationState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(NotificationState::new())
        .setup(|app| {
            // 创建系统托盘
            tray::create_tray(app.handle())?;

            // Apply vibrancy (MacOS only)
            if let Some(window) = app.get_webview_window("main") {
                use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
                apply_vibrancy(
                    &window,
                    NSVisualEffectMaterial::UnderWindowBackground,
                    None,
                    Some(16.0),
                )
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
            }

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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
