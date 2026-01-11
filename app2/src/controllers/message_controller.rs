use crate::models::{ApiResponse, EchoRequest};
use crate::services::MessageService;
use crate::views::json_view::JsonView;
use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Incoming};
use hyper::{Request, Response, StatusCode};

/// 消息控制器
pub struct MessageController {
    service: MessageService,
}

impl MessageController {
    /// 创建新的消息控制器
    pub fn new(service: MessageService) -> Self {
        Self { service }
    }

    /// 处理 Hello 请求
    pub fn handle_hello(&self) -> Response<Full<Bytes>> {
        let data = self.service.handle_hello();
        let response = ApiResponse::success("请求成功", data);
        JsonView::render(StatusCode::OK, &response)
    }

    /// 处理 Echo 请求
    pub async fn handle_echo(&self, req: Request<Incoming>) -> Response<Full<Bytes>> {
        // 读取请求体
        let body = match req.collect().await {
            Ok(collected) => collected.to_bytes(),
            Err(_) => {
                let response = ApiResponse::<()>::error("无法读取请求体");
                return JsonView::render(StatusCode::BAD_REQUEST, &response);
            }
        };

        // 解析 JSON
        let echo_req: EchoRequest = match serde_json::from_slice(&body) {
            Ok(req) => req,
            Err(_) => {
                let response = ApiResponse::<()>::error("无效的 JSON 格式");
                return JsonView::render(StatusCode::BAD_REQUEST, &response);
            }
        };

        // 处理请求
        match self.service.handle_echo(echo_req) {
            Ok(data) => {
                let response = ApiResponse::success("Echo 成功", data);
                JsonView::render(StatusCode::OK, &response)
            }
            Err(err) => {
                let response = ApiResponse::<()>::error(err);
                JsonView::render(StatusCode::BAD_REQUEST, &response)
            }
        }
    }
}
