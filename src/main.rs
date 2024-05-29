use std::{env, process::exit, thread, time::Duration};

use clap::{Args, Parser, Subcommand};
use cli::{Cli, Command};
use color_eyre::Result;
use console::{style, Term};
use database::Database;
use dotenv::dotenv;
use todo::todo_item::CreatedTodoItem;

use crate::{
    cli::dialogue::create_item_dialogue,
    todo::{TodoItem, TodoList},
};

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

    Cli::indicatif_test(10);
    //Cli::dialoguer_test();
    let item = create_item_dialogue()?;
    println!("{item:#?}");
    exit(0);

    let mut database: Database = Database::new(&env::var("DATABASE_URL")?).await?;
    let item = CreatedTodoItem::new("Hello", Some("This is a test"), None);
    //database.add_item(&item).await?;

    match Cli::get_command() {
        Command::Add { item } => println!("Should add item {item}"),
        Command::List => {
            database
                .retrieve_data()
                .await?
                .get_items()
                .iter()
                .for_each(|item| println!("{item:#?}"));
        }
        Command::Debug => println!("Debug command"),
    }

    Ok(())
}
