use color_eyre::eyre::Result;

use super::todo_item::LocalTodoItem;
use super::TodoItem;
use crate::database::Database;

pub struct TodoList<'d> {
    database: &'d Database,
    items: Vec<TodoItem>,
}

impl<'d> TodoList<'d> {
    pub fn new(items: Vec<TodoItem>, database: &'d Database) -> Self {
        Self { database, items }
    }
    pub async fn add_item(&mut self, item: LocalTodoItem) -> Result<TodoItem> {
        let uuid = item.insert_into_db(self.database.get_connection()).await?;
        // Sanity check
        let item = TodoItem::query_with_uuid(uuid, self.database.get_connection()).await?;
        self.items.push(item.clone());

        Ok(item)
    }

    pub fn get_items(&self) -> &Vec<TodoItem> {
        &self.items
    }
    pub async fn read(&mut self) -> Result<()> {
        let items = TodoItem::query_all(self.database.get_connection()).await?;
        self.items.extend(items);
        Ok(())
    }
}
