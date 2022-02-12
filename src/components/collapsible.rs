use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CollapsibleProps {
    pub children: Children,
    pub title: String,
    pub open: bool,
}

#[function_component(Collapsible)]
pub fn collapsible(props: &CollapsibleProps) -> Html {
    let open = use_state(|| props.open);
    let onclick = {
        let open = open.clone();
        move |e: MouseEvent| {
            e.prevent_default();
            open.set(!*open)
        }
    };

    html! {
      <details class="collapsible" open={*open}>
        <summary {onclick}>{props.title.to_string()}</summary>
        {
          if *open {
            props.children.iter().collect::<Html>()
          } else {
            html! {}
          }
        }
      </details>
    }
}
