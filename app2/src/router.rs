use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::controllers::{HealthController, MessageController, PageController};
use crate::services::{HealthService, MessageService};
use crate::views::html_view::HtmlView;

/// 路由器
pub struct Router {
    host: String,
    port: u16,
    health_controller: Arc<HealthController>,
    message_controller: Arc<MessageController>,
    page_controller: Arc<PageController>,
}

impl Router {
    /// 创建新的路由器
    pub fn new(host: String, port: u16) -> Self {
        // 初始化服务层
        let health_service = HealthService::new();
        let message_service = MessageService::new();

        // 初始化控制器层
        let health_controller = Arc::new(HealthController::new(health_service));
        let message_controller = Arc::new(MessageController::new(message_service));
        let page_controller = Arc::new(PageController::new());

        Self {
            host,
            port,
            health_controller,
            message_controller,
            page_controller,
        }
    }

    /// 启动服务器
    pub async fn start(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr: SocketAddr = format!("{}:{}", self.host, self.port).parse()?;
        let listener = TcpListener::bind(addr).await?;

        println!("🚀 服务器启动成功！");
        println!("📍 监听地址: http://{}", addr);
        println!("🏗️  架构模式: MVC");
        println!("📝 可用端点:");
        println!("   GET  /          - 欢迎页面");
        println!("   GET  /health    - 健康检查");
        println!("   GET  /api/hello - Hello API");
        println!("   POST /api/echo  - Echo API");
        println!("\n按 Ctrl+C 停止服务器\n");

        let router = Arc::new(self);

        loop {
            let (stream, client_addr) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let router = Arc::clone(&router);

            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(
                        io,
                        service_fn(move |req| {
                            let router = Arc::clone(&router);
                            async move { router.handle_request(req, client_addr).await }
                        }),
                    )
                    .await
                {
                    eprintln!("❌ 处理连接时出错: {:?}", err);
                }
            });
        }
    }

    /// 处理 HTTP 请求
    async fn handle_request(
        &self,
        req: Request<Incoming>,
        client_addr: SocketAddr,
    ) -> Result<Response<Full<Bytes>>, hyper::Error> {
        let method = req.method();
        let path = req.uri().path();

        println!("📨 {} {} - 来自 {}", method, path, client_addr);

        let response = match (method, path) {
            // 页面路由
            (&Method::GET, "/") => self.page_controller.render_home(),

            // API 路由
            (&Method::GET, "/health") => self.health_controller.check_health(),
            (&Method::GET, "/api/hello") => self.message_controller.handle_hello(),
            (&Method::POST, "/api/echo") => self.message_controller.handle_echo(req).await,

            // 404
            _ => self.handle_not_found(),
        };

        Ok(response)
    }

    /// 处理 404
    fn handle_not_found(&self) -> Response<Full<Bytes>> {
        HtmlView::render_not_found()
    }
}
