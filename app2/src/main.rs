mod controllers;
mod models;
mod router;
mod services;
mod views;

use router::Router;
use shared::greet;

#[tokio::main]
async fn main() {
    println!("=== App2 启动 (MVC 架构) ===");
    greet("App2");

    // 创建并启动路由器
    let router = Router::new("127.0.0.1".to_string(), 3000);

    if let Err(e) = router.start().await {
        eprintln!("❌ 服务器错误: {}", e);
        std::process::exit(1);
    }
}
