use chrono::{Local, NaiveDateTime};

use crate::{
    models::{NewTodoItem, TodoItem},
    todo::TodoList,
};
use color_eyre::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect, Input};
use regex::Regex;

pub fn create_item_dialogue() -> Result<NewTodoItem> {
    let theme = &ColorfulTheme::default();
    let title: String = Input::with_theme(theme)
        .with_prompt("Title")
        .interact_text()?;

    let description: String = Input::with_theme(theme)
        .with_prompt("Description")
        .allow_empty(true)
        .interact()?;

    let description: Option<String> = if description.is_empty() {
        None
    } else {
        Some(description)
    };

    if !Confirm::with_theme(theme)
        .with_prompt("Do you want to set a deadline?")
        .interact()?
    {
        return Ok(NewTodoItem::new(title, description, None));
    }

    let date_input: String = Input::with_theme(theme)
        .with_prompt("Date (dd.mm.yyyy)")
        .validate_with({
            let date_regex =
                Regex::new(r"^(0[1-9]|[12][0-9]|3[01])\.(0[1-9]|1[012])\.\d{4}$").unwrap();
            move |input: &String| -> Result<(), &str> {
                if date_regex.is_match(input) {
                    Ok(())
                } else {
                    Err("Not a valid date!")
                }
            }
        })
        .interact_text()?;

    let date_time_input = format!("{} 00:00:00", date_input);
    let date_time = NaiveDateTime::parse_from_str(&date_time_input, "%d.%m.%Y %H:%M:%S")?
        .and_local_timezone(Local)
        .unwrap()
        .to_utc();

    Ok(NewTodoItem::new(title, description, Some(date_time)))
}

