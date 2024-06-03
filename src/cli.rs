use core::fmt;
use std::{cmp::min, fmt::Write, thread, time::Duration};

use clap::{command, Parser, Subcommand};
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm, Editor, FuzzySelect, Input};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};

pub mod dialogue;

#[derive(Debug, Parser)]
#[command(name = "todo")]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Add,
    List,
    Debug,
}

impl Cli {
    pub fn get_command() -> Command {
        Self::parse().command
    }

    pub fn indicatif_test(total: u64) {
        let mut finished = 0;
        let progress_bar = ProgressBar::new(total);
        progress_bar.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] (eta: {eta})",
            )
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
                write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
            })
            .progress_chars("#>-"),
        );
        while finished < total {
            let new = min(finished + 1, total);
            finished = new;
            progress_bar.set_position(new);
            thread::sleep(Duration::from_millis(10));
        }
        println!("{}", style("Application Ready!").green().bold());
    }

    pub fn dialoguer_test() {
        let selection = &["Critical", "Not important", "School"];
        let theme = &ColorfulTheme::default();

        if !Confirm::with_theme(theme)
            .with_prompt("Do you want to continue?")
            .interact()
            .unwrap()
        {
            return;
        }

        let task_name: String = Input::with_theme(theme)
            .with_prompt("Task Name")
            .interact_text()
            .unwrap();

        let task_description: String = Input::with_theme(theme)
            .with_prompt("Description")
            .interact_text()
            .unwrap();

        let task_category = FuzzySelect::with_theme(theme)
            .with_prompt("Category")
            .default(0)
            .items(&selection[..])
            .interact()
            .unwrap();
    }
}
