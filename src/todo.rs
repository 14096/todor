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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_creation() {
        let todo = Todo::new(1, "Test todo".to_string());
        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "Test todo");
        assert_eq!(todo.category, None);
        assert_eq!(todo.description, None);
        assert!(!todo.completed);
    }

    #[test]
    fn test_todo_with_category() {
        let todo = Todo::new_with_category(1, "Test todo".to_string(), Some("Work".to_string()));
        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "Test todo");
        assert_eq!(todo.category, Some("Work".to_string()));
        assert!(!todo.completed);
    }

    #[test]
    fn test_todo_toggle_completed() {
        let mut todo = Todo::new(1, "Test todo".to_string());
        assert!(!todo.completed);

        todo.toggle_completed();
        assert!(todo.completed);

        todo.toggle_completed();
        assert!(!todo.completed);
    }

    #[test]
    fn test_todo_list_creation() {
        let todo_list = TodoList::new();
        assert!(todo_list.todos.is_empty());
        assert_eq!(todo_list.selected, None);
        assert_eq!(todo_list.next_id, 0);
    }

    #[test]
    fn test_add_todo() {
        let mut todo_list = TodoList::new();
        todo_list.add_todo("First todo".to_string());

        assert_eq!(todo_list.todos.len(), 1);
        assert_eq!(todo_list.todos[0].title, "First todo");
        assert_eq!(todo_list.todos[0].id, 0);
        assert_eq!(todo_list.next_id, 1);
    }

    #[test]
    fn test_add_multiple_todos() {
        let mut todo_list = TodoList::new();
        todo_list.add_todo("First todo".to_string());
        todo_list.add_todo("Second todo".to_string());

        assert_eq!(todo_list.todos.len(), 2);
        assert_eq!(todo_list.todos[0].id, 0);
        assert_eq!(todo_list.todos[1].id, 1);
        assert_eq!(todo_list.next_id, 2);
    }

    #[test]
    fn test_add_todo_with_category() {
        let mut todo_list = TodoList::new();
        todo_list.add_todo_with_category("Work task".to_string(), Some("Work".to_string()));

        assert_eq!(todo_list.todos.len(), 1);
        assert_eq!(todo_list.todos[0].title, "Work task");
        assert_eq!(todo_list.todos[0].category, Some("Work".to_string()));
    }

    #[test]
    fn test_selection_navigation() {
        let mut todo_list = TodoList::new();
        todo_list.add_todo("Todo 1".to_string());
        todo_list.add_todo("Todo 2".to_string());
        todo_list.add_todo("Todo 3".to_string());

        // Initially no selection
        assert_eq!(todo_list.selected, None);

        // Select next (should wrap to first)
        todo_list.select_next();
        assert_eq!(todo_list.selected, Some(0));

        // Select next
        todo_list.select_next();
        assert_eq!(todo_list.selected, Some(1));

        // Select next
        todo_list.select_next();
        assert_eq!(todo_list.selected, Some(2));

        // Select next (should wrap to first)
        todo_list.select_next();
        assert_eq!(todo_list.selected, Some(0));

        // Select previous (should wrap to last)
        todo_list.select_previous();
        assert_eq!(todo_list.selected, Some(2));
    }

    #[test]
    fn test_get_selected() {
        let mut todo_list = TodoList::new();
        todo_list.add_todo("Selected todo".to_string());
        todo_list.selected = Some(0);

        let selected = todo_list.get_selected();
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().title, "Selected todo");

        todo_list.selected = None;
        assert!(todo_list.get_selected().is_none());
    }

    #[test]
    fn test_toggle_selected() {
        let mut todo_list = TodoList::new();
        todo_list.add_todo("Toggle todo".to_string());
        todo_list.selected = Some(0);

        assert!(!todo_list.todos[0].completed);

        todo_list.toggle_selected();
        assert!(todo_list.todos[0].completed);

        todo_list.toggle_selected();
        assert!(!todo_list.todos[0].completed);
    }

    #[test]
    fn test_remove_selected() {
        let mut todo_list = TodoList::new();
        todo_list.add_todo("Todo 1".to_string());
        todo_list.add_todo("Todo 2".to_string());
        todo_list.add_todo("Todo 3".to_string());
        todo_list.selected = Some(1);

        let removed = todo_list.remove_selected();
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().title, "Todo 2");
        assert_eq!(todo_list.todos.len(), 2);
        assert_eq!(todo_list.selected, Some(1)); // Should adjust to next item
        assert_eq!(todo_list.todos[1].title, "Todo 3");
    }

    #[test]
    fn test_remove_last_item() {
        let mut todo_list = TodoList::new();
        todo_list.add_todo("Only todo".to_string());
        todo_list.selected = Some(0);

        let removed = todo_list.remove_selected();
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().title, "Only todo");
        assert!(todo_list.todos.is_empty());
        assert_eq!(todo_list.selected, None);
    }

    #[test]
    fn test_remove_with_no_selection() {
        let mut todo_list = TodoList::new();
        todo_list.add_todo("Todo 1".to_string());
        todo_list.selected = None;

        let removed = todo_list.remove_selected();
        assert!(removed.is_none());
        assert_eq!(todo_list.todos.len(), 1);
    }
}
