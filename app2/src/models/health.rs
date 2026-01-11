use serde::{Deserialize, Serialize};

/// 健康检查数据模型
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HealthCheck {
    pub status: String,
    pub version: String,
    pub uptime: u64,
}

impl HealthCheck {
    /// 创建健康检查实例
    pub fn new(uptime: u64) -> Self {
        Self {
            status: "healthy".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime,
        }
    }

    /// 检查是否健康
    pub fn is_healthy(&self) -> bool {
        self.status == "healthy"
    }
}
