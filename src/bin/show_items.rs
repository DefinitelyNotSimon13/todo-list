use color_eyre::Result;
use diesel::prelude::*;
use todo_list::database::Database;
use todo_list::models::*;
use todo_list::*;

fn main() -> Result<()> {
    use todo_list::schema::todo_items::dsl::*;

    let mut database = Database::new(&get_connection_string()?)?;
    let connection = database.get_connection();

    let results = todo_items
        .filter(completed.eq(true))
        .limit(5)
        .select(TodoItem::as_select())
        .load(connection)?;

    println!("Displaying {} posts", results.len());
    for item in results {
        println!("{item:#?}")
    }
    Ok(())
}
