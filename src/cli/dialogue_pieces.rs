use color_eyre::{Result};
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect, Input};
use regex::Regex;

use crate::todo::TodoList;
use lazy_static::lazy_static;

lazy_static! {
    static ref THEME: ColorfulTheme = ColorfulTheme::default();
}

pub fn input_without_validation(
    prompt: &str,
    initial_text: &str,
    allow_empty: bool,
) -> Result<String> {
    Ok(default_input(prompt, initial_text, allow_empty).interact_text()?)
}

pub fn input_with_date_validation(
    prompt: &str,
    initial_text: &str,
    allow_empty: bool,
) -> Result<String> {
    Ok(default_input(prompt, initial_text, allow_empty)
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
        .interact_text()?)
}

pub fn input_with_time_validation(
    prompt: &str,
    initial_text: &str,
    allow_empty: bool,
) -> Result<String> {
    Ok(default_input(prompt, initial_text, allow_empty)
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
        .interact_text()?)
}

fn default_input<'a>(
    prompt: &'a str,
    initial_text: &'a str,
    allow_empty: bool,
) -> Input<'a, String> {
    Input::with_theme(&*THEME)
        .with_prompt(prompt)
        .with_initial_text(initial_text)
        .allow_empty(allow_empty)
}

pub fn fuzzy_select_item(todo_list: &mut TodoList, get_all: bool, prompt: &str) -> Result<i32> {
    let items = todo_list.get_items(get_all)?;

    let selection = FuzzySelect::with_theme(&*THEME)
        .with_prompt(prompt)
        .items(&items)
        .interact()?;

    Ok(items[selection].id)
}

pub fn confirm(prompt: &str) -> bool {
    Confirm::with_theme(&*THEME)
        .with_prompt(prompt)
        .interact()
        .expect("Error during confirm interaction")
}
