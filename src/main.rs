use color_eyre::Result;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};
use console::{style, Term};
use todo_list::cli;
use todo_list::cli::update_item_dialogue::update_item_dialogue;
use todo_list::database::Database;
use todo_list::models::NewTodoItem;
use todo_list::{
    cli::{create_item_dialogue::create_item_dialogue, fuzzy_select_item, Cli, Command},
    get_connection_string,
    todo::TodoList,
};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let mut database = Database::new(&get_connection_string()?)?;

    let term = Term::stdout();
    term.clear_screen()?;

    let mut todo_list = TodoList::new(&mut database);

    match Cli::get_command() {
        Command::Add { title } => add_item(&mut todo_list, title)?,
        Command::Update => update_item(&mut todo_list)?,
        Command::List { all } => list_items_dbg(&mut todo_list, all)?,
        Command::Search { title } => search_item_dbg(&mut todo_list, title)?,
        Command::Delete { title } => delete_item(&mut todo_list, title)?,
        Command::Complete => complete_item(&mut todo_list)?,
        Command::Debug => println!("Debug command"),
    }

    Ok(())
}

fn delete_item(todo_list: &mut TodoList, title: Option<String>) -> Result<()> {
    let items = match title {
        None => {
            let id = fuzzy_select_item(todo_list, true, "Select item to delete")?;
            vec![todo_list.get_item_with_id(id)?]
        }
        Some(title) => todo_list.get_item_with_title(&title)?,
    };

    println!("Trying to delete {} items", items.len());
    items.iter().for_each(|item| {
        if cli::confirm(&format!(
            "Are you sure you want to delete \"{}\"?",
            item.title
        )) {
            todo_list
                .delete_item_with_id(item.id)
                .expect("failed to delete item");
            println!("Item deleted");
        }
    });

    Ok(())
}

fn update_item(todo_list: &mut TodoList) -> Result<()> {
    let id = fuzzy_select_item(todo_list, true, "Select item to update")?;
    let updated_item = update_item_dialogue(todo_list.get_item_with_id(id)?)?;
    let item = todo_list.update_item(id, updated_item)?;
    println!("Updated item: {}", item.title);
    Ok(())
}

fn complete_item(todo_list: &mut TodoList) -> Result<()> {
    let id = fuzzy_select_item(todo_list, false, "Select item to complete")?;
    let item = todo_list.complete_item(id).unwrap();
    println!("Completed item: {}", item.title);
    Ok(())
}

fn search_item_dbg(todo_list: &mut TodoList, title: Option<String>) -> Result<()> {
    let items = match title {
        None => {
            let id = fuzzy_select_item(todo_list, true, "Debug selection").unwrap();
            vec![todo_list.get_item_with_id(id)?]
        }
        Some(ref title) => todo_list.get_item_with_title(&title)?,
    };
    if items.is_empty() {
        println!(
            "No item with title like \"{}\" found!",
            title.expect("error bad")
        );
    } else {
        items
            .iter()
            .for_each(|item| println!("{}:\n{item:#?}", style(item.title.clone()).cyan().bold()));
        println!(
            "\nShowing a total of {} results",
            style(items.len()).blue().bold()
        )
    }
    Ok(())
}

fn list_items_dbg(todo_list: &mut TodoList, get_all: bool) -> Result<()> {
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

    todo_list.get_items(get_all)?.iter().for_each(|item| {
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
    Ok(())
}

fn add_item(todo_list: &mut TodoList<'_>, title: Option<String>) -> Result<()> {
    let item = match title {
        Some(title) => NewTodoItem::new(title, None, None),
        None => create_item_dialogue()?,
    };
    let item = todo_list.add_item(item);
    println!("Added item {item:#?}");
    Ok(())
}
