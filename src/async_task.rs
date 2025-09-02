use tokio;
use std::time::Duration;


async fn process_task(id: i32) -> String {
    tokio::time::sleep(Duration::from_millis(100)).await;
    format!("Task {} completed", id)
}

async fn process_all_tasks() -> Vec<String> {
    let mut handles = vec![];
    for i in 0..10 {
        let handle = tokio::spawn(async move {
            process_task(i).await
        });
        handles.push(handle);
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await.unwrap());
    }
    results
}

#[tokio::main]
async fn main() {
    let results = process_all_tasks().await;
    println!("All tasks completed: {:?}", results);
}
