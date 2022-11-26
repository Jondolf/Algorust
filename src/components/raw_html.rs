use web_sys::Element;
use yew::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct RawHtmlProps {
    pub inner_html: String,
}

#[function_component]
pub fn RawHtml(props: &RawHtmlProps) -> Html {
    let el_ref = use_node_ref();
    let el: UseStateHandle<Option<Element>> = use_state_eq(|| None);

    {
        let el = el.clone();

        use_effect_with_deps(
            move |el_ref| {
                el.set(el_ref.cast::<Element>());
                || ()
            },
            el_ref.clone(),
        );
    }
    {
        let html = props.inner_html.to_string();
        use_effect(move || {
            if let Some(el) = &*el {
                el.set_inner_html(&html);
            }
            || ()
        })
    }

    html! {
        <div ref={el_ref} />
    }
}
