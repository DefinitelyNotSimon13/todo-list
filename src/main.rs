use std::env;

use cli::{dialogue::create_item_dialogue, Cli, Command};
use color_eyre::Result;
use console::{style, Term};
use database::Database;
use dotenv::dotenv;

use crate::todo::TodoList;

mod cli;
mod database;
mod todo;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenv().ok();
    let term = Term::stdout();
    term.clear_screen()?;
    println!("Hello, {}!", style("World").cyan().bold());

    //Cli::indicatif_test(10);
    //Cli::dialoguer_test();
    //let item = create_item_dialogue()?;
    //println!("{item:#?}");
    let database: Database = Database::new(&env::var("DATABASE_URL")?).await?;

    let mut todo_list = TodoList::new(Vec::default(), &database);
    todo_list.read().await?;

    match Cli::get_command() {
        Command::Add => add_item(&mut todo_list).await?,
        Command::List => list_items_dbg(&todo_list),
        Command::Debug => println!("Debug command"),
    }

    Ok(())
}

fn list_items_dbg(todo_list: &TodoList) {
    todo_list
        .get_items()
        .iter()
        .for_each(|item| println!("{item:#?}"));
}

async fn add_item(todo_list: &mut TodoList<'_>) -> Result<()> {
    let item = create_item_dialogue()?;
    let item = todo_list.add_item(item).await?;
    println!("Added item {item:#?}");
    Ok(())
}
