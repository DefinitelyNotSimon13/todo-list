use self::models::TodoItem;
use color_eyre::Result;
use diesel::prelude::*;
use std::env::args;
use todo_list::{database::Database, schema::todo_items::title, *};

fn main() -> Result<()> {
    use self::schema::todo_items::dsl::todo_items;

    let target = args().nth(1).expect("Expected a target to mach against");
    let pattern = format!("%{}%", target);

    let mut database = Database::new(&get_connection_string()?)?;
    let connection = database.get_connection();
    let num_deleted = diesel::delete(todo_items.filter(title.like(pattern))).execute(connection)?;

    println!("Deleted {} posts", num_deleted);

    Ok(())
}
