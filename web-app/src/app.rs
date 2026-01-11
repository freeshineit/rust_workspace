use crate::components::{Footer, Nav};
use crate::router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

/// 应用根组件
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="app-wrapper">
                <Nav />
                <main class="container">
                    <Switch<Route> render={switch} />
                </main>
                <Footer />
            </div>
        </BrowserRouter>
    }
}
