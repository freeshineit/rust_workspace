use crate::models::{EchoRequest, EchoResponse, HelloResponse};

/// 消息处理服务
pub struct MessageService;

impl MessageService {
    /// 创建新的消息服务
    pub fn new() -> Self {
        Self
    }

    /// 处理 Hello 请求
    pub fn handle_hello(&self) -> HelloResponse {
        HelloResponse::new()
    }

    /// 处理 Echo 请求
    pub fn handle_echo(&self, request: EchoRequest) -> Result<EchoResponse, String> {
        // 验证请求
        request.validate()?;

        // 创建响应
        Ok(EchoResponse::from_request(request))
    }

    /// 处理消息转换（示例业务逻辑）
    pub fn transform_message(&self, message: &str) -> String {
        message.to_uppercase()
    }
}

impl Default for MessageService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_hello() {
        let service = MessageService::new();
        let response = service.handle_hello();
        assert!(!response.message.is_empty());
    }

    #[test]
    fn test_handle_echo() {
        let service = MessageService::new();
        let request = EchoRequest {
            message: "test".to_string(),
        };
        let response = service.handle_echo(request).unwrap();
        assert_eq!(response.echo, "test");
        assert_eq!(response.length, 4);
    }

    #[test]
    fn test_echo_validation() {
        let service = MessageService::new();
        let request = EchoRequest {
            message: "".to_string(),
        };
        assert!(service.handle_echo(request).is_err());
    }
}
