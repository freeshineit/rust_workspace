use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

/// 404 页面组件
#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="page not-found-page">
            <div class="not-found-content">
                <h1 class="not-found-title">{ "404" }</h1>
                <h2>{ "页面未找到" }</h2>
                <p>{ "抱歉，您访问的页面不存在。" }</p>
                <div class="not-found-actions">
                    <Link<Route> to={Route::Home} classes="btn btn-primary btn-large">
                        { "返回首页" }
                    </Link<Route>>
                    <Link<Route> to={Route::About} classes="btn btn-outline btn-large">
                        { "关于我们" }
                    </Link<Route>>
                </div>
            </div>
        </div>
    }
}
