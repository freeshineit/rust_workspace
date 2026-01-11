use serde::{Deserialize, Serialize};

/// 统一的 API 响应结构
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    /// 创建成功响应
    pub fn success(message: impl Into<String>, data: T) -> Self {
        Self {
            success: true,
            message: message.into(),
            data: Some(data),
        }
    }

    /// 创建错误响应
    pub fn error(message: impl Into<String>) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            message: message.into(),
            data: None,
        }
    }
}

impl<T> Default for ApiResponse<T> {
    fn default() -> Self {
        Self {
            success: true,
            message: "操作成功".to_string(),
            data: None,
        }
    }
}
