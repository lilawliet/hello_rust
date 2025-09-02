use std::sync::Arc;
use tokio::sync::RwLock;

// 线程安全的异步数据存储
struct AsyncDataStore {
    data: Arc<RwLock<Vec<String>>>,
}

impl AsyncDataStore {
    fn new() -> Self {
        AsyncDataStore {
            data: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    async fn write(&self, item: String) {
        let mut data = self.data.write().await;
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        data.push(item);
        println!("写入: {}", data.len());
    }
    
    async fn read(&self) -> Vec<String> {
        let data = self.data.read().await;
        data.clone()
    }
}

async fn rust_concurrency() {
    let store = Arc::new(AsyncDataStore::new());
    
    // 并发写入
    let mut write_handles = vec![];
    for i in 0..5 {
        let store = Arc::clone(&store);
        let handle = tokio::spawn(async move {
            store.write(format!("数据 {}", i)).await;
        });
        write_handles.push(handle);
    }
    
    // 等待所有写入完成
    for handle in write_handles {
        handle.await.unwrap();
    }
    
    // 读取数据
    let data = store.read().await;
    println!("读取到数据: {:?}", data);
}

// 使用 rayon 进行并行计算
use rayon::prelude::*;

fn parallel_computation() {
    let numbers: Vec<i32> = (0..1_000_000).collect();
    
    let start = std::time::Instant::now();
    
    // 并行计算
    let sum: i64 = numbers.par_iter()
        .map(|&x| x as i64)
        .sum();
    
    let duration = start.elapsed();
    println!("并行计算耗时: {:?}, 结果: {}", duration, sum);
}

#[tokio::main]
async fn main() {
    // 运行异步并发示例
    rust_concurrency().await;
    
    // 运行并行计算示例
    parallel_computation();
}