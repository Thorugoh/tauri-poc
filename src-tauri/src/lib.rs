// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct TodoState {
    todos: Vec<String>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_todos(state: tauri::State<Arc<Mutex<TodoState>>>) -> Vec<String> {
    let todos = state.lock().unwrap();
    todos.todos.clone()
}

#[tauri::command]
fn add_todo(task: String, state: tauri::State<Arc<Mutex<TodoState>>>) -> Vec<String> {
    let mut todos = state.lock().unwrap();
    todos.todos.push(task);
    todos.todos.clone()
}

#[tauri::command]
fn delete_todo(index: usize, state: tauri::State<Arc<Mutex<TodoState>>>) -> Vec<String> {
    let mut todos = state.lock().unwrap();
    if index < todos.todos.len() {
        todos.todos.remove(index);
    }
    todos.todos.clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(TodoState::default())))
        .invoke_handler(tauri::generate_handler![
            get_todos,
            add_todo,
            delete_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}