use std::env;

use color_eyre::eyre::Result;
use diesel::{PgConnection, RunQueryDsl, SelectableHelper};
use dotenvy::dotenv;
use models::{NewTodoItem, TodoItem};

pub mod cli;
pub mod database;
pub mod models;
pub mod schema;
pub mod todo;

pub fn get_connection_string() -> Result<String> {
    dotenv().ok();
    Ok(env::var("DATABASE_URL")?)
}

pub fn create_item(conn: &mut PgConnection, new_item: NewTodoItem) -> Result<TodoItem> {
    use crate::schema::todo_items;

    Ok(diesel::insert_into(todo_items::table)
        .values(&new_item)
        .returning(TodoItem::as_returning())
        .get_result(conn)?)
}
