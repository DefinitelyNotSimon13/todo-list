use color_eyre::eyre::Result;
use diesel::prelude::*;

use crate::models::UpdatedTodoItem;
use crate::DbConnection;
use crate::{
    create_item,
    database::Database,
    models::{NewTodoItem, TodoItem},
};

pub struct TodoList<'d> {
    database: &'d mut Database,
}

impl<'d> TodoList<'d> {
    pub fn new(database: &'d mut Database) -> Self {
        Self { database }
    }
    pub fn add_item(&mut self, item: NewTodoItem) -> Result<TodoItem> {
        let item = create_item(&mut self.get_db_connection(), item)?;
        Ok(item)
    }

    pub fn add_item_static(conn: &mut DbConnection, item: NewTodoItem) {
        let _ = create_item(conn, item).unwrap();
    }

    pub fn get_items(&mut self, get_all: bool) -> Result<Vec<TodoItem>> {
        use crate::schema::todo_items::dsl::*;

        if !get_all {
            Ok(todo_items
                .filter(completed.eq(false))
                .select(TodoItem::as_select())
                .load(&mut self.get_db_connection())?)
        } else {
            Ok(todo_items
                .select(TodoItem::as_select())
                .load(&mut self.get_db_connection())?)
        }
    }

    pub fn get_item_with_id(&mut self, queried_id: i32) -> Result<TodoItem> {
        use crate::schema::todo_items::dsl::*;

        Ok(todo_items
            .find(queried_id)
            .select(TodoItem::as_select())
            .first(&mut self.get_db_connection())?)
    }

    pub fn get_item_with_title(&mut self, queried_title: &str) -> Result<Vec<TodoItem>> {
        use crate::schema::todo_items::dsl::*;

        Ok(todo_items
            .filter(title.like(format!("%{queried_title}%")))
            .select(TodoItem::as_select())
            .load(&mut self.get_db_connection())?)
    }

    pub fn complete_item(&mut self, id: i32) -> Result<TodoItem> {
        use crate::schema::todo_items::dsl::{completed, todo_items};

        let completed_item = diesel::update(todo_items.find(id))
            .set(completed.eq(true))
            .returning(TodoItem::as_returning())
            .get_result(&mut self.get_db_connection())?;

        Ok(completed_item)
    }

    pub fn get_db_connection(&mut self) -> DbConnection {
        self.database.get_connection()
    }

    pub fn delete_item_with_id(&mut self, deleted_id: i32) -> Result<usize> {
        use crate::schema::todo_items::dsl::{id, todo_items};

        let deleted_num = diesel::delete(todo_items.filter(id.eq(deleted_id)))
            .execute(&mut self.get_db_connection())?;
        Ok(deleted_num)
    }

    pub fn update_item(&mut self, id: i32, updated_item: UpdatedTodoItem) -> Result<TodoItem> {
        use crate::schema::todo_items::dsl::todo_items;

        let updated_item = diesel::update(todo_items.find(id))
            .set(&updated_item)
            .returning(TodoItem::as_returning())
            .get_result(&mut self.get_db_connection())?;

        Ok(updated_item)
    }
}
