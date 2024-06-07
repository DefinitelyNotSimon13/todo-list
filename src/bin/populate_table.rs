use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::process::exit;
use std::sync::Arc;
use std::time::Duration;

use color_eyre::eyre::Result;
use diesel::r2d2::ConnectionManager;
use diesel::{Connection, PgConnection, QueryDsl, RunQueryDsl};
use dotenvy::dotenv;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use todo_list::get_connection_string;
use todo_list::models::NewTodoItem;
use todo_list::schema::todo_items::{self, id};
use todo_list::todo::TodoList;
use tokio::sync::Semaphore;
use tokio::task;

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
        .max_size(20)
        .build(manager)
        .expect("failed to create pool");

    let pool = Arc::new(pool);

    let amount = 10_000;
    let tasks: Vec<String> = (0..amount)
        .map(|i| format!("{} {}", generate_todo_item(), i))
        .collect();

    let mut handles = vec![];
    let batch_size = 1_000;
    let max_concurrent_tasks = 20;
    let semaphore = Arc::new(Semaphore::new(max_concurrent_tasks));

    let chunks = tasks.chunks(batch_size);

    for task_batch in chunks {
        let permit = semaphore.clone().acquire_owned().await;
        let pool = Arc::clone(&pool);
        let task_batch = task_batch.to_vec();

        let handle = tokio::spawn(async move {
            task::spawn_blocking(move || {
                let mut conn = pool.get().expect("failed to retrieve connection");
                conn.transaction::<_, diesel::result::Error, _>(|conn| {
                    let new_items: Vec<NewTodoItem> = task_batch
                        .into_iter()
                        .map(|task| NewTodoItem::new(task, None, None))
                        .collect();
                    diesel::insert_into(todo_items::table)
                        .values(&new_items)
                        .execute(conn)
                })
                .expect("error inserting items");
                drop(permit);
            })
            .await
            .expect("task failed");
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("Tried to insert {}", amount,);

    Ok(())
}
