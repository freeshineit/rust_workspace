/// 共享的问候函数
pub fn greet(name: &str) {
    println!("你好, {}! 欢迎使用 Rust Workspace!", name);
}

/// 共享的工具函数
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
