use crate::cli::Command;
use crate::storage;
use crate::todo::Todo;
use std::io;

pub fn execute(command: Command, file_path: &str) -> io::Result<()> {
    let mut todos = storage::load_todos(file_path)?;

    match command {
        Command::Add { text } => {
            let next_id = todos.iter().map(|todo| todo.id).max().unwrap_or(0) + 1;
            todos.push(Todo {
                id: next_id,
                text,
                completed: false,
            });
            storage::save_todos(file_path, &todos)?;
            println!("Added todo #{next_id}");
        }
        Command::List => {
            if todos.is_empty() {
                println!("No todos yet.");
            } else {
                for todo in &todos {
                    let status = if todo.completed { "x" } else { " " };
                    println!("[{}] {}: {}", status, todo.id, todo.text);
                }
            }
        }
        Command::Complete { id } => {
            let mut found = false;
            for todo in &mut todos {
                if todo.id == id {
                    todo.completed = true;
                    found = true;
                    break;
                }
            }

            if found {
                storage::save_todos(file_path, &todos)?;
                println!("Completed todo #{id}");
            } else {
                println!("Todo #{id} not found");
            }
        }
        Command::Delete { id } => {
            let original_len = todos.len();
            todos.retain(|todo| todo.id != id);

            if todos.len() < original_len {
                storage::save_todos(file_path, &todos)?;
                println!("Deleted todo #{id}");
            } else {
                println!("Todo #{id} not found");
            }
        }
    }

    Ok(())
}
