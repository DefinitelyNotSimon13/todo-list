use core::fmt;

use crate::schema::todo_items;
use chrono::DateTime;
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = todo_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TodoItem {
    pub id: i32,
    pub uuid: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub deadline: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.title)
    }
}

#[derive(Insertable)]
#[diesel(table_name = todo_items)]
pub struct NewTodoItem {
    uuid: Uuid,
    title: String,
    description: Option<String>,
    completed: bool,
    deadline: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

impl NewTodoItem {
    pub fn new(
        title: String,
        description: Option<String>,
        deadline: Option<DateTime<Utc>>,
    ) -> Self {
        NewTodoItem {
            uuid: Uuid::new_v4(),
            title,
            description,
            completed: false,
            deadline,
            created_at: Utc::now(),
        }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = todo_items)]
#[diesel(treat_none_as_null = true)]
pub struct UpdatedTodoItem {
    title: String,
    description: Option<String>,
    deadline: Option<DateTime<Utc>>,
    updated_at: DateTime<Utc>,
}

impl UpdatedTodoItem {
    pub fn from(todo_item: TodoItem) -> Self {
        Self {
            title: todo_item.title,
            description: todo_item.description,
            deadline: todo_item.deadline,
            updated_at: Utc::now(),
        }
    }

    pub fn update_title(&mut self, new_title: &str) {
        if self.title == new_title {
            return;
        }

        self.title = new_title.to_owned();
        self.updated_at = Utc::now();
    }

    pub fn update_description(&mut self, new_description: Option<String>) {
        if self.description == new_description {
            return;
        }

        self.description = new_description;
        self.updated_at = Utc::now();
    }

    pub fn update_deadline(&mut self, new_deadline: Option<DateTime<Utc>>) {
        if self.deadline == new_deadline {
            return;
        }

        self.deadline = new_deadline;
        self.updated_at = Utc::now();
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn get_deadline(&self) -> Option<&DateTime<Utc>> {
        self.deadline.as_ref()
    }
}
