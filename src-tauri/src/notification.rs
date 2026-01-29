use crate::db::Database;
use crate::models::Todo;
use chrono::Utc;
use std::time::Duration;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_notification::NotificationExt;

pub async fn check_reminders(app: AppHandle) {
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;

        let now = Utc::now();
        let db_state: State<Database> = app.state();
        
        // Optimize: Only fetch overdue reminders that haven't been notified
        // Filter by time in SQL to avoid loading future tasks into memory
        // LIMIT 100 to prevent memory spikes if there's a massive backlog of overdue tasks
        let todos_result = sqlx::query_as::<_, Todo>(
            "SELECT * FROM todos WHERE completed = 0 AND notified = 0 AND remind_time IS NOT NULL AND remind_time <= ? LIMIT 100"
        )
        .bind(now)
        .fetch_all(&db_state.pool)
        .await;

        if let Ok(todos) = todos_result {
            for todo in todos {
                 // Double check not strictly needed if SQL is correct, but safe to keep logical check or just proceed.
                 // Since we filtered by remind_time <= now in SQL, we can process directly.
                
                 // 发送通知
                 let _ = app
                    .notification()
                    .builder()
                            .title("Todo 提醒")
                            .body(&format!("{}: {}", todo.title, todo.content))
                            .show();

                        // 更新 notified 状态
                        let _ = sqlx::query("UPDATE todos SET notified = 1 WHERE id = ?")
                            .bind(todo.id)
                            .execute(&db_state.pool)
                            .await;
            }
        }
    }
}
