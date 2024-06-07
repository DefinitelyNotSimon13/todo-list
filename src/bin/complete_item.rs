use self::models::TodoItem;
use color_eyre::Result;
use diesel::prelude::*;
use std::env::args;
use todo_list::{database::Database, *};

fn main() -> Result<()> {
    use self::schema::todo_items::dsl::{completed, todo_items};

    let id = args()
        .nth(1)
        .expect("complete_item requires an item id")
        .parse::<i32>()
        .expect("Invalid ID");

    let mut database = Database::new(&get_connection_string()?)?;
    let connection = database.get_connection();

    let item = diesel::update(todo_items.find(id))
        .set(completed.eq(true))
        .returning(TodoItem::as_returning())
        .get_result(connection)?;

    println!("Completed task {}", item.title);
    Ok(())
}
