mod models;
mod utils;

use models::{Product, User};
use shared::greet;
use utils::{calculate_average, format_title, is_valid_email};

fn main() {
    println!("App1 启动!");
    greet("App1");

    println!("\n{}", format_title("用户管理"));

    // 创建用户
    let user = User::new(
        1,
        "张三".to_string(),
        "zhangsan@example.com".to_string(),
        25,
    );
    println!("{}", user.get_info());
    println!("是否成年: {}", user.is_adult());
    println!("邮箱验证: {}", is_valid_email(&user.email));

    println!("\n{}", format_title("产品管理"));

    // 创建产品
    let product = Product::new(101, "笔记本电脑".to_string(), 5999.99, 10);
    println!("产品: {} - 价格: ¥{:.2}", product.name, product.price);
    println!("库存: {} 件", product.stock);
    println!("总价值: ¥{:.2}", product.total_value());
    println!("是否有货: {}", product.is_available());

    println!("\n{}", format_title("工具函数"));

    // 使用工具函数
    let numbers = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    if let Some(avg) = calculate_average(&numbers) {
        println!("数字平均值: {:.2}", avg);
    }

    println!("\nApp1 完成!");
}
