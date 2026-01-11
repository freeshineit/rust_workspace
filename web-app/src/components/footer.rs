use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

/// 页脚组件
#[function_component(Footer)]
pub fn footer() -> Html {
    let current_year = 2026;

    html! {
        <footer class="footer">
            <div class="footer-container">
                <div class="footer-content">
                    <div class="footer-section">
                        <h3>{ "Yew Web App" }</h3>
                        <p>{ "使用 Rust 和 WebAssembly 构建的现代 Web 应用" }</p>
                    </div>

                    <div class="footer-section">
                        <h4>{ "快速链接" }</h4>
                        <ul class="footer-links">
                            <li>
                                <Link<Route> to={Route::Home}>{ "首页" }</Link<Route>>
                            </li>
                            <li>
                                <Link<Route> to={Route::About}>{ "关于" }</Link<Route>>
                            </li>
                        </ul>
                    </div>

                    <div class="footer-section">
                        <h4>{ "资源" }</h4>
                        <ul class="footer-links">
                            <li><a href="https://yew.rs" target="_blank">{ "Yew 文档" }</a></li>
                            <li><a href="https://www.rust-lang.org" target="_blank">{ "Rust 官网" }</a></li>
                            <li><a href="https://webassembly.org" target="_blank">{ "WebAssembly" }</a></li>
                        </ul>
                    </div>

                    <div class="footer-section">
                        <h4>{ "联系我们" }</h4>
                        <ul class="footer-links">
                            <li>{ "📧 contact@example.com" }</li>
                            <li>{ "🐙 GitHub" }</li>
                        </ul>
                    </div>
                </div>

                <div class="footer-bottom">
                    <p>{ format!("© {} Yew Web App. All rights reserved.", current_year) }</p>
                    <p>{ "Made with ❤️ using Rust & Yew" }</p>
                </div>
            </div>
        </footer>
    }
}
