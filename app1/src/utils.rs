/// 工具函数模块
///
/// 格式化字符串为标题格式
pub fn format_title(text: &str) -> String {
    format!("=== {} ===", text.to_uppercase())
}

/// 计算两个数字的平均值
pub fn calculate_average(numbers: &[f64]) -> Option<f64> {
    if numbers.is_empty() {
        return None;
    }
    let sum: f64 = numbers.iter().sum();
    Some(sum / numbers.len() as f64)
}

/// 验证邮箱格式（简单验证）
pub fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_title() {
        assert_eq!(format_title("hello"), "=== HELLO ===");
    }

    #[test]
    fn test_calculate_average() {
        assert_eq!(calculate_average(&[1.0, 2.0, 3.0]), Some(2.0));
        assert_eq!(calculate_average(&[]), None);
    }

    #[test]
    fn test_is_valid_email() {
        assert!(is_valid_email("test@example.com"));
        assert!(!is_valid_email("invalid-email"));
    }
}
