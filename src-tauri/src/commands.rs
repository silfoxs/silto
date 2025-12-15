use crate::models::{Note, Settings, Todo};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const TODOS_STORE_KEY: &str = "todos";
const NOTES_STORE_KEY: &str = "notes";
const SETTINGS_STORE_KEY: &str = "settings";

#[tauri::command]
pub async fn get_todos(app: AppHandle) -> Result<Vec<Todo>, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;

    let todos_value = store.get(TODOS_STORE_KEY);

    if let Some(value) = todos_value {
        let todos: Vec<Todo> = serde_json::from_value(value.clone())
            .map_err(|e| format!("Failed to parse todos: {}", e))?;
        Ok(todos)
    } else {
        Ok(vec![])
    }
}

#[tauri::command]
pub async fn save_todo(app: AppHandle, todo: Todo) -> Result<(), String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;

    let mut todos = get_todos(app.clone()).await?;

    // Check if todo exists, update or add
    if let Some(pos) = todos.iter().position(|t| t.id == todo.id) {
        todos[pos] = todo;
    } else {
        todos.push(todo);
    }

    let todos_value =
        serde_json::to_value(&todos).map_err(|e| format!("Failed to serialize todos: {}", e))?;

    store.set(TODOS_STORE_KEY, todos_value);
    store.save().map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn delete_todo(app: AppHandle, id: String) -> Result<(), String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;

    let mut todos = get_todos(app.clone()).await?;
    todos.retain(|t| t.id != id);

    let todos_value =
        serde_json::to_value(&todos).map_err(|e| format!("Failed to serialize todos: {}", e))?;

    store.set(TODOS_STORE_KEY, todos_value);
    store.save().map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_notes(app: AppHandle) -> Result<Vec<Note>, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;

    let notes_value = store.get(NOTES_STORE_KEY);

    if let Some(value) = notes_value {
        let notes: Vec<Note> = serde_json::from_value(value.clone())
            .map_err(|e| format!("Failed to parse notes: {}", e))?;
        Ok(notes)
    } else {
        Ok(vec![])
    }
}

#[tauri::command]
pub async fn save_note(app: AppHandle, note: Note) -> Result<(), String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;

    let mut notes = get_notes(app.clone()).await?;

    // Check if note exists, update or add
    if let Some(pos) = notes.iter().position(|n| n.id == note.id) {
        notes[pos] = note;
    } else {
        notes.push(note);
    }

    let notes_value =
        serde_json::to_value(&notes).map_err(|e| format!("Failed to serialize notes: {}", e))?;

    store.set(NOTES_STORE_KEY, notes_value);
    store.save().map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn delete_note(app: AppHandle, id: String) -> Result<(), String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;

    let mut notes = get_notes(app.clone()).await?;
    notes.retain(|n| n.id != id);

    let notes_value =
        serde_json::to_value(&notes).map_err(|e| format!("Failed to serialize notes: {}", e))?;

    store.set(NOTES_STORE_KEY, notes_value);
    store.save().map_err(|e| e.to_string())?;

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

    Ok(())
}
