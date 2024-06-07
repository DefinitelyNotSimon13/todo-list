use chrono::{Local, NaiveDateTime};

use crate::models::{TodoItem, UpdatedTodoItem};
use color_eyre::Result;
use dialoguer::{theme::ColorfulTheme, Confirm};

use super::dialogue_pieces::{
    input_with_date_validation, input_with_time_validation, input_without_validation,
};

pub fn update_item_dialogue(todo_item: TodoItem) -> Result<UpdatedTodoItem> {
    let theme = &ColorfulTheme::default();
    let mut updated_item = UpdatedTodoItem::from(todo_item);

    let title = input_without_validation("Title", updated_item.get_title(), false)?;
    updated_item.update_title(&title);

    let description = input_without_validation(
        "Description",
        updated_item.get_description().unwrap_or(""),
        true,
    )?;
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
    let initial_date = updated_item
        .get_deadline()
        .map_or("".to_string(), |deadline| {
            format!("{}", deadline.format("%d.%m.%Y"))
        });
    let date = input_with_date_validation("Date (dd.mm.yyyy)", &initial_date, true)?;

    let initial_time = updated_item
        .get_deadline()
        .map_or("".to_string(), |deadline| {
            format!("{}", deadline.format("%H:%M"))
        });
    let time = input_with_time_validation("Time (hh:mm)", &initial_time, false)?;

    let deadline = if date.is_empty() {
        None
    } else {
        let deadline_string = format!("{} {}", date, time);
        Some(
            NaiveDateTime::parse_from_str(&deadline_string, "%d.%m.%Y %H:%M:%S")?
                .and_local_timezone(Local)
                .unwrap()
                .to_utc(),
        )
    };
    updated_item.update_deadline(deadline);
    Ok(updated_item)
}
