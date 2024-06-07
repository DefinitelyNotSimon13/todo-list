use self::models::TodoItem;
use color_eyre::Result;
use diesel::prelude::*;
use std::env::args;
use todo_list::{database::Database, *};

fn main() -> Result<()> {
    use self::schema::todo_items::dsl::todo_items;

    let id = args()
        .nth(1)
        .expect("get_item requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    let mut database = Database::new(&get_connection_string()?)?;
    let mut connection = database.get_connection();

    let item = todo_items
        .find(id)
        .select(TodoItem::as_select())
        .first(&mut connection)
        .optional()?;

    match item {
        Some(item) => println!("Item with id: {} - title: {}", item.id, item.title),
        None => println!("Unable to find item with id: {}", id),
    }
    Ok(())
}
