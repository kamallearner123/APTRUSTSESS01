use tokio::time::{sleep, Duration};

// Asynchronous function that simulates some work
async fn perform_task(task_name: &str) {
    println!("Task '{}' started!", task_name);
    
    // Simulating some asynchronous work with sleep
    sleep(Duration::from_secs(2)).await;
    
    println!("Task '{}' completed!", task_name);
}

// Main asynchronous function
#[tokio::main]
async fn main() {
    // Running multiple tasks concurrently
    let task1 = perform_task("Task 1");
    let task2 = perform_task("Task 2");
    
    // Wait for both tasks to finish concurrently
    //tokio::join!(task1, task2);

    task1.await();

    println!("All tasks completed!");
}
