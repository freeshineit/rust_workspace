use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="page about-page">
            <h1>{ "关于我们" }</h1>
            
            <div class="about-content">
                <section class="about-section">
                    <h2>{ "📖 项目介绍" }</h2>
                    <p>
                        { "这是一个使用 Yew 框架构建的 Rust Web 应用示例。" }
                        { "Yew 是一个现代化的 Rust 框架，用于使用 WebAssembly 创建多线程前端 Web 应用。" }
                    </p>
                    <p>
                        { "本项目展示了如何使用 Rust 生态系统构建完整的 Web 应用，" }
                        { "包括路由管理、状态管理、组件化开发和现代化的 UI 设计。" }
                    </p>
                </section>

                <div class="divider-light"></div>

                <section class="about-section">
                    <h2>{ "🛠️ 技术栈" }</h2>
                    <ul class="tech-list">
                        <li>{ "🦀 Rust - 系统编程语言" }</li>
                        <li>{ "🎨 Yew 0.21 - Rust 前端框架" }</li>
                        <li>{ "🔀 Yew Router 0.18 - 路由管理" }</li>
                        <li>{ "🌐 WebAssembly - 高性能 Web 运行时" }</li>
                        <li>{ "💅 Sass - CSS 预处理器" }</li>
                        <li>{ "📦 Trunk - 构建工具" }</li>
                    </ul>
                </section>

                <div class="divider-light"></div>

                <section class="about-section">
                    <h2>{ "✨ 核心特性" }</h2>
                    <div class="features-grid">
                        <div class="feature-item">
                            <strong>{ "🧩 组件化开发" }</strong>
                            <p>{ "使用函数式组件构建可复用的 UI，提高开发效率" }</p>
                        </div>
                        <div class="feature-item">
                            <strong>{ "🗺️ 路由管理" }</strong>
                            <p>{ "支持客户端路由和导航，实现单页应用体验" }</p>
                        </div>
                        <div class="feature-item">
                            <strong>{ "🎯 状态管理" }</strong>
                            <p>{ "使用 hooks 管理组件状态，简洁高效" }</p>
                        </div>
                        <div class="feature-item">
                            <strong>{ "🔐 类型安全" }</strong>
                            <p>{ "编译时类型检查，减少运行时错误" }</p>
                        </div>
                        <div class="feature-item">
                            <strong>{ "⚡ 高性能" }</strong>
                            <p>{ "WebAssembly 提供接近原生的执行速度" }</p>
                        </div>
                        <div class="feature-item">
                            <strong>{ "📱 响应式设计" }</strong>
                            <p>{ "适配各种屏幕尺寸，提供最佳用户体验" }</p>
                        </div>
                    </div>
                </section>

                <div class="divider-light"></div>

                <section class="about-section">
                    <h2>{ "🚀 开发历程" }</h2>
                    <div class="timeline">
                        <div class="timeline-item">
                            <h3>{ "项目初始化" }</h3>
                            <p>{ "搭建 Rust workspace，配置 Yew 框架和路由系统" }</p>
                        </div>
                        <div class="timeline-item">
                            <h3>{ "页面开发" }</h3>
                            <p>{ "创建首页和关于页面，实现基础导航功能" }</p>
                        </div>
                        <div class="timeline-item">
                            <h3>{ "样式优化" }</h3>
                            <p>{ "引入 Sass，构建模块化的样式系统" }</p>
                        </div>
                        <div class="timeline-item">
                            <h3>{ "功能完善" }</h3>
                            <p>{ "添加交互式组件，优化用户体验" }</p>
                        </div>
                    </div>
                </section>

                <div class="divider-light"></div>

                <section class="about-section">
                    <h2>{ "👥 团队成员" }</h2>
                    <div class="team-grid">
                        <div class="team-member">
                            <div class="avatar">{ "R" }</div>
                            <h3>{ "Rust 开发者" }</h3>
                            <p class="role">{ "后端架构师" }</p>
                        </div>
                        <div class="team-member">
                            <div class="avatar">{ "Y" }</div>
                            <h3>{ "Yew 专家" }</h3>
                            <p class="role">{ "前端工程师" }</p>
                        </div>
                        <div class="team-member">
                            <div class="avatar">{ "W" }</div>
                            <h3>{ "WASM 工程师" }</h3>
                            <p class="role">{ "性能优化" }</p>
                        </div>
                    </div>
                </section>

                <div class="divider-light"></div>

                <section class="about-section">
                    <h2>{ "📬 联系方式" }</h2>
                    <p>{ "如有任何问题、建议或合作意向，欢迎随时联系我们！" }</p>
                    <div class="contact-info">
                        <p>{ "📧 Email: contact@example.com" }</p>
                        <p>{ "🐙 GitHub: github.com/example/yew-app" }</p>
                        <p>{ "🌐 Website: https://example.com" }</p>
                        <p>{ "💬 Discord: discord.gg/example" }</p>
                    </div>
                </section>
            </div>
        </div>
    }
}
