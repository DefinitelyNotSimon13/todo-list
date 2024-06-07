use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::process::Command;

// Function to generate a random TODO item name
fn generate_todo_item() -> String {
    let items = vec![
        "Buy groceries",
        "Read a book",
        "Write a report",
        "Call mom",
        "Walk the dog",
        "Go to the gym",
        "Finish homework",
        "Clean the house",
        "Pay bills",
        "Plan vacation",
        "Send emails",
        "Attend meeting",
        "Study Rust",
        "Update resume",
        "Organize files",
        "Cook dinner",
        "Water plants",
        "Watch a movie",
        "Fix the bike",
        "Write a blog post",
        "Lernen",
        "Test",
        "DeadlineTest",
        "Nach Hause gehen",
    ];
    let mut rng = thread_rng();
    items.choose(&mut rng).unwrap().to_string()
}

fn main() {
    // Generate 100 tasks
    let tasks: Vec<String> = (31_347..=5_000_000)
        .map(|i| format!("{} {}", generate_todo_item(), i))
        .collect();

    // Execute the tasks in parallel using rayon
    tasks.par_iter().for_each(|task| {
        println!("Running: ./target/release/todo-list add \"{}\"", task);
        Command::new("./target/release/todo-list")
            .arg("add")
            .arg(task)
            .status()
            .expect("Failed to execute command");
    });
}
