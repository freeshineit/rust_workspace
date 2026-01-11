mod app;
mod components;
mod pages;
mod router;

use app::App;

/// 应用入口点
fn main() {
    // 渲染应用
    yew::Renderer::<App>::new().render();
}
