use crate::todo::Todo;
use std::fs;
use std::io;
use std::path::Path;

pub fn load_todos(file_path: &str) -> io::Result<Vec<Todo>> {
    if !Path::new(file_path).exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(file_path)?;

    if content.trim().is_empty() {
        return Ok(Vec::new());
    }

    let todos: Vec<Todo> = serde_json::from_str(&content).map_err(invalid_data)?;
    Ok(todos)
}

pub fn save_todos(file_path: &str, todos: &[Todo]) -> io::Result<()> {
    let json = serde_json::to_string_pretty(todos).map_err(invalid_data)?;
    fs::write(file_path, json)
}

fn invalid_data(error: serde_json::Error) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error)
}
