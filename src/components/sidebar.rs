use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::window;
use yew::prelude::*;
use yew_hooks::use_window_size;

use crate::hooks::use_mouse_pos::use_mouse_pos;

/// The screen width where the layout changes and the sidebar moves to the bottom
const MOBILE_MODE_THRESHOLD: i32 = 600;
const RESIZE_HANDLE_THICKNESS_PX: i32 = 4;

#[derive(Properties, Clone, PartialEq)]
pub struct SidebarProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let width_px = use_state_eq(|| 350); // Used on large screens, sidebar on the left side
    let height_px = use_state_eq(|| 350); // Used on small screens, sidebar at the bottom
    let resizing_width = use_state_eq(|| false);
    let resizing_height = use_state_eq(|| false);
    let (window_width, window_height) = use_window_size();
    let (mouse_x, mouse_y) = use_mouse_pos();

    let on_resize_width_mousedown = {
        let resizing_width = resizing_width.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            resizing_width.set(true);
        })
    };
    let on_resize_width_touchstart = {
        let resizing_width = resizing_width.clone();

        Callback::from(move |e: TouchEvent| {
            e.prevent_default();
            e.stop_propagation();
            resizing_width.set(true);
        })
    };

    let on_resize_height_mousedown = {
        let resizing_height = resizing_height.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            resizing_height.set(true);
        })
    };
    let on_resize_height_touchstart = {
        let resizing_height = resizing_height.clone();

        Callback::from(move |e: TouchEvent| {
            e.stop_propagation();
            resizing_height.set(true);
        })
    };

    {
        let resizing_width = resizing_width.clone();
        let resizing_height = resizing_height.clone();

        use_effect_with_deps(
            move |_| {
                let callback = Closure::<dyn Fn(MouseEvent)>::wrap(Box::new(move |_| {
                    resizing_width.set(false);
                    resizing_height.set(false);
                }));
                window()
                    .unwrap()
                    .add_event_listener_with_callback("mouseup", callback.as_ref().unchecked_ref())
                    .unwrap();
                window()
                    .unwrap()
                    .add_event_listener_with_callback("touchend", callback.as_ref().unchecked_ref())
                    .unwrap();

                move || drop(callback)
            },
            (),
        );
    }

    {
        let width_px = width_px.clone();
        let height_px = height_px.clone();
        let resizing_width = resizing_width.clone();
        let resizing_height = resizing_height.clone();

        use_effect_with_deps(
            move |(x, y, win_width, win_height)| {
                let (new_width, new_height) = (*x, *win_height as i32 - *y);
                let (max_width, max_height) = ((win_width * 0.75) as i32, *win_height as i32);
                let min = RESIZE_HANDLE_THICKNESS_PX;

                if *width_px > max_width && *win_width >= MOBILE_MODE_THRESHOLD as f64 {
                    width_px.set(max_width);
                }
                if *height_px > max_height {
                    height_px.set(max_height);
                }

                if *resizing_width && new_width >= min && new_width <= max_width {
                    width_px.set(new_width);
                }

                if *resizing_height && new_height >= min && new_height <= max_height {
                    height_px.set(new_height);
                }
                || ()
            },
            (mouse_x, mouse_y, window_width, window_height),
        );
    }

    html! {
        <div class="sidebar" style={format!("min-width: {}px; min-height: {}px", *width_px, *height_px)}
        >
            <div
                class="resize-handle resize-width"
                style={if *resizing_width { "opacity: 1" } else { "" }}
                onmousedown={on_resize_width_mousedown}
                ontouchstart={on_resize_width_touchstart}>
            </div>
            <div
                class="resize-handle resize-height"
                style={if *resizing_height { "opacity: 1" } else { "" }}
                onmousedown={on_resize_height_mousedown}
                ontouchstart={on_resize_height_touchstart}>
            </div>
            <div class="sidebar-content">
                { for props.children.iter() }
            </div>
        </div>
    }
}
