use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav class="navbar">
            <div class="nav-container">
                <div class="nav-brand">
                    <h2>{ "Yew Web App" }</h2>
                </div>
                <ul class="nav-menu">
                    <li class="nav-item">
                        <Link<Route> to={Route::Home} classes="nav-link">
                            { "首页" }
                        </Link<Route>>
                    </li>
                    <li class="nav-item">
                        <Link<Route> to={Route::About} classes="nav-link">
                            { "关于" }
                        </Link<Route>>
                    </li>
                </ul>
            </div>
        </nav>
    }
}
