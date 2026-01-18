use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub category: Option<String>,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Todo {
    pub fn new(id: usize, title: String) -> Self {
        Self {
            id,
            title,
            category: None,
            description: None,
            completed: false,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn new_with_category(id: usize, title: String, category: Option<String>) -> Self {
        Self {
            id,
            title,
            category,
            description: None,
            completed: false,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }
}

#[derive(Debug, Default)]
pub struct TodoList {
    pub todos: Vec<Todo>,
    pub selected: Option<usize>,
    pub next_id: usize,
}

impl TodoList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_todo(&mut self, title: String) {
        let todo = Todo::new(self.next_id, title);
        self.todos.push(todo);
        self.next_id += 1;
    }

    pub fn add_todo_with_category(&mut self, title: String, category: Option<String>) {
        let todo = Todo::new_with_category(self.next_id, title, category);
        self.todos.push(todo);
        self.next_id += 1;
    }

    pub fn remove_selected(&mut self) -> Option<Todo> {
        if let Some(index) = self.selected {
            if index < self.todos.len() {
                let removed = self.todos.remove(index);
                if self.todos.is_empty() {
                    self.selected = None;
                } else if index >= self.todos.len() {
                    self.selected = Some(self.todos.len() - 1);
                }
                return Some(removed);
            }
        }
        None
    }

    pub fn toggle_selected(&mut self) {
        if let Some(index) = self.selected {
            if let Some(todo) = self.todos.get_mut(index) {
                todo.toggle_completed();
            }
        }
    }

    pub fn select_next(&mut self) {
        if !self.todos.is_empty() {
            self.selected = Some(match self.selected {
                Some(index) => (index + 1) % self.todos.len(),
                None => 0,
            });
        }
    }

    pub fn select_previous(&mut self) {
        if !self.todos.is_empty() {
            self.selected = Some(match self.selected {
                Some(index) => {
                    if index > 0 {
                        index - 1
                    } else {
                        self.todos.len() - 1
                    }
                }
                None => 0,
            });
        }
    }

    pub fn get_selected(&self) -> Option<&Todo> {
        self.selected.and_then(|index| self.todos.get(index))
    }
}
