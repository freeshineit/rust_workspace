use crate::models::HealthCheck;
use std::time::Instant;

/// 健康检查服务
pub struct HealthService {
    start_time: Instant,
}

impl HealthService {
    /// 创建新的健康检查服务
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }

    /// 获取健康状态
    pub fn get_health(&self) -> HealthCheck {
        let uptime = self.start_time.elapsed().as_secs();
        HealthCheck::new(uptime)
    }

    /// 获取运行时间（秒）
    pub fn get_uptime(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }
}

impl Default for HealthService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_service() {
        let service = HealthService::new();
        let health = service.get_health();
        assert!(health.is_healthy());
        assert_eq!(health.status, "healthy");
    }
}
