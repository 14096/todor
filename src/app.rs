use crate::{storage, todo::TodoList, ui};
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

#[derive(Debug, PartialEq)]
pub enum InputMode {
    Normal,
    AddingTodo,
    EditingTodo,
}

#[derive(Debug, PartialEq)]
pub enum PopupField {
    Title,
    Category,
    Description,
}

#[derive(Debug)]
pub struct TodoForm {
    pub title: String,
    pub category: String,
    pub description: String,
    pub current_field: PopupField,
}

impl Default for TodoForm {
    fn default() -> Self {
        Self {
            title: String::new(),
            category: String::new(),
            description: String::new(),
            current_field: PopupField::Title,
        }
    }
}

impl TodoForm {
    pub fn clear(&mut self) {
        self.title.clear();
        self.category.clear();
        self.description.clear();
        self.current_field = PopupField::Title;
    }

    pub fn get_current_input(&self) -> &str {
        match self.current_field {
            PopupField::Title => &self.title,
            PopupField::Category => &self.category,
            PopupField::Description => &self.description,
        }
    }

    pub fn get_current_input_mut(&mut self) -> &mut String {
        match self.current_field {
            PopupField::Title => &mut self.title,
            PopupField::Category => &mut self.category,
            PopupField::Description => &mut self.description,
        }
    }

    pub fn next_field(&mut self) {
        self.current_field = match self.current_field {
            PopupField::Title => PopupField::Category,
            PopupField::Category => PopupField::Description,
            PopupField::Description => PopupField::Title,
        };
    }

    pub fn prev_field(&mut self) {
        self.current_field = match self.current_field {
            PopupField::Title => PopupField::Description,
            PopupField::Category => PopupField::Title,
            PopupField::Description => PopupField::Category,
        };
    }
}

pub struct App {
    pub todo_list: TodoList,
    pub input_mode: InputMode,
    pub todo_form: TodoForm,
    pub should_quit: bool,
    pub main_split_percentage: u16,
    pub editing_todo_id: Option<usize>,
}

impl App {
    pub fn new() -> Result<Self> {
        let todo_list = storage::load_todos()?;
        Ok(Self {
            todo_list,
            input_mode: InputMode::Normal,
            todo_form: TodoForm::default(),
            should_quit: false,
            main_split_percentage: 40,
            editing_todo_id: None,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.run_app(&mut terminal);

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        storage::save_todos(&self.todo_list)?;
        result
    }

    fn run_app(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        loop {
            terminal.draw(|f| ui::draw(f, self))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match self.input_mode {
                        InputMode::Normal => self.handle_normal_input(key.code),
                        InputMode::AddingTodo => self.handle_popup_input(key.code),
                        InputMode::EditingTodo => self.handle_popup_input(key.code),
                    }
                }
            }

            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn handle_normal_input(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('a') => {
                self.input_mode = InputMode::AddingTodo;
                self.todo_form.clear();
                self.editing_todo_id = None;
            }
            KeyCode::Char('e') => {
                if let Some(selected_idx) = self.todo_list.selected {
                    if let Some(todo) = self.todo_list.todos.get(selected_idx) {
                        let todo_clone = todo.clone();
                        self.input_mode = InputMode::EditingTodo;
                        self.editing_todo_id = Some(todo_clone.id);
                        self.populate_form_with_todo(&todo_clone);
                    }
                }
            }
            KeyCode::Up | KeyCode::Char('k') => self.todo_list.select_previous(),
            KeyCode::Down | KeyCode::Char('j') => self.todo_list.select_next(),
            KeyCode::Char(' ') => self.todo_list.toggle_selected(),
            KeyCode::Char('d') => {
                self.todo_list.remove_selected();
            }
            KeyCode::Char('[') => {
                self.main_split_percentage = self.main_split_percentage.saturating_sub(5).max(10);
            }
            KeyCode::Char(']') => {
                self.main_split_percentage = (self.main_split_percentage + 5).min(90);
            }
            _ => {}
        }
    }

    fn handle_popup_input(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Esc => {
                self.todo_form.clear();
                self.input_mode = InputMode::Normal;
                self.editing_todo_id = None;
            }
            KeyCode::Tab => {
                self.todo_form.next_field();
            }
            KeyCode::BackTab => {
                self.todo_form.prev_field();
            }
            KeyCode::Up if matches!(self.todo_form.current_field, PopupField::Description) => {
                self.todo_form.prev_field();
            }
            KeyCode::Down if !matches!(self.todo_form.current_field, PopupField::Description) => {
                self.todo_form.next_field();
            }
            KeyCode::Enter => {
                if matches!(self.todo_form.current_field, PopupField::Title)
                    && self.todo_form.title.trim().is_empty()
                {
                    return;
                }

                if !self.todo_form.title.trim().is_empty() {
                    let category = if self.todo_form.category.trim().is_empty() {
                        None
                    } else {
                        Some(self.todo_form.category.trim().to_string())
                    };

                    match self.input_mode {
                        InputMode::AddingTodo => {
                            self.create_todo_from_form(category);

                            if self.todo_list.selected.is_none() && !self.todo_list.todos.is_empty()
                            {
                                self.todo_list.selected = Some(self.todo_list.todos.len() - 1);
                            }
                        }
                        InputMode::EditingTodo => {
                            self.update_todo_from_form(category);
                        }
                        _ => {}
                    }

                    self.todo_form.clear();
                    self.input_mode = InputMode::Normal;
                    self.editing_todo_id = None;
                }
            }
            KeyCode::Backspace => {
                self.todo_form.get_current_input_mut().pop();
            }
            KeyCode::Char(c) => {
                self.todo_form.get_current_input_mut().push(c);
            }
            _ => {}
        }
    }

    fn create_todo_from_form(&mut self, category: Option<String>) {
        let mut todo = crate::todo::Todo::new_with_category(
            self.todo_list.next_id,
            self.todo_form.title.trim().to_string(),
            category,
        );

        if !self.todo_form.description.trim().is_empty() {
            todo.description = Some(self.todo_form.description.trim().to_string());
        }

        self.todo_list.todos.push(todo);
        self.todo_list.next_id += 1;
    }

    fn populate_form_with_todo(&mut self, todo: &crate::todo::Todo) {
        self.todo_form.title = todo.title.clone();
        self.todo_form.category = todo.category.clone().unwrap_or_default();
        self.todo_form.description = todo.description.clone().unwrap_or_default();
        self.todo_form.current_field = PopupField::Title;
    }

    fn update_todo_from_form(&mut self, category: Option<String>) {
        if let Some(editing_id) = self.editing_todo_id {
            if let Some(todo) = self.todo_list.todos.iter_mut().find(|t| t.id == editing_id) {
                todo.title = self.todo_form.title.trim().to_string();
                todo.category = category;
                todo.description = if self.todo_form.description.trim().is_empty() {
                    None
                } else {
                    Some(self.todo_form.description.trim().to_string())
                };
            }
        }
    }
}
