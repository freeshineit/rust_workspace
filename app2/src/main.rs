use shared::greet;

#[tokio::main]
async fn main() {
    println!("App2 启动!");
    greet("App2");
    
    println!("执行 App2 异步逻辑...");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("App2 完成!");
}
