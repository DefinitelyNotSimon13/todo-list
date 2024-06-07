use chrono::{Local, NaiveDateTime};

use crate::{
    models::{NewTodoItem, TodoItem, UpdatedTodoItem},
    todo::TodoList,
};
use color_eyre::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect, Input};
use regex::Regex;

pub fn update_item_dialogue(todo_item: TodoItem) -> Result<UpdatedTodoItem> {
    let theme = &ColorfulTheme::default();
    let mut updated_item = UpdatedTodoItem::from(todo_item);

    let title: String = Input::with_theme(theme)
        .with_prompt("Title")
        .with_initial_text(updated_item.get_title())
        .interact_text()?;

    updated_item.update_title(&title);

    let description: String = Input::with_theme(theme)
        .with_prompt("Description")
        .with_initial_text(match updated_item.get_description() {
            Some(desc) => desc,
            None => "",
        })
        .allow_empty(true)
        .interact_text()?;

    let description: Option<String> = if description.is_empty() {
        None
    } else {
        Some(description)
    };
    updated_item.update_description(description);

    if updated_item.get_deadline().is_none()
        && !Confirm::with_theme(theme)
            .with_prompt("Do you want to add a deadline?")
            .interact()?
    {
        return Ok(updated_item);
    }
    let date_input: String = Input::with_theme(theme)
        .with_prompt("Date (dd.mm.yyyy)")
        .with_initial_text(match updated_item.get_deadline() {
            Some(deadline) => format!("{}", deadline.format("%d.%m.%Y")),
            None => "".to_string(),
        })
        .allow_empty(true)
        .validate_with({
            let date_regex =
                Regex::new(r"^(0[1-9]|[12][0-9]|3[01])\.(0[1-9]|1[012])\.\d{4}$").unwrap();
            move |input: &String| -> Result<(), &str> {
                if input.is_empty() || date_regex.is_match(input) {
                    Ok(())
                } else {
                    Err("Not a valid date!")
                }
            }
        })
        .interact_text()?;

    let deadline = if date_input.is_empty() {
        None
    } else {
        let date_time_input = format!("{} 00:00:00", date_input);
        Some(
            NaiveDateTime::parse_from_str(&date_time_input, "%d.%m.%Y %H:%M:%S")?
                .and_local_timezone(Local)
                .unwrap()
                .to_utc(),
        )
    };
    updated_item.update_deadline(deadline);
    Ok(updated_item)
}
