use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let counter = use_state(|| 0);
    
    let increment = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(*counter + 1);
        })
    };
    
    let decrement = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(*counter - 1);
        })
    };
    
    let reset = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(0);
        })
    };

    html! {
        <div class="page home-page">
            // Hero 区域
            <div class="hero">
                <h1>{ "欢迎来到 Yew Web 应用" }</h1>
                <p class="subtitle">{ "使用 Rust 和 WebAssembly 构建的现代 Web 应用" }</p>
                <div class="hero-buttons">
                    <button class="btn btn-primary btn-large">{ "开始使用" }</button>
                    <button class="btn btn-outline btn-large">{ "了解更多" }</button>
                </div>
            </div>

            // 特性卡片
            <div class="features">
                <div class="feature-card">
                    <div class="icon-box">{ "🚀" }</div>
                    <h3>{ "高性能" }</h3>
                    <p>{ "基于 WebAssembly，提供接近原生的性能体验" }</p>
                </div>
                <div class="feature-card">
                    <div class="icon-box">{ "🔒" }</div>
                    <h3>{ "类型安全" }</h3>
                    <p>{ "Rust 的类型系统确保代码的安全性和可靠性" }</p>
                </div>
                <div class="feature-card">
                    <div class="icon-box">{ "⚡" }</div>
                    <h3>{ "响应式" }</h3>
                    <p>{ "现代化的组件式开发体验，快速构建 UI" }</p>
                </div>
                <div class="feature-card">
                    <div class="icon-box">{ "🎨" }</div>
                    <h3>{ "现代设计" }</h3>
                    <p>{ "使用 Sass 构建的美观响应式界面" }</p>
                </div>
            </div>

            // 统计数据
            <div class="stats">
                <div class="stat-item">
                    <span class="stat-number">{ "100%" }</span>
                    <span class="stat-label">{ "Rust 编写" }</span>
                </div>
                <div class="stat-item">
                    <span class="stat-number">{ "0" }</span>
                    <span class="stat-label">{ "运行时错误" }</span>
                </div>
                <div class="stat-item">
                    <span class="stat-number">{ "∞" }</span>
                    <span class="stat-label">{ "可能性" }</span>
                </div>
            </div>

            <div class="divider"></div>

            // 计数器演示
            <div class="counter-demo">
                <h2>{ "交互式计数器演示" }</h2>
                <p class="subtitle">{ "体验 Yew 的响应式状态管理" }</p>
                
                <div class="counter-display">
                    <span class="counter-value">{ *counter }</span>
                </div>
                
                <div class="counter-buttons">
                    <button onclick={decrement} class="btn btn-secondary btn-large">
                        { "➖ 减少" }
                    </button>
                    <button onclick={reset} class="btn btn-outline btn-large">
                        { "🔄 重置" }
                    </button>
                    <button onclick={increment} class="btn btn-primary btn-large">
                        { "➕ 增加" }
                    </button>
                </div>
                
                <div class="mt-3">
                    if *counter > 0 {
                        <span class="badge badge-success">{ "正数" }</span>
                    } else if *counter < 0 {
                        <span class="badge badge-warning">{ "负数" }</span>
                    } else {
                        <span class="badge badge-info">{ "零" }</span>
                    }
                </div>
            </div>
        </div>
    }
}
