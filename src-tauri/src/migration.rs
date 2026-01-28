use crate::db::Database;
use crate::models::{Note, Todo};
use std::collections::HashMap;
use std::fs;
use tauri::{AppHandle, Manager};

pub async fn check_and_migrate_from_json(app: &AppHandle, db: &Database) -> Result<(), String> {
    // Check if we need to migrate (tables empty)
    let todos_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM todos")
        .fetch_one(&db.pool)
        .await
        .map_err(|e| format!("Failed to count todos: {}", e))?;

    let notes_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM notes")
        .fetch_one(&db.pool)
        .await
        .map_err(|e| format!("Failed to count notes: {}", e))?;

    if todos_count > 0 || notes_count > 0 {
        println!("Database already has data (todos: {}, notes: {}). Skipping migration.", todos_count, notes_count);
        return Ok(());
    }

    println!("Database is empty. Checking for legacy store.json...");

    let app_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    let store_path = app_dir.join("store.json");

    if !store_path.exists() {
        println!("No legacy store.json found. Skipping migration.");
        return Ok(());
    }

    let content = fs::read_to_string(&store_path)
        .map_err(|e| format!("Failed to read store.json: {}", e))?;

    let store_data: HashMap<String, serde_json::Value> = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse store.json: {}", e))?;

    // Migrate Todos
    if let Some(todos_value) = store_data.get("todos") {
        if let Ok(todos) = serde_json::from_value::<Vec<Todo>>(todos_value.clone()) {
            println!("Migrating {} todos...", todos.len());
            for todo in todos {
                sqlx::query(
                    "INSERT INTO todos (id, title, content, remind_time, completed, created_at) VALUES (?, ?, ?, ?, ?, ?)"
                )
                .bind(todo.id)
                .bind(todo.title)
                .bind(todo.content)
                .bind(todo.remind_time.map(|d| d.to_rfc3339())) // Store DateTime as string
                .bind(todo.completed)
                .bind(todo.created_at.to_rfc3339())
                .execute(&db.pool)
                .await
                .map_err(|e| format!("Failed to insert old todo: {}", e))?;
            }
        }
    }

    // Migrate Notes
    if let Some(notes_value) = store_data.get("notes") {
        if let Ok(notes) = serde_json::from_value::<Vec<Note>>(notes_value.clone()) {
             println!("Migrating {} notes...", notes.len());
            for note in notes {
                 sqlx::query(
                    "INSERT INTO notes (id, title, content, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"
                )
                .bind(note.id)
                .bind(note.title)
                .bind(note.content)
                .bind(note.created_at.to_rfc3339())
                .bind(note.updated_at.to_rfc3339())
                .execute(&db.pool)
                .await
                .map_err(|e| format!("Failed to insert old note: {}", e))?;
            }
        }
    }

    println!("Migration completed successfully.");
    Ok(())
}
