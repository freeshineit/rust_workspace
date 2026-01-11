use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

/// 导航栏组件
#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav class="navbar">
            <div class="nav-container">
                <div class="nav-brand">
                    <Link<Route> to={Route::Home}>
                        <h2>{ "🦀 Yew Web App" }</h2>
                    </Link<Route>>
                </div>
                <ul class="nav-menu">
                    <li class="nav-item">
                        <Link<Route> to={Route::Home} classes="nav-link">
                            { "🏠 首页" }
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::About} classes="nav-link">
                            { "ℹ️ 关于" }
                        </Link<Route>>
                    </li>
                </ul>
            </div>
        </nav>
    }
}
