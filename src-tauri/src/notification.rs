use crate::commands::get_todos;
use chrono::Utc;
use std::sync::Mutex;
use std::time::Duration;
use tauri::{AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;

pub struct NotificationState {
    pub notified_todos: Mutex<Vec<String>>,
}

impl NotificationState {
    pub fn new() -> Self {
        Self {
            notified_todos: Mutex::new(Vec::new()),
        }
    }
}

pub async fn check_reminders(app: AppHandle) {
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;

        let todos_result = get_todos(app.clone()).await;
        if let Ok(todos) = todos_result {
            let now = Utc::now();

            if let Some(state) = app.try_state::<NotificationState>() {
                let mut notified = state.notified_todos.lock().unwrap();

                for todo in todos {
                    if let Some(remind_time) = todo.remind_time {
                        if !todo.completed && remind_time <= now && !notified.contains(&todo.id) {
                            // 发送通知
                            let _ = app
                                .notification()
                                .builder()
                                .title("Todo 提醒")
                                .body(&format!("{}: {}", todo.title, todo.content))
                                .show();

                            notified.push(todo.id.clone());
                        }
                    }
                }
            }
        }
    }
}
