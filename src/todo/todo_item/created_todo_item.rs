use color_eyre::Result;
use sqlx::{query, types::time::OffsetDateTime, PgPool};
use uuid::Uuid;

use super::{IntoTodoItem, TodoItem};

#[derive(Debug)]
pub struct CreatedTodoItem {
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub deadline: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}

impl CreatedTodoItem {
    pub fn new(title: &str, description: Option<&str>, deadline: Option<OffsetDateTime>) -> Self {
        Self {
            title: title.to_owned(),
            description: match description {
                Some(str) => Some(str.to_owned()),
                None => None,
            },
            completed: false,
            deadline,
            created_at: OffsetDateTime::now_utc(),
        }
    }
    pub async fn insert_into_db(&self, connection: &PgPool) -> Result<Uuid> {
        let uuid = Uuid::new_v4();
        query!(
            "
INSERT INTO todo_item (uuid, title, description, deadline, created_at) VALUES ($1, $2, $3, $4, $5)
",
            Uuid::new_v4(),
            self.title,
            self.description,
            self.deadline,
            self.created_at,
        )
        .fetch_all(connection)
        .await?;
        Ok(uuid)
    }
}
impl IntoTodoItem for CreatedTodoItem {
    fn into_todo_item(&self) -> super::TodoItem {
        TodoItem {
            id: None,
            uuid: Uuid::new_v4(),
            title: self.title.clone(),
            description: self.description.clone(),
            completed: self.completed,
            deadline: self.deadline,
            created_at: self.created_at,
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}
