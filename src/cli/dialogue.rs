use std::fmt;
use time::{Month, OffsetDateTime, Time};

use color_eyre::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use regex::Regex;
use sqlx::types::time::Date;

use crate::todo::todo_item::CreatedTodoItem;

pub fn create_item_dialogue() -> Result<CreatedTodoItem> {
    let theme = &ColorfulTheme::default();
    let title: String = Input::with_theme(theme)
        .with_prompt("Title")
        .interact_text()?;

    let description: String = Input::with_theme(theme)
        .with_prompt("Description")
        .allow_empty(true)
        .interact()?;

    let description: Option<&str> = if description.is_empty() {
        None
    } else {
        Some(&description)
    };

    if !Confirm::with_theme(theme)
        .with_prompt("Do you want to set a deadline?")
        .interact()?
    {
        return Ok(CreatedTodoItem::new(&title, description, None));
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

    let date_parts: Vec<&str> = date_input.split('.').collect();

    let year: i32 = date_parts[2].parse()?;
    let month: u8 = date_parts[1].parse()?;
    let day: u8 = date_parts[0].parse()?;

    let month = match month {
        1 => Month::January,
        2 => Month::February,
        3 => Month::March,
        4 => Month::April,
        5 => Month::May,
        6 => Month::June,
        7 => Month::July,
        8 => Month::August,
        9 => Month::September,
        10 => Month::October,
        11 => Month::November,
        12 => Month::December,
        _ => panic!("Unchecked month"),
    };

    let date = Date::from_calendar_date(year, month, day)?;
    println!("{description:#?}");

    let time = Time::from_hms(00, 00, 00)?;

    Ok(CreatedTodoItem::new(
        &title,
        description,
        Some(OffsetDateTime::new_utc(date, time)),
    ))
}
