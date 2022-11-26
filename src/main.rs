extern crate pathfinding;
extern crate sorting;

mod components;
mod hooks;
mod pages;
mod utils;

use gloo_storage::{LocalStorage, Storage};
use hooks::use_color_scheme::{use_color_scheme, ColorScheme, ColorSchemeMode};
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlMetaElement};
use yew::prelude::*;
use yew_hooks::use_update;
use yew_router::prelude::*;

#[derive(Clone, Debug, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/sorting")]
    Sorting,
    // Without this, subroutes don't seem to be recognized even though they are defined in pages::sorting
    #[at("/sorting/:algorithm")]
    SortingAlgorithm,
    #[at("/pathfinding")]
    Pathfinding,
    #[at("/pathfinding/:algorithm")]
    PathfindingAlgorithm,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {
            <pages::home::HomePage />
        },
        Route::Sorting => html! {
            <Switch<pages::sorting::SortingRoute> render={pages::sorting::switch_sorting} />
        },
        Route::SortingAlgorithm => html! {
            <Switch<pages::sorting::SortingRoute> render={pages::sorting::switch_sorting} />
        },
        Route::Pathfinding => html! {
            <Switch<pages::pathfinding::PathfindingRoute> render={pages::pathfinding::switch_pathfinding} />
        },
        Route::PathfindingAlgorithm => html! {
            <Switch<pages::pathfinding::PathfindingRoute> render={pages::pathfinding::switch_pathfinding} />
        },
    }
}

#[function_component]
fn App() -> Html {
    let color_scheme = use_color_scheme();
    let update = use_update();

    let set_color_scheme = move |color_scheme: ColorScheme| {
        let document = window().unwrap().document().unwrap();
        // Set body class according to color scheme
        document
            .body()
            .unwrap()
            .set_class_name(&color_scheme.to_string().to_lowercase());
        // Set PWA theme color
        if let Ok(meta) = document.query_selector("meta[name=theme-color]") {
            meta.unwrap()
                .dyn_into::<HtmlMetaElement>()
                .unwrap()
                .set_content(match color_scheme {
                    ColorScheme::Light => "#d3dbde",
                    ColorScheme::Dark => "#161b1d",
                });
        }
    };

    use_effect_with_deps(
        move |_| {
            set_color_scheme(color_scheme);
            || ()
        },
        color_scheme,
    );

    let toggle_theme = {
        Callback::from(move |_| {
            let next_color_scheme = match LocalStorage::get("app-color-scheme-mode").unwrap() {
                ColorSchemeMode::Auto => match LocalStorage::get("preferred-color-scheme").unwrap()
                {
                    ColorScheme::Light => ColorScheme::Dark,
                    ColorScheme::Dark => ColorScheme::Light,
                },
                ColorSchemeMode::Light => ColorScheme::Dark,
                ColorSchemeMode::Dark => ColorScheme::Light,
            };
            LocalStorage::set("app-color-scheme-mode", next_color_scheme.to_string()).unwrap();
            set_color_scheme(next_color_scheme);
            update();
        })
    };

    html! {
        <BrowserRouter>
            <ContextProvider<ColorScheme> context={color_scheme}>
                <div class="top-bar">
                    <div class="page-links">
                        <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                        <Link<Route> to={Route::Sorting}>{ "Sorting" }</Link<Route>>
                        <Link<Route> to={Route::Pathfinding}>{ "Pathfinding" }</Link<Route>>
                    </div>
                    <div class="other-links">
                        <button onclick={toggle_theme}>{
                            match color_scheme {
                                ColorScheme::Light => "☀️",
                                ColorScheme::Dark => "🌙"
                            }
                        }</button>
                        <a href="https://github.com/Jondolf/rust-algorithms" target="_blank" aria-label="Link to this website's GitHub repository (opens in a new window)">
                            <img
                                src={
                                    match color_scheme {
                                        ColorScheme::Light => "/assets/images/GitHub-Mark-64px.png",
                                        ColorScheme::Dark => "/assets/images/GitHub-Mark-Light-64px.png",
                                    }
                                }
                                alt="GitHub logo"
                                width="40"
                                height="40"
                            />
                        </a>
                    </div>
                </div>
                <Switch<Route> render={switch} />
            </ContextProvider<ColorScheme>>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
