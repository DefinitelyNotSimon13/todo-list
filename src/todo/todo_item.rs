use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

mod created_todo_item;
mod queried_todo_item;

pub use created_todo_item::CreatedTodoItem;
pub use queried_todo_item::QueriedTodoItem;

trait IntoTodoItem {
    fn into_todo_item(&self) -> TodoItem;
}

#[derive(Clone, Debug)]
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
}
