use crate::{components::raw_html::RawHtml, utils::fetch};

use pulldown_cmark::{html, Options, Parser};
use web_sys::window;
use yew::prelude::*;
use yew_hooks::{use_async, use_mount};

#[derive(Clone, Properties, PartialEq)]
pub struct AlgoDescProps {
    pub algorithm: String,
}

#[function_component(AlgoDesc)]
pub fn algo_desc(props: &AlgoDescProps) -> Html {
    let url = use_state(String::new);

    {
        let url = url.clone();
        use_effect_with_deps(
            move |algorithm| {
                let location = window().unwrap().location();
                let origin = location.origin().unwrap();
                let pathname = location.pathname().unwrap();
                // sorting or pathfinding etc.
                let algorithm_type = pathname.split('/').collect::<Vec<&str>>()[1];

                url.set(format!(
                    "{}/{}_algorithms/{}/README.md",
                    origin,
                    algorithm_type,
                    algorithm
                        .to_lowercase()
                        .replace(' ', "_")
                        .replace('*', "_star")
                ));
                || ()
            },
            props.algorithm.clone(),
        );
    }

    let md = {
        let url = (*url).clone();
        use_async(async move { fetch(url, "text/markdown").await })
    };

    {
        let md = md.clone();
        use_mount(move || md.run());
    }

    {
        let md = md.clone();
        use_effect_with_deps(move |_| || md.run(), url);
    }

    html! {
        <>
            {
                if let Some(md) = &md.data {
                    if !md.is_empty(){
                        let mut options = Options::empty();
                        options.insert(Options::ENABLE_TABLES);
                        let parser = Parser::new_ext(md, options);

                        let mut html_output = String::new();
                        html::push_html(&mut html_output, parser);

                        html! {
                            <div class="algorithm-description">
                                <RawHtml inner_html={html_output} />
                            </div>
                        }
                    } else {
                        html! {}
                    }
                } else {
                    html! {}
                }
            }
            {
                if let Some(error) = &md.error {
                    html! { error }
                } else {
                    html! {}
                }
            }
        </>
    }
}
