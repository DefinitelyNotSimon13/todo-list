use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

use super::{IntoTodoItem, TodoItem};

pub struct QueriedTodoItem {
    pub id: i32,
    pub uuid: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub deadline: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl IntoTodoItem for QueriedTodoItem {
    fn into_todo_item(&self) -> TodoItem {
        TodoItem {
            id: Some(self.id),
            uuid: self.uuid,
            title: self.title.clone(),
            description: self.description.clone(),
            completed: self.completed,
            deadline: self.deadline,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
