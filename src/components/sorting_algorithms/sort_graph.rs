use sorting_algorithms::SortCommand;

// std::time isn't supported on WASM platforms
use instant::{Duration, Instant};
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast, JsValue,
};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;
use yew_hooks::use_size;

#[wasm_bindgen]
extern "C" {
    fn setTimeout(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
    fn clearTimeout(timeout_id: i32);
}

#[derive(Properties, PartialEq)]
pub struct SortGraphProps {
    pub items: Vec<u32>,
    #[prop_or(vec![])]
    pub step: Vec<SortCommand<u32>>,
    #[prop_or(None)]
    pub prev_step: Option<Vec<SortCommand<u32>>>,
}

#[derive(Clone, PartialEq)]
pub struct SortGraphConfig {
    color_changed: String,
    color_unchanged: String,
    update_rate: Duration,
}

#[function_component(SortGraph)]
pub fn sort_graph(props: &SortGraphProps) -> Html {
    let canvas_ref = use_node_ref();
    let canvas = use_state_eq(|| None);
    let ctx: UseStateHandle<Option<CanvasRenderingContext2d>> = use_state_eq(|| None);
    let canvas_size = use_size(canvas_ref.clone());
    let render_timeout_id: UseStateHandle<Option<i32>> = use_state_eq(|| None);
    let _render_timeout_closure: UseStateHandle<Option<Closure<dyn FnMut()>>> = use_state(|| None);

    let prev_draw = use_state_eq(Instant::now);
    let config = SortGraphConfig {
        color_changed: "#00aaff".to_string(),
        color_unchanged: "#adff2f".to_string(),
        update_rate: Duration::from_millis(50),
    };

    let draw_bars = {
        let items = props.items.clone();
        let ctx = ctx.clone();

        move |indices: &[usize], max_val: u32, width: f64, canvas_height: f64| {
            if let Some(ctx) = ctx.as_ref() {
                ctx.begin_path();
                for i in indices.iter() {
                    let i = *i;
                    let val = items[i] as f64;
                    let x = (width * i as f64) + width * 0.5;
                    let height = val / max_val as f64 * canvas_height;
                    ctx.move_to(x, canvas_height);
                    ctx.line_to(x, canvas_height - height);
                }
                ctx.stroke();
            }
        }
    };

    // Draw the current step's values on the canvas.
    let draw = {
        let items = props.items.clone();
        let step = props.step.clone();
        let canvas = canvas.clone();
        let ctx = ctx.clone();
        let config = config.clone();

        move || {
            if let Some(canvas) = canvas.as_ref() as Option<&HtmlCanvasElement> {
                if let Some(ctx) = ctx.as_ref() {
                    let canvas_width = canvas.width() as f64;
                    let canvas_height = canvas.height() as f64;
                    let max_val = match items.iter().max() {
                        Some(val) => *val,
                        None => 0,
                    };
                    let width = canvas_width / items.len() as f64;
                    let margin = width * 0.1;
                    // Remove margin when it's small enough to avoid problem where some bars have a tiny margin and some don't.
                    let margin = if margin < 0.5 { 0.0 } else { margin };

                    let unchanged_indices = (0..items.len())
                        .filter(|i| {
                            !step.iter().any(|command| match command {
                                SortCommand::Swap(from, to) => from == i || to == i,
                                SortCommand::Set(index, _) => index == i,
                            })
                        })
                        .to_owned()
                        .collect::<Vec<usize>>();

                    ctx.clear_rect(0.0, 0.0, canvas_width, canvas_height);
                    ctx.set_line_width(width - margin);

                    set_stroke_style(ctx, config.color_unchanged);
                    draw_bars(&unchanged_indices, max_val, width, canvas_height);

                    set_stroke_style(ctx, config.color_changed);
                    draw_bars(
                        &step
                            .iter()
                            .map(|command| match command.to_owned() {
                                SortCommand::Swap(from, to) => vec![from, to],
                                SortCommand::Set(index, _) => vec![index],
                            })
                            .collect::<Vec<Vec<usize>>>()
                            .concat(),
                        max_val,
                        width,
                        canvas_height,
                    );
                }
            }
        }
    };

    // Set canvas and canvas ctx, draw initial
    if (*canvas).is_none() {
        if let Some(canvas_el) = canvas_ref.cast::<HtmlCanvasElement>() {
            ctx.set(Some(
                canvas_el
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into()
                    .unwrap(),
            ));
            canvas_el.set_width(canvas_size.0);
            canvas_el.set_height(canvas_size.1);
            draw.clone()();

            canvas.set(Some(canvas_el));
        }
    }

    {
        let draw = draw.clone();

        // Draw step with debounce. If timer has elapsed, draw, else draw after timeout.
        use_effect_with_deps(
            move |_| {
                // Limit rate of redraws
                if prev_draw.elapsed() > config.update_rate {
                    draw.clone()();
                    prev_draw.set(Instant::now());
                } else {
                    if let Some(id) = *render_timeout_id {
                        clearTimeout(id)
                    }

                    let cb = Closure::wrap(Box::new(move || {
                        draw.clone()();
                    }) as Box<dyn FnMut()>);

                    let timeout_id = setTimeout(&cb, config.update_rate.as_millis() as u32);

                    render_timeout_id.set(Some(timeout_id));
                    _render_timeout_closure.set(Some(cb));
                };
                || ()
            },
            (props.items.clone(), props.step.clone()),
        );
    }

    use_effect_with_deps(
        move |size| {
            if let Some(canvas) = &*canvas {
                canvas.set_width(size.0);
                canvas.set_height(size.1);
                draw.clone()();
            }
            || ()
        },
        canvas_size,
    );

    html! {
        <canvas class="sort-graph" ref={canvas_ref.clone()}></canvas>
    }
}

fn set_stroke_style(ctx: &CanvasRenderingContext2d, stroke_style: String) {
    ctx.set_stroke_style(&JsValue::from_str(&stroke_style));
}
