use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Response, StatusCode};

/// HTML 视图渲染器
pub struct HtmlView;

impl HtmlView {
    /// 渲染首页
    pub fn render_home() -> Response<Full<Bytes>> {
        let html = include_str!("../../templates/home.html");
        Self::render_html(StatusCode::OK, html)
    }

    /// 渲染 404 页面
    pub fn render_not_found() -> Response<Full<Bytes>> {
        let html = r#"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>404 - 页面未找到</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
        }
        .container {
            background: white;
            border-radius: 20px;
            padding: 3rem;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
            text-align: center;
            max-width: 500px;
        }
        h1 {
            font-size: 6rem;
            color: #667eea;
            margin-bottom: 1rem;
        }
        h2 { color: #333; margin-bottom: 1rem; }
        p { color: #666; margin-bottom: 2rem; }
        a {
            display: inline-block;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 1rem 2rem;
            border-radius: 10px;
            text-decoration: none;
            font-weight: 600;
            transition: transform 0.3s;
        }
        a:hover { transform: translateY(-2px); }
    </style>
</head>
<body>
    <div class="container">
        <h1>404</h1>
        <h2>页面未找到</h2>
        <p>抱歉，您访问的端点不存在。</p>
        <a href="/">返回首页</a>
    </div>
</body>
</html>
        "#;
        Self::render_html(StatusCode::NOT_FOUND, html)
    }

    /// 渲染 HTML 响应
    fn render_html(status: StatusCode, html: &str) -> Response<Full<Bytes>> {
        Response::builder()
            .status(status)
            .header("Content-Type", "text/html; charset=utf-8")
            .header("X-Content-Type-Options", "nosniff")
            .body(Full::new(Bytes::from(html.to_string())))
            .unwrap()
    }
}
