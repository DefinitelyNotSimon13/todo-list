use color_eyre::Result;
use std::io::{stdin, Read};
use todo_list::{cli::create_item_dialogue::create_item_dialogue, database::Database, *};

fn main() -> Result<()> {
    let mut database = Database::new(&get_connection_string()?)?;
    let mut connection = database.get_connection();

    let new_item = create_item_dialogue()?;

    let item = create_item(&mut connection, new_item)?;
    println!("\nSaved {} with id {}", item.title, item.uuid);
    Ok(())
}
