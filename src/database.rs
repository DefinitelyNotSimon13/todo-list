use color_eyre::Result;
use std::{env, path::Path};
use uuid::Uuid;

use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    query, query_as, Connection, PgPool, Postgres,
};

use crate::todo::{
    todo_item::{CreatedTodoItem, QueriedTodoItem},
    TodoItem, TodoList,
};

pub struct Database {
    connection: PgPool,
}

impl Database {
    pub async fn new(url: &str) -> Result<Self> {
        let connection = PgPoolOptions::new().max_connections(5).connect(url).await?;
        Ok(Self { connection })
    }

    pub async fn add_item(&mut self, item: &CreatedTodoItem) -> Result<()> {
        query!(
            "
INSERT INTO todo_item (uuid, title, description, created_at) VALUES ($1, $2, $3, $4)
",
            Uuid::new_v4(),
            item.title,
            item.description,
            item.created_at
        )
        .fetch_all(&self.connection)
        .await?;
        Ok(())
    }

    pub async fn retrieve_data(&mut self) -> Result<TodoList> {
        let items = query_as!(QueriedTodoItem, "SELECT * FROM todo_item")
            .fetch_all(&self.connection)
            .await?
            .iter()
            .map(TodoItem::from)
            .collect();
        Ok(TodoList::new(items))
    }
}
