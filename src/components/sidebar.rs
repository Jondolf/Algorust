use log::info;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::window;
use yew::prelude::*;

use crate::hooks::use_mouse_pos::use_mouse_pos;

#[derive(Properties, Clone, PartialEq)]
pub struct SidebarProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let width_px = use_state_eq(|| 350);
    let resizing = use_state_eq(|| false);
    let (mouse_x, _) = use_mouse_pos();

    let on_handle_pressed = {
        let resizing = resizing.clone();

        Callback::from(move |e: MouseEvent| {
            info!("Hello {}", *resizing);
            e.prevent_default();
            e.stop_propagation();
            resizing.set(true);
        })
    };

    {
        let resizing = resizing.clone();

        use_effect_with_deps(
            move |_| {
                let mouseup_listener =
                    Closure::<dyn Fn(MouseEvent)>::wrap(Box::new(move |_| resizing.set(false)));
                window()
                    .unwrap()
                    .add_event_listener_with_callback(
                        "mouseup",
                        mouseup_listener.as_ref().unchecked_ref(),
                    )
                    .unwrap();

                move || drop(mouseup_listener)
            },
            (),
        );
    }

    {
        let width_px = width_px.clone();
        let resizing = resizing.clone();

        use_effect_with_deps(
            move |x| {
                let window_width = window().unwrap().inner_width().unwrap().as_f64().unwrap();
                // Minimum width is 1 to ensure that the resize handle is always available
                if *resizing && *x >= 1 && *x <= (window_width * 0.75) as i32 {
                    width_px.set(*x);
                }
                || ()
            },
            mouse_x,
        );
    }

    html! {
        <div class="sidebar" style={format!("min-width: {}px; max-width: {}px", *width_px, *width_px)}>
            <div
                class="resize-handle"
                style={if *resizing { "opacity: 1" } else { "" }}
                onmousedown={on_handle_pressed}>
            </div>
            { for props.children.iter() }
        </div>
    }
}
