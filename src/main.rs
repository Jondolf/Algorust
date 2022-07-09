extern crate pathfinding;
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
    #[at("/pathfinding")]
    PathfindingAlgorithms,
    #[at("/pathfinding/:algorithm")]
    PathfindingAlgorithm,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <pages::home::HomePage />
        },
        Route::SortingAlgorithms => html! {
            <Switch<pages::sorting_algorithms::SortingAlgorithmsRoute> render={Switch::render(pages::sorting_algorithms::switch_sorting_algorithms)} />
        },
        Route::SortingAlgorithm => html! {
            <Switch<pages::sorting_algorithms::SortingAlgorithmsRoute> render={Switch::render(pages::sorting_algorithms::switch_sorting_algorithms)} />
        },
        Route::PathfindingAlgorithms => html! {
            <Switch<pages::pathfinding::PathfindingRoute> render={Switch::render(pages::pathfinding::switch_pathfinding)} />
        },
        Route::PathfindingAlgorithm => html! {
            <Switch<pages::pathfinding::PathfindingRoute> render={Switch::render(pages::pathfinding::switch_pathfinding)} />
        },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="top-bar">
                <div class="page-links">
                    <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                    <Link<Route> to={Route::SortingAlgorithms}>{ "Sorting" }</Link<Route>>
                    <Link<Route> to={Route::PathfindingAlgorithms}>{ "Pathfinding" }</Link<Route>>
                </div>
                <div class="other-links">
                    <a href="https://github.com/Jondolf/rust-algorithms" target="_blank">
                        <img src="/assets/images/GitHub-Mark-Light-64px.png" />
                    </a>
                </div>
            </div>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
    wasm_logger::init(wasm_logger::Config::default());
}
