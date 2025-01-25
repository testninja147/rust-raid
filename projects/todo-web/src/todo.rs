use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Todo {
    id: usize,
    content: String,
    checked: bool,
    // created: u64,
}
impl Todo {
    fn new(id: usize, content: String) -> Self {
        Self {
            id,
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
        todo_list.insert("This is content 1".to_owned());
        todo_list.insert("This is content 2".to_owned());
        todo_list
    }

    fn insert(&mut self, content: String) {
        self.current_index += 1;
        self.items
            .insert(self.current_index, Todo::new(self.current_index, content));
    }

    fn list(&self) -> Vec<Todo> {
        // let items = (&self.items)
        //     .iter()
        //     .map(|(_, v)| v.clone().to_owned().to_owned().to_owned())
        //     .collect::<Vec<Todo>>();
        // items
        return vec![];
    }

    fn get(&self, id: usize) -> Option<&Todo> {
        self.items.get(&id)
    }
}
