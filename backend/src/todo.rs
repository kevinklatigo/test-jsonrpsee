use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u64,
    pub text: String,
    pub done: bool,
}

#[derive(Clone)]
pub struct TodoStore {
    todos: Arc<RwLock<Vec<Todo>>>,
    next_id: Arc<AtomicU64>,
}

impl TodoStore {
    pub fn new() -> Self {
        Self {
            todos: Arc::new(RwLock::new(Vec::new())),
            next_id: Arc::new(AtomicU64::new(1)),
        }
    }

    pub fn list(&self) -> Vec<Todo> {
        self.todos.read().unwrap().clone()
    }

    pub fn add(&self, text: String) -> Todo {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let todo = Todo {
            id,
            text,
            done: false,
        };
        self.todos.write().unwrap().push(todo.clone());
        todo
    }

    pub fn toggle(&self, id: u64) -> Option<Todo> {
        let mut todos = self.todos.write().unwrap();
        if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
            todo.done = !todo.done;
            Some(todo.clone())
        } else {
            None
        }
    }

    pub fn remove(&self, id: u64) -> bool {
        let mut todos = self.todos.write().unwrap();
        let len_before = todos.len();
        todos.retain(|t| t.id != id);
        todos.len() != len_before
    }

    pub fn clear_completed(&self) -> u64 {
        let mut todos = self.todos.write().unwrap();
        let len_before = todos.len();
        todos.retain(|t| !t.done);
        (len_before - todos.len()) as u64
    }
}
