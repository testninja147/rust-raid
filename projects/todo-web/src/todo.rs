use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Todo {
    id: usize,
    title: String,
    content: String,
    checked: bool,
    // created: u64,
}
impl Todo {
    fn new(id: usize, title: String, content: String) -> Self {
        Self {
            id,
            title,
            content,
            checked: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TodoList {
    current_index: usize,
    pub(crate) items: HashMap<usize, Todo>,
}

impl TodoList {
    pub(crate) fn new() -> Self {
        let mut todo_list = Self {
            items: HashMap::new(),
            current_index: 0,
        };
        todo_list.insert("Title 1".to_owned(), "This is content 1".to_owned());
        todo_list.insert("Title 2".to_owned(), "This is content 2".to_owned());
        todo_list
    }

    fn insert(&mut self, title: String, content: String) {
        self.current_index += 1;
        self.items.insert(
            self.current_index,
            Todo::new(self.current_index, title, content),
        );
    }
}
