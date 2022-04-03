use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    pub title: String,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let ButtonProps { title, onclick } = props.clone();

    html! {
        <div class="input button">
            <button {onclick}>{title}</button>
        </div>
    }
}
