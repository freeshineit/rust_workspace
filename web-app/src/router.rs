use crate::pages::{About, Home, NotFound};
use yew::prelude::*;
use yew_router::prelude::*;

/// 应用路由定义
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// 路由切换函数
pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
