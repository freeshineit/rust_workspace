mod pages;
mod components;

use yew::prelude::*;
use yew_router::prelude::*;
use pages::{Home, About};
use components::Nav;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::NotFound => html! { <h1>{ "404 - 页面未找到" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Nav />
            <main class="container">
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
