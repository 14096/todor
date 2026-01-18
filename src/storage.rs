use crate::todo::TodoList;
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

fn get_config_dir() -> Result<PathBuf> {
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| anyhow::anyhow!("Could not determine home directory"))?;

    let config_dir = Path::new(&home_dir).join(".config").join("todor");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    Ok(config_dir)
}

fn get_data_file_path() -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join("todos.json"))
}

pub fn load_todos() -> Result<TodoList> {
    let data_file = get_data_file_path()?;

    if data_file.exists() {
        let data = fs::read_to_string(&data_file)?;
        let todos: Vec<crate::todo::Todo> = serde_json::from_str(&data)?;

        let mut todo_list = TodoList::new();
        for todo in todos {
            let id = todo.id;
            todo_list.todos.push(todo);
            todo_list.next_id = todo_list.next_id.max(id + 1);
        }

        if !todo_list.todos.is_empty() {
            todo_list.selected = Some(0);
        }

        Ok(todo_list)
    } else {
        Ok(TodoList::new())
    }
}

pub fn save_todos(todo_list: &TodoList) -> Result<()> {
    let data_file = get_data_file_path()?;
    let data = serde_json::to_string_pretty(&todo_list.todos)?;
    fs::write(data_file, data)?;
    Ok(())
}
