use crate::components::input_items::input_title_to_id;

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct CheckboxProps {
    pub title: String,
    pub value: bool,
    pub oninput: Callback<InputEvent>,
}

#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let CheckboxProps {
        title,
        value,
        oninput,
    } = props.clone();
    let id = input_title_to_id(&title);

    html! {
        <div class="input checkbox">
            <input id={id.clone()} type="checkbox" checked={value} {oninput} />
            <label for={id}>{title}</label>
        </div>
    }
}
