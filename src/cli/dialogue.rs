use chrono::{Local, NaiveDateTime};
use std::fmt;
use time::{Month, OffsetDateTime, Time};

use color_eyre::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use regex::Regex;
use sqlx::types::chrono::{DateTime, Utc};

use crate::todo::todo_item::CreatedTodoItem;

pub fn create_item_dialogue() -> Result<CreatedTodoItem> {
    let theme = &ColorfulTheme::default();

    let title: String = Input::with_theme(theme)
        .with_prompt("Title")
        .interact_text()?;

    let description: String = Input::with_theme(theme)
        .with_prompt("Description")
        .allow_empty(true)
        .interact_text()?;

    let description: Option<String> = if description.is_empty() {
        None
    } else {
        Some(description)
    };

    if !Confirm::with_theme(theme)
        .with_prompt("Do you want to set a deadline?")
        .interact()?
    {
        return Ok(CreatedTodoItem::new(&title, description.as_deref(), None));
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

    let time: String = Input::with_theme(theme)
        .with_prompt("Time (hh:mm)")
        .validate_with({
            let time_regex = Regex::new(r"^(?:[01]\d|2[0-3]):[0-5]\d$").unwrap();
            move |input: &String| -> Result<(), &str> {
                if time_regex.is_match(input) {
                    Ok(())
                } else {
                    Err("Not a valid time!")
                }
            }
        })
        .default("23:59".to_string())
        .interact_text()?;

    let datetime_str = format!("{} {}", date_input, time);
    let datetime =
        NaiveDateTime::parse_from_str(&datetime_str, "%d.%m.%Y %H:%M")?.and_local_timezone(Local);
    let datetime = match datetime {
        chrono::offset::LocalResult::Single(dt) => dt.to_utc(),
        _ => panic!("Unexpected error while parsing datetime"),
    };
    //let datetime = OffsetDateTimes

    Ok(CreatedTodoItem::new(
        &title,
        description.as_deref(),
        Some(datetime),
    ))
}
