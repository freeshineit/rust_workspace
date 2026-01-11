use crate::models::ApiResponse;
use crate::services::HealthService;
use crate::views::json_view::JsonView;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Response, StatusCode};

/// 健康检查控制器
pub struct HealthController {
    service: HealthService,
}

impl HealthController {
    /// 创建新的健康检查控制器
    pub fn new(service: HealthService) -> Self {
        Self { service }
    }

    /// 处理健康检查请求
    pub fn check_health(&self) -> Response<Full<Bytes>> {
        let health = self.service.get_health();
        let response = ApiResponse::success("服务器运行正常", health);
        JsonView::render(StatusCode::OK, &response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_health() {
        let service = HealthService::new();
        let controller = HealthController::new(service);
        let response = controller.check_health();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
