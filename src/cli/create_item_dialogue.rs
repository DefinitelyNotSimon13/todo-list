use crate::models::NewTodoItem;
use chrono::{Local, NaiveDateTime};
use color_eyre::Result;

use super::{
    confirm,
    dialogue_pieces::{
        input_with_date_validation, input_with_time_validation, input_without_validation,
    },
};

pub fn create_item_dialogue() -> Result<NewTodoItem> {
    let title = input_without_validation("Title", "", false)?;
    let description = input_without_validation("Description", "", true)?;
    let description = if description.is_empty() {
        None
    } else {
        Some(description)
    };

    if !confirm("Do you want to set a deadline?") {
        return Ok(NewTodoItem::new(title, description, None));
    }

    let date = input_with_date_validation("Date (dd.mm.yyyy)", "", false)?;
    let time = input_with_time_validation("Time (hh:mm)", "", false)?;

    let deadline_string = format!("{} {}:00", date, time);
    let deadline = NaiveDateTime::parse_from_str(&deadline_string, "%d.%m.%Y %H:%M:%S")?
        .and_local_timezone(Local)
        .unwrap()
        .to_utc();

    Ok(NewTodoItem::new(title, description, Some(deadline)))
}
