use crate::db::Database;
use crate::models::{Note, Settings, Todo};
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_store::StoreExt;

const SETTINGS_STORE_KEY: &str = "settings";

#[tauri::command]
pub async fn get_todos(app: AppHandle) -> Result<Vec<Todo>, String> {
    let db: State<Database> = app.state();
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY created_at DESC")
        .fetch_all(&db.pool)
        .await
        .map_err(|e| format!("Failed to fetch todos: {}", e))?;
    Ok(todos)
}

#[tauri::command]
pub async fn save_todo(app: AppHandle, todo: Todo) -> Result<(), String> {
    let db: State<Database> = app.state();
    
    // Store DateTime as string (rfc3339) or rely on sqlx implementation if supported
    // Since we used TEXT in migration, we bind it carefully.
    // If sqlx converts DateTime to string automatically, it works.
    // If not, we might need manual bind. 
    // Usually sqlx sqlite + chrono works fine.
    
    // Check if we need to reset notified status
    // If there is a remind time and it's in the future, we reset notified to false
    // regardless of what the frontend sent.
    let mut todo = todo;
    if let Some(remind_time) = todo.remind_time {
        if remind_time > chrono::Utc::now() {
            todo.notified = false;
        }
    } else {
        todo.notified = false;
    }

    sqlx::query(
        "INSERT OR REPLACE INTO todos (id, title, content, remind_time, completed, created_at, notified) VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(todo.id)
    .bind(todo.title)
    .bind(todo.content)
    .bind(todo.remind_time)
    .bind(todo.completed)
    .bind(todo.created_at)
    .bind(todo.notified)
    .execute(&db.pool)
    .await
    .map_err(|e| format!("Failed to save todo: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn delete_todo(app: AppHandle, id: String) -> Result<(), String> {
    let db: State<Database> = app.state();
    sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(&db.pool)
        .await
        .map_err(|e| format!("Failed to delete todo: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn get_notes(app: AppHandle) -> Result<Vec<Note>, String> {
    let db: State<Database> = app.state();
    let notes = sqlx::query_as::<_, Note>("SELECT * FROM notes ORDER BY updated_at DESC")
        .fetch_all(&db.pool)
        .await
        .map_err(|e| format!("Failed to fetch notes: {}", e))?;
    Ok(notes)
}

#[tauri::command]
pub async fn save_note(app: AppHandle, note: Note) -> Result<(), String> {
    let db: State<Database> = app.state();
    sqlx::query(
        "INSERT OR REPLACE INTO notes (id, title, content, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(note.id)
    .bind(note.title)
    .bind(note.content)
    .bind(note.created_at)
    .bind(note.updated_at)
    .execute(&db.pool)
    .await
    .map_err(|e| format!("Failed to save note: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn delete_note(app: AppHandle, id: String) -> Result<(), String> {
    let db: State<Database> = app.state();
    sqlx::query("DELETE FROM notes WHERE id = ?")
        .bind(id)
        .execute(&db.pool)
        .await
        .map_err(|e| format!("Failed to delete note: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn get_settings(app: AppHandle) -> Result<Settings, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;

    let settings_value = store.get(SETTINGS_STORE_KEY);

    if let Some(value) = settings_value {
        let settings: Settings = serde_json::from_value(value.clone())
            .map_err(|e| format!("Failed to parse settings: {}", e))?;
        Ok(settings)
    } else {
        Ok(Settings::default())
    }
}

#[tauri::command]
pub async fn save_settings(app: AppHandle, settings: Settings) -> Result<(), String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;

    let settings_value = serde_json::to_value(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    store.set(SETTINGS_STORE_KEY, settings_value);
    store.save().map_err(|e| e.to_string())?;

    // Emit event to notify frontend of changes
    app.emit("settings-changed", &settings)
        .map_err(|e| e.to_string())?;

    // Update tray menu language
    crate::tray::update_tray_lang(&app, &settings.language).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn apply_vibrancy(app: AppHandle, theme: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
        let window = app
            .get_webview_window("main")
            .ok_or("Main window not found")?;

        // Set transparency based on theme
        match theme.as_str() {
            "light" => {
                let _ = window.set_theme(Some(tauri::Theme::Light));
            }
            "dark" => {
                let _ = window.set_theme(Some(tauri::Theme::Dark));
            }
            _ => {
                let _ = window.set_theme(None);
            }
        }

        let material = NSVisualEffectMaterial::UnderWindowBackground;

        apply_vibrancy(&window, material, None, Some(16.0))
            .map_err(|e| format!("Failed to apply vibrancy: {}", e))?;
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = theme;
        let _ = app;
    }
    Ok(())
}
