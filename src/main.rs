#[tokio::main]
async fn main() {
    ruhn::run().await;
}


// add multithreading
// use tokio::time::{sleep, Duration};

// async fn async_task(id: u32) {
//     println!("Task {} started", id);

//     // Simulate some asynchronous work
//     sleep(Duration::from_secs(2)).await;

//     println!("Task {} completed", id);
// }

// #[tokio::main]
// async fn main() {
//     // Spawn multiple async tasks onto the tokio runtime
//     let task1 = tokio::spawn(async_task(1));
//     let task2 = tokio::spawn(async_task(2));
//     let task3 = tokio::spawn(async_task(3));

//     // Wait for all tasks to complete
//     tokio::try_join!(task1, task2, task3).unwrap();
// }

