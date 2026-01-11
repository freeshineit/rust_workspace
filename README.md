# Rust Workspace 示例

![MacOS Build](https://github.com/freeshineit/rust_workspace/workflows/macos-build/badge.svg)
![Windows Build](https://github.com/freeshineit/rust_workspace/workflows/windows-build/badge.svg)
![Ubuntu Build](https://github.com/freeshineit/rust_workspace/workflows/ubuntu-build/badge.svg)


这是一个包含多个 bin 项目的 Rust workspace。

## 项目结构

- `app1/` - 第一个可执行程序
- `app2/` - 第二个可执行程序（使用 tokio 异步运行时）
- `shared/` - 共享库，被两个应用使用
- `web-app/` - yew web 应用

## 运行项目

```bash
# 运行 app1
cargo run --bin app1

# 运行 app2
cargo run --bin app2

# 运行 web-app
cd web-app
trunk serve

# 构建所有项目
cargo build --workspace

# 运行测试
cargo test --workspace

# 格式化代码
make fmt

# 检查格式
make fmt-check

# 代码质量检查
make lint

# 完整检查
make check
```

## 添加新的 bin 项目

1. 在 workspace 根目录的 `Cargo.toml` 中添加新成员
2. 创建新的项目目录和 `Cargo.toml`
3. 在 `[[bin]]` 部分指定 bin 名称和路径
