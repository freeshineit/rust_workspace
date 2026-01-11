use serde::{Deserialize, Serialize};

/// Echo 请求模型
#[derive(Deserialize, Debug)]
pub struct EchoRequest {
    pub message: String,
}

impl EchoRequest {
    /// 验证请求
    pub fn validate(&self) -> Result<(), String> {
        if self.message.is_empty() {
            return Err("消息不能为空".to_string());
        }
        if self.message.len() > 1000 {
            return Err("消息长度不能超过 1000 字符".to_string());
        }
        Ok(())
    }
}

/// Echo 响应模型
#[derive(Serialize, Debug)]
pub struct EchoResponse {
    pub echo: String,
    pub length: usize,
    pub timestamp: String,
}

impl EchoResponse {
    /// 从请求创建响应
    pub fn from_request(request: EchoRequest) -> Self {
        let length = request.message.len();
        Self {
            echo: request.message,
            length,
            timestamp: chrono::Local::now().to_rfc3339(),
        }
    }
}

/// Hello 响应模型
#[derive(Serialize, Debug)]
pub struct HelloResponse {
    pub message: String,
    pub timestamp: String,
    pub server: String,
}

impl HelloResponse {
    /// 创建 Hello 响应
    pub fn new() -> Self {
        Self {
            message: "Hello from App2 Server!".to_string(),
            timestamp: chrono::Local::now().to_rfc3339(),
            server: "App2/Tokio".to_string(),
        }
    }
}

impl Default for HelloResponse {
    fn default() -> Self {
        Self::new()
    }
}
