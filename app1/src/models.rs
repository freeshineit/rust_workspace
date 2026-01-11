use serde::{Deserialize, Serialize};

/// 用户模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub age: u32,
}

impl User {
    /// 创建新用户
    pub fn new(id: u64, name: String, email: String, age: u32) -> Self {
        Self {
            id,
            name,
            email,
            age,
        }
    }

    /// 获取用户的完整信息
    pub fn get_info(&self) -> String {
        format!(
            "用户 #{}: {} ({}岁) - 邮箱: {}",
            self.id, self.name, self.age, self.email
        )
    }

    /// 检查用户是否成年
    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

/// 产品模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub price: f64,
    pub stock: u32,
}

impl Product {
    /// 创建新产品
    pub fn new(id: u64, name: String, price: f64, stock: u32) -> Self {
        Self {
            id,
            name,
            price,
            stock,
        }
    }

    /// 检查是否有库存
    pub fn is_available(&self) -> bool {
        self.stock > 0
    }

    /// 计算总价值
    pub fn total_value(&self) -> f64 {
        self.price * self.stock as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(
            1,
            "张三".to_string(),
            "zhangsan@example.com".to_string(),
            25,
        );
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "张三");
        assert!(user.is_adult());
    }

    #[test]
    fn test_product_availability() {
        let product = Product::new(1, "笔记本电脑".to_string(), 5999.99, 10);
        assert!(product.is_available());
        // 使用近似比较来处理浮点数精度问题
        let total = product.total_value();
        assert!((total - 59999.9).abs() < 0.01);
    }
}
