use std::env;

use cli::{dialogue::create_item_dialogue, Cli, Command};
use color_eyre::Result;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};
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
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth)
        .set_header(vec![
            "Title",
            "Description",
            "Deadline",
            "Created",
            "Last Updated",
        ]);

    todo_list.get_items().iter().for_each(|item| {
        let item = item.into_local();
        table.add_row(vec![
            item.title.clone(),
            item.description
                .clone()
                .or(Some("None".to_owned()))
                .expect("error getting description"),
            match item.deadline {
                Some(deadline) => deadline.to_string(),
                None => "None".to_string(),
            },
            item.created_at.to_string(),
            item.updated_at.to_string(),
        ]);
    });
    println!("{table}");
}

async fn add_item(todo_list: &mut TodoList<'_>) -> Result<()> {
    let item = create_item_dialogue()?;
    let item = todo_list.add_item(item).await?;
    println!("Added item {item:#?}");
    Ok(())
}
