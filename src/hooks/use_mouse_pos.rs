use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::window;
use yew::prelude::*;

pub fn use_mouse_pos() -> (i32, i32) {
    let mouse_pos = use_state_eq(|| (0, 0));

    {
        let mouse_pos = mouse_pos.clone();
        let touch_pos = mouse_pos.clone();

        use_effect_with_deps(
            move |_| {
                let pointermove_cb =
                    Closure::<dyn Fn(PointerEvent)>::wrap(Box::new(move |e: PointerEvent| {
                        mouse_pos.set((e.page_x(), e.page_y()));
                    }));
                let touchmove_cb =
                    Closure::<dyn Fn(TouchEvent)>::wrap(Box::new(move |e: TouchEvent| {
                        touch_pos.set((
                            e.touches().item(0).unwrap().page_x(),
                            e.touches().item(0).unwrap().page_y(),
                        ));
                    }));

                window()
                    .unwrap()
                    .add_event_listener_with_callback(
                        "pointermove",
                        pointermove_cb.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                window()
                    .unwrap()
                    .add_event_listener_with_callback(
                        "touchmove",
                        touchmove_cb.as_ref().unchecked_ref(),
                    )
                    .unwrap();

                move || {
                    drop(pointermove_cb);
                    drop(touchmove_cb);
                }
            },
            (),
        );
    }

    *mouse_pos
}
