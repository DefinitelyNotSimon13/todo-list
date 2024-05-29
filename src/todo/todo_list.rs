use super::TodoItem;

#[derive(Default)]
pub struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    pub fn new(items: Vec<TodoItem>) -> Self {
        Self { items }
    }
    pub fn add_item(&mut self, item: TodoItem) {
        self.items.push(item)
    }

    pub fn get_items(&self) -> &Vec<TodoItem> {
        &self.items
    }
}
