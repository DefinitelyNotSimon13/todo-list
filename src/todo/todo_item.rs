use color_eyre::eyre::Result;
use sqlx::{query_as, types::time::OffsetDateTime, PgPool};
use uuid::Uuid;

mod created_todo_item;

pub use created_todo_item::CreatedTodoItem;

pub trait IntoTodoItem {
    fn into_todo_item(&self) -> TodoItem;
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct TodoItem {
    id: Option<i32>,
    uuid: Uuid,
    title: String,
    description: Option<String>,
    completed: bool,
    deadline: Option<OffsetDateTime>,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl TodoItem {
    pub fn from(source: impl IntoTodoItem) -> Self {
        source.into_todo_item()
    }

    pub async fn query_all(connection: &PgPool) -> Result<Vec<TodoItem>> {
        Ok(query_as!(TodoItem, "SELECT * from todo_item")
            .fetch_all(connection)
            .await?)
    }

    pub async fn query_with_uuid(uuid: Uuid, connection: &PgPool) -> Result<Self> {
        Ok(
            query_as!(TodoItem, "SELECT * FROM todo_item where uuid = $1", uuid)
                .fetch_one(connection)
                .await?,
        )
    }
}
