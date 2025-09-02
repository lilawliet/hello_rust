use std::time::Duration;
use tokio;

// 多线程版本 - 适合 CPU 密集型任务
fn cpu_intensive_task() -> i32 {
    // 模拟 CPU 密集型计算
    let mut result = 0i32;
    for i in 0..100_000 {  // 减少循环次数避免溢出
        result = result.wrapping_add(i);  // 使用wrapping_add避免溢出panic
    }
    result
}

fn multi_threaded_example() {
    let start = std::time::Instant::now();
    
    let handles: Vec<_> = (0..4).map(|_| {
        std::thread::spawn(|| {
            cpu_intensive_task()
        })
    }).collect();
    
    let results: Vec<i32> = handles.into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    
    let duration = start.elapsed();
    println!("多线程耗时: {:?}, 结果: {:?}", duration, results);
}

// 异步版本 - 适合 I/O 密集型任务
async fn io_intensive_task() -> String {
    // 模拟 I/O 操作
    tokio::time::sleep(Duration::from_millis(100)).await;
    String::from("I/O 任务完成")
}

async fn async_example() {
    let start = std::time::Instant::now();
    
    let handles: Vec<_> = (0..100).map(|_| {
        tokio::spawn(async {
            io_intensive_task().await
        })
    }).collect();
    
    let results: Vec<String> = futures::future::join_all(handles).await
        .into_iter()
        .map(|r| r.unwrap())
        .collect();
    
    let duration = start.elapsed();
    println!("异步耗时: {:?}, 处理了 {} 个任务", duration, results.len());
}

#[tokio::main]
async fn main() {
    // 比较两种方法
    multi_threaded_example();
    async_example().await;
}