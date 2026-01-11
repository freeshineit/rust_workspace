use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode, Method};
use hyper::body::{Bytes, Incoming};
use http_body_util::{Full, BodyExt};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};

/// 服务器配置
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
        }
    }
}

/// API 响应结构
#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(message: impl Into<String>, data: T) -> Self {
        Self {
            success: true,
            message: message.into(),
            data: Some(data),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            data: None,
        }
    }
}

/// 健康检查数据
#[derive(Serialize)]
pub struct HealthCheck {
    pub status: String,
    pub version: String,
}

/// 启动 HTTP 服务器
pub async fn start_server(config: ServerConfig) -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    let listener = TcpListener::bind(addr).await?;
    
    println!("🚀 服务器启动成功！");
    println!("📍 监听地址: http://{}", addr);
    println!("📝 可用端点:");
    println!("   GET  /          - 欢迎页面");
    println!("   GET  /health    - 健康检查");
    println!("   GET  /api/hello - Hello API");
    println!("   POST /api/echo  - Echo API");
    println!("\n按 Ctrl+C 停止服务器\n");

    loop {
        let (stream, client_addr) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(|req| handle_request(req, client_addr)))
                .await
            {
                eprintln!("❌ 处理连接时出错: {:?}", err);
            }
        });
    }
}

/// 处理 HTTP 请求
async fn handle_request(
    req: Request<Incoming>,
    client_addr: SocketAddr,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let method = req.method();
    let path = req.uri().path();
    
    println!("📨 {} {} - 来自 {}", method, path, client_addr);

    let response = match (method, path) {
        (&Method::GET, "/") => handle_root(),
        (&Method::GET, "/health") => handle_health(),
        (&Method::GET, "/api/hello") => handle_hello(),
        (&Method::POST, "/api/echo") => handle_echo(req).await,
        _ => handle_not_found(),
    };

    Ok(response)
}

/// 处理根路径
fn handle_root() -> Response<Full<Bytes>> {
    let html = r#"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>App2 Server</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            color: #333;
        }
        .container {
            background: white;
            border-radius: 20px;
            padding: 3rem;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
            max-width: 600px;
            width: 90%;
        }
        h1 {
            color: #667eea;
            margin-bottom: 1rem;
            font-size: 2.5rem;
        }
        p { margin-bottom: 1rem; line-height: 1.6; }
        .endpoints {
            background: #f8f9fa;
            border-radius: 10px;
            padding: 1.5rem;
            margin-top: 2rem;
        }
        .endpoint {
            margin: 0.5rem 0;
            font-family: 'Courier New', monospace;
            padding: 0.5rem;
            background: white;
            border-radius: 5px;
            border-left: 4px solid #667eea;
        }
        .method {
            color: #667eea;
            font-weight: bold;
            margin-right: 0.5rem;
        }
        .badge {
            display: inline-block;
            background: #667eea;
            color: white;
            padding: 0.25rem 0.75rem;
            border-radius: 20px;
            font-size: 0.875rem;
            margin-bottom: 1rem;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="badge">🦀 Rust + Tokio</div>
        <h1>欢迎使用 App2 Server</h1>
        <p>这是一个使用 Tokio 和 Hyper 构建的异步 HTTP 服务器。</p>
        
        <div class="endpoints">
            <h3 style="margin-bottom: 1rem; color: #667eea;">📡 可用端点</h3>
            <div class="endpoint">
                <span class="method">GET</span>
                <span>/health</span>
            </div>
            <div class="endpoint">
                <span class="method">GET</span>
                <span>/api/hello</span>
            </div>
            <div class="endpoint">
                <span class="method">POST</span>
                <span>/api/echo</span>
            </div>
        </div>
    </div>
</body>
</html>
    "#;

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(Full::new(Bytes::from(html)))
        .unwrap()
}

/// 处理健康检查
fn handle_health() -> Response<Full<Bytes>> {
    let health = HealthCheck {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    
    let response = ApiResponse::success("服务器运行正常", health);
    json_response(StatusCode::OK, &response)
}

/// 处理 Hello API
fn handle_hello() -> Response<Full<Bytes>> {
    #[derive(Serialize)]
    struct HelloData {
        message: String,
        timestamp: String,
    }
    
    let data = HelloData {
        message: "Hello from App2 Server!".to_string(),
        timestamp: chrono::Local::now().to_rfc3339(),
    };
    
    let response = ApiResponse::success("请求成功", data);
    json_response(StatusCode::OK, &response)
}

/// 处理 Echo API
async fn handle_echo(req: Request<Incoming>) -> Response<Full<Bytes>> {
    #[derive(Deserialize)]
    struct EchoRequest {
        message: String,
    }
    
    #[derive(Serialize)]
    struct EchoData {
        echo: String,
        length: usize,
    }
    
    // 读取请求体
    let body = match req.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(_) => {
            let response: ApiResponse<()> = ApiResponse::error("无法读取请求体");
            return json_response(StatusCode::BAD_REQUEST, &response);
        }
    };
    
    // 解析 JSON
    let echo_req: EchoRequest = match serde_json::from_slice(&body) {
        Ok(req) => req,
        Err(_) => {
            let response: ApiResponse<()> = ApiResponse::error("无效的 JSON 格式");
            return json_response(StatusCode::BAD_REQUEST, &response);
        }
    };
    
    let data = EchoData {
        length: echo_req.message.len(),
        echo: echo_req.message,
    };
    
    let response = ApiResponse::success("Echo 成功", data);
    json_response(StatusCode::OK, &response)
}

/// 处理 404
fn handle_not_found() -> Response<Full<Bytes>> {
    let response: ApiResponse<()> = ApiResponse::error("端点未找到");
    json_response(StatusCode::NOT_FOUND, &response)
}

/// 创建 JSON 响应
fn json_response<T: Serialize>(status: StatusCode, data: &T) -> Response<Full<Bytes>> {
    let json = serde_json::to_string(data).unwrap();
    
    Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(Full::new(Bytes::from(json)))
        .unwrap()
}
