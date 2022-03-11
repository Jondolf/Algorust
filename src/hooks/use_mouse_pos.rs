use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::window;
use yew::prelude::*;

pub fn use_mouse_pos() -> (i32, i32) {
    let mouse_pos = use_state_eq(|| (0, 0));

    {
        let mouse_pos = mouse_pos.clone();

        use_effect_with_deps(
            move |_| {
                let listener =
                    Closure::<dyn Fn(MouseEvent)>::wrap(Box::new(move |e: MouseEvent| {
                        mouse_pos.set((e.page_x(), e.page_y()));
                    }));
                window()
                    .unwrap()
                    .add_event_listener_with_callback(
                        "mousemove",
                        listener.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                move || drop(listener)
            },
            (),
        );
    }

    *mouse_pos
}
