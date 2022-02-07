extern crate sorting_algorithms;

mod components;
mod pages;
mod utils;

use pages::sorting_algorithms::SortingAlgorithms;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    SortingAlgorithms,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::SortingAlgorithms => html! { <SortingAlgorithms /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
    wasm_logger::init(wasm_logger::Config::default());
}
