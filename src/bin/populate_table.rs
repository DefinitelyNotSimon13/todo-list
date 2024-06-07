use std::sync::Arc;

use color_eyre::eyre::Result;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenvy::dotenv;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use todo_list::database::Database;
use todo_list::get_connection_string;
use todo_list::models::NewTodoItem;
use todo_list::todo::TodoList;

use todo_list::DbPool;

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

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let database_url = get_connection_string().unwrap();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("failed to create pool");

    //let pool = Arc::new(pool);

    let tasks: Vec<String> = (0..=500_000)
        .map(|i| format!("{} {}", generate_todo_item(), i))
        .collect();

    //let mut handles = vec![];
    //let batch_size = 10_000;
    tasks.par_iter().for_each(|task| {
        println!("Adding task: {task}");
        TodoList::add_item_static(
            &mut pool.get().unwrap(),
            NewTodoItem::new(task.to_string(), None, None),
        )
    });
    Ok(())
}
