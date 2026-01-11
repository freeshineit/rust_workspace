# App2 MVC 架构说明

## 🏗️ 架构概览

App2 采用经典的 MVC (Model-View-Controller) 架构模式，并增加了 Service 层来处理业务逻辑。

```
┌─────────────────────────────────────────────────────────┐
│                      Client Request                      │
└────────────────────────┬────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────┐
│                    Router (路由层)                       │
│  - 请求分发                                              │
│  - 路由匹配                                              │
└────────────────────────┬────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────┐
│                Controller (控制器层)                     │
│  - HealthController    (健康检查)                        │
│  - MessageController   (消息处理)                        │
│  - PageController      (页面渲染)                        │
└────────────┬───────────────────────┬────────────────────┘
             │                       │
             ▼                       ▼
┌────────────────────────┐  ┌──────────────────────────┐
│   Service (服务层)      │  │   View (视图层)          │
│  - HealthService       │  │  - JsonView              │
│  - MessageService      │  │  - HtmlView              │
│                        │  │                          │
│  业务逻辑处理           │  │  响应渲染                │
└────────────┬───────────┘  └──────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────────────────────┐
│                   Model (模型层)                         │
│  - HealthCheck         (健康检查数据)                    │
│  - HelloResponse       (Hello 响应)                      │
│  - EchoRequest         (Echo 请求)                       │
│  - EchoResponse        (Echo 响应)                       │
│  - ApiResponse<T>      (统一响应格式)                    │
└─────────────────────────────────────────────────────────┘
```

## 📁 目录结构

```
app2/
├── src/
│   ├── main.rs                      # 应用入口
│   ├── router.rs                    # 路由器
│   ├── models/                      # 模型层
│   │   ├── mod.rs
│   │   ├── api_response.rs          # API 响应结构
│   │   ├── health.rs                # 健康检查模型
│   │   └── message.rs               # 消息模型
│   ├── views/                       # 视图层
│   │   ├── mod.rs
│   │   ├── json_view.rs             # JSON 视图渲染
│   │   └── html_view.rs             # HTML 视图渲染
│   ├── controllers/                 # 控制器层
│   │   ├── mod.rs
│   │   ├── health_controller.rs     # 健康检查控制器
│   │   ├── message_controller.rs    # 消息控制器
│   │   └── page_controller.rs       # 页面控制器
│   └── services/                    # 服务层
│       ├── mod.rs
│       ├── health_service.rs        # 健康检查服务
│       └── message_service.rs       # 消息服务
├── templates/                       # HTML 模板
│   └── home.html                    # 首页模板
├── test_api.sh                      # API 测试脚本
├── Cargo.toml                       # 依赖配置
├── README.md                        # 项目说明
└── ARCHITECTURE.md                  # 架构文档（本文件）
```

## 🎯 各层职责

### 1. Model (模型层)

**职责**: 定义数据结构和业务实体

**文件**:
- `api_response.rs` - 统一的 API 响应格式
- `health.rs` - 健康检查数据模型
- `message.rs` - 消息相关模型（请求/响应）

**特点**:
- 纯数据结构，不包含业务逻辑
- 实现序列化/反序列化（Serde）
- 包含数据验证方法

**示例**:
```rust
#[derive(Serialize, Deserialize)]
pub struct EchoRequest {
    pub message: String,
}

impl EchoRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.message.is_empty() {
            return Err("消息不能为空".to_string());
        }
        Ok(())
    }
}
```

### 2. View (视图层)

**职责**: 负责渲染响应（JSON/HTML）

**文件**:
- `json_view.rs` - JSON 响应渲染器
- `html_view.rs` - HTML 页面渲染器

**特点**:
- 不包含业务逻辑
- 只负责格式化输出
- 设置正确的 HTTP 头

**示例**:
```rust
pub struct JsonView;

impl JsonView {
    pub fn render<T: Serialize>(status: StatusCode, data: &T) -> Response<Full<Bytes>> {
        let json = serde_json::to_string_pretty(data).unwrap();
        Response::builder()
            .status(status)
            .header("Content-Type", "application/json")
            .body(Full::new(Bytes::from(json)))
            .unwrap()
    }
}
```

### 3. Controller (控制器层)

**职责**: 处理 HTTP 请求，协调 Model 和 View

**文件**:
- `health_controller.rs` - 健康检查端点
- `message_controller.rs` - 消息处理端点
- `page_controller.rs` - 页面渲染端点

**特点**:
- 接收 HTTP 请求
- 调用 Service 层处理业务逻辑
- 使用 View 层渲染响应
- 处理错误和异常

**示例**:
```rust
pub struct MessageController {
    service: MessageService,
}

impl MessageController {
    pub fn handle_hello(&self) -> Response<Full<Bytes>> {
        let data = self.service.handle_hello();
        let response = ApiResponse::success("请求成功", data);
        JsonView::render(StatusCode::OK, &response)
    }
}
```

### 4. Service (服务层)

**职责**: 实现核心业务逻辑

**文件**:
- `health_service.rs` - 健康检查业务逻辑
- `message_service.rs` - 消息处理业务逻辑

**特点**:
- 包含业务规则
- 数据处理和转换
- 可被多个 Controller 复用
- 易于单元测试

**示例**:
```rust
pub struct MessageService;

impl MessageService {
    pub fn handle_echo(&self, request: EchoRequest) -> Result<EchoResponse, String> {
        request.validate()?;
        Ok(EchoResponse::from_request(request))
    }
}
```

### 5. Router (路由层)

**职责**: 请求分发和路由匹配

**文件**:
- `router.rs` - 路由器实现

**特点**:
- 管理所有 Controller 实例
- 根据 HTTP 方法和路径分发请求
- 处理 404 错误

**示例**:
```rust
let response = match (method, path) {
    (&Method::GET, "/") => self.page_controller.render_home(),
    (&Method::GET, "/health") => self.health_controller.check_health(),
    (&Method::GET, "/api/hello") => self.message_controller.handle_hello(),
    (&Method::POST, "/api/echo") => self.message_controller.handle_echo(req).await,
    _ => self.handle_not_found(),
};
```

## 🔄 请求处理流程

### 示例：处理 POST /api/echo 请求

1. **Router** 接收请求
   ```
   POST /api/echo
   Body: {"message": "Hello"}
   ```

2. **Router** 路由到 MessageController
   ```rust
   self.message_controller.handle_echo(req).await
   ```

3. **MessageController** 解析请求体
   ```rust
   let echo_req: EchoRequest = serde_json::from_slice(&body)?;
   ```

4. **MessageController** 调用 MessageService
   ```rust
   let data = self.service.handle_echo(echo_req)?;
   ```

5. **MessageService** 执行业务逻辑
   ```rust
   request.validate()?;
   Ok(EchoResponse::from_request(request))
   ```

6. **MessageController** 构建响应
   ```rust
   let response = ApiResponse::success("Echo 成功", data);
   ```

7. **JsonView** 渲染 JSON
   ```rust
   JsonView::render(StatusCode::OK, &response)
   ```

8. **返回响应**
   ```json
   {
     "success": true,
     "message": "Echo 成功",
     "data": {
       "echo": "Hello",
       "length": 5,
       "timestamp": "2026-01-12T10:30:00+08:00"
     }
   }
   ```

## ✨ 架构优势

### 1. 关注点分离
- 每层只关注自己的职责
- 代码更清晰，易于理解

### 2. 可测试性
- 每层可以独立测试
- Service 层特别容易编写单元测试

### 3. 可维护性
- 修改某一层不影响其他层
- 易于定位和修复问题

### 4. 可扩展性
- 添加新功能只需添加新的 Controller 和 Service
- 不影响现有代码

### 5. 代码复用
- Service 层可被多个 Controller 使用
- View 层可渲染不同的数据

## 🧪 测试策略

### Model 层测试
```rust
#[test]
fn test_echo_request_validation() {
    let request = EchoRequest { message: "".to_string() };
    assert!(request.validate().is_err());
}
```

### Service 层测试
```rust
#[test]
fn test_message_service() {
    let service = MessageService::new();
    let request = EchoRequest { message: "test".to_string() };
    let response = service.handle_echo(request).unwrap();
    assert_eq!(response.echo, "test");
}
```

### Controller 层测试
```rust
#[test]
fn test_health_controller() {
    let service = HealthService::new();
    let controller = HealthController::new(service);
    let response = controller.check_health();
    assert_eq!(response.status(), StatusCode::OK);
}
```

## 🚀 扩展指南

### 添加新的 API 端点

1. **定义 Model**
   ```rust
   // src/models/user.rs
   #[derive(Serialize, Deserialize)]
   pub struct User {
       pub id: u64,
       pub name: String,
   }
   ```

2. **实现 Service**
   ```rust
   // src/services/user_service.rs
   pub struct UserService;
   
   impl UserService {
       pub fn get_user(&self, id: u64) -> Option<User> {
           // 业务逻辑
       }
   }
   ```

3. **创建 Controller**
   ```rust
   // src/controllers/user_controller.rs
   pub struct UserController {
       service: UserService,
   }
   
   impl UserController {
       pub fn get_user(&self, id: u64) -> Response<Full<Bytes>> {
           match self.service.get_user(id) {
               Some(user) => {
                   let response = ApiResponse::success("获取成功", user);
                   JsonView::render(StatusCode::OK, &response)
               }
               None => {
                   let response = ApiResponse::<()>::error("用户不存在");
                   JsonView::render(StatusCode::NOT_FOUND, &response)
               }
           }
       }
   }
   ```

4. **添加路由**
   ```rust
   // src/router.rs
   (&Method::GET, path) if path.starts_with("/api/users/") => {
       let id = path.strip_prefix("/api/users/").unwrap().parse().unwrap();
       self.user_controller.get_user(id)
   }
   ```

## 📚 参考资源

- [MVC 架构模式](https://en.wikipedia.org/wiki/Model%E2%80%93view%E2%80%93controller)
- [Tokio 文档](https://tokio.rs)
- [Hyper 文档](https://hyper.rs)
- [Rust 设计模式](https://rust-unofficial.github.io/patterns/)
