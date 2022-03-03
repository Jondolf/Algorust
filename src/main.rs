extern crate sorting_algorithms;

mod components;
mod hooks;
mod pages;
mod utils;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/sorting-algorithms")]
    SortingAlgorithms,
    // Without this, subroutes don't seem to be recognized even though they are defined in pages::sorting_algorithms
    #[at("/sorting-algorithms/:algorithm")]
    SortingAlgorithm,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <Redirect<Route> to={Route::SortingAlgorithms} />
        },
        Route::SortingAlgorithms => html! {
            <Switch<pages::sorting_algorithms::SortingAlgorithmsRoute> render={Switch::render(pages::sorting_algorithms::switch_sorting_algorithms)} />
        },
        Route::SortingAlgorithm => html! {
            <Switch<pages::sorting_algorithms::SortingAlgorithmsRoute> render={Switch::render(pages::sorting_algorithms::switch_sorting_algorithms)} />
        },
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
