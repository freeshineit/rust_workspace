use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Response, StatusCode};
use serde::Serialize;

/// JSON 视图渲染器
pub struct JsonView;

impl JsonView {
    /// 渲染 JSON 响应
    pub fn render<T: Serialize>(status: StatusCode, data: &T) -> Response<Full<Bytes>> {
        let json = serde_json::to_string_pretty(data)
            .unwrap_or_else(|_| r#"{"error": "序列化失败"}"#.to_string());

        Response::builder()
            .status(status)
            .header("Content-Type", "application/json; charset=utf-8")
            .header("X-Content-Type-Options", "nosniff")
            .body(Full::new(Bytes::from(json)))
            .unwrap()
    }
}
