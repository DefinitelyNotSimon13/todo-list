use color_eyre::Result;

use clap::{command, Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect};

use crate::{models::TodoItem, todo::TodoList};

pub mod create_item_dialogue;
pub mod update_item_dialogue;

#[derive(Debug, Parser)]
#[command(name = "todo")]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Add {
        /// Quickly create a todo item with the given title
        title: Option<String>,
    },
    Update,
    List {
        /// Also get completed items
        #[arg(short, long)]
        all: bool,
    },
    Search {
        title: Option<String>,
    },
    Delete {
        title: Option<String>,
    },
    Complete,
    Debug,
}

impl Cli {
    pub fn get_command() -> Command {
        Self::parse().command
    }
}

pub fn fuzzy_select_item(todo_list: &mut TodoList, get_all: bool, prompt: &str) -> Result<i32> {
    let theme = &ColorfulTheme::default();
    let items = todo_list.get_items(get_all)?;

    let selection = FuzzySelect::with_theme(theme)
        .with_prompt(prompt)
        .items(&items)
        .interact()?;

    Ok(items[selection].id)
}

pub fn confirm(prompt: &str) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact()
        .expect("Error during confirm interaction")
}
