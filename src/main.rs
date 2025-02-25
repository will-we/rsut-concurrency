use std::sync::Arc;

#[tokio::main]
async fn main() {
    let data = Arc::new(vec![1, 2, 3]);

    let handle = tokio::spawn({
        let data = Arc::clone(&data);
        async move {
            println!("子任务访问数据: {:?}", data);
        }
    });
    handle.await.unwrap();
}