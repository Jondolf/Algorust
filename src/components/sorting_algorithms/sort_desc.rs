use crate::{components::raw_html::RawHtml, utils::fetch};

use pulldown_cmark::{html, Options, Parser};
use yew::prelude::*;
use yew_hooks::{use_async, use_mount};

#[derive(Clone, Properties, PartialEq)]
pub struct SortDescProps {
    pub url: String,
}

#[function_component(SortDesc)]
pub fn sort_desc(props: &SortDescProps) -> Html {
    let url = props.url.to_string();
    let md = use_async(async move { fetch(url, "text/markdown").await });

    {
        let md = md.clone();
        use_mount(move || md.run());
    }

    {
        let md = md.clone();
        use_effect_with_deps(move |_| || md.run(), props.url.to_string());
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
