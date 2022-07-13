use pathfinding::{Coord, VertexState};
use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
};
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsCast,
};
use web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement};
use yew_hooks::use_size;

use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn setTimeout(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
    fn clearTimeout(timeout_id: i32);
}

#[derive(Clone, PartialEq)]
pub struct PathGridConfig {
    color_start: String,
    color_end: String,
    color_wall: String,
    color_new_visited: String,
    color_not_visited: String,
    color_visited: String,
    color_path: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct PathGridProps {
    pub width: usize,
    pub height: usize,
    pub graph: Rc<RefCell<BTreeMap<Coord, VertexState>>>,
    pub walls: Rc<RefCell<BTreeSet<Coord>>>,
    pub path: Option<Rc<RefCell<Vec<Coord>>>>,
    pub start: Coord,
    pub end: Coord,
    pub on_click_cell: Callback<Coord>,
    pub on_draw_end: Callback<()>,
}

#[function_component(PathGrid)]
pub fn path_grid(props: &PathGridProps) -> Html {
    let PathGridProps {
        width,
        height,
        graph,
        walls,
        path,
        on_click_cell,
        on_draw_end,
        ..
    } = props.clone();
    let config = PathGridConfig {
        color_start: "#00ff66".to_string(),
        color_end: "#ff4500".to_string(),
        color_wall: "#cccccc".to_string(),
        color_new_visited: "#00bbff".to_string(),
        color_not_visited: "".to_string(),
        color_visited: "#0066ff".to_string(),
        color_path: "#ffa500".to_string(),
    };
    let (start, end) = (props.start, props.end);

    let canvas_container_ref = use_node_ref();
    let canvas_container_size = use_size(canvas_container_ref.clone());

    let background_canvas_ref = use_node_ref();
    let background_canvas_size = use_size(background_canvas_ref.clone());
    let background_canvas: UseStateHandle<Option<HtmlCanvasElement>> = use_state(|| None);
    let background_ctx: UseStateHandle<Option<CanvasRenderingContext2d>> = use_state(|| None);

    let foreground_canvas_ref = use_node_ref();
    let foreground_canvas: UseStateHandle<Option<HtmlCanvasElement>> = use_state(|| None);
    let foreground_ctx: UseStateHandle<Option<CanvasRenderingContext2d>> = use_state(|| None);

    let wall_canvas_ref = use_node_ref();
    let wall_canvas: UseStateHandle<Option<HtmlCanvasElement>> = use_state(|| None);
    let wall_ctx: UseStateHandle<Option<CanvasRenderingContext2d>> = use_state(|| None);

    // Emit the coordinates of the hovered cell if the mouse button is down
    let oncellclick = Callback::from(move |(e, x, y): (MouseEvent, isize, isize)| {
        e.prevent_default();
        if e.buttons() == 1 {
            on_click_cell.emit(Coord::new(x, y));
        }
    });

    // Draw the current step's values on the canvas.
    let draw_background = {
        let graph = graph.clone();
        let canvas = background_canvas.clone();
        let ctx = background_ctx.clone();
        let config = config.clone();

        move || {
            if let Some(canvas) = canvas.as_ref() as Option<&HtmlCanvasElement> {
                if let Some(ctx) = ctx.as_ref() {
                    let canvas_width = canvas.width() as f64;
                    let canvas_height = canvas.height() as f64;
                    let cell_width = canvas_width / width as f64;
                    let cell_height = canvas_height / height as f64;

                    ctx.clear_rect(0.0, 0.0, canvas_width, canvas_height);

                    ctx.begin_path();
                    ctx.set_fill_style(&config.color_visited.as_str().into());
                    for (vertex, _) in graph
                        .borrow()
                        .iter()
                        .filter(|(_, state)| **state == VertexState::Visited)
                    {
                        ctx.rect(
                            vertex.x as f64 * cell_width,
                            vertex.y as f64 * cell_height,
                            cell_width,
                            cell_height,
                        );
                    }
                    ctx.fill();

                    ctx.begin_path();
                    ctx.set_fill_style(&config.color_new_visited.as_str().into());
                    for (vertex, _) in graph
                        .borrow()
                        .iter()
                        .filter(|(_, state)| **state == VertexState::NewVisited)
                    {
                        ctx.rect(
                            vertex.x as f64 * cell_width,
                            vertex.y as f64 * cell_height,
                            cell_width,
                            cell_height,
                        );
                    }
                    ctx.fill();
                }
            }
        }
    };

    let draw_foreground = {
        let path = path.clone();
        let canvas = foreground_canvas.clone();
        let ctx = foreground_ctx.clone();
        let config = config.clone();

        move || {
            if let Some(canvas) = canvas.as_ref() as Option<&HtmlCanvasElement> {
                if let Some(ctx) = ctx.as_ref() {
                    let canvas_width = canvas.width() as f64;
                    let canvas_height = canvas.height() as f64;
                    let cell_width = canvas_width / width as f64;
                    let cell_height = canvas_height / height as f64;

                    ctx.clear_rect(0.0, 0.0, canvas_width, canvas_height);

                    if let Some(path) = path {
                        if !path.borrow().is_empty() {
                            ctx.set_stroke_style(&config.color_path.as_str().into());
                            ctx.set_line_cap("round");
                            ctx.set_line_join("round");
                            ctx.set_line_width(cell_width * 0.4);

                            ctx.begin_path();

                            ctx.move_to(
                                path.borrow()[0].x as f64 * cell_width + 0.5 * cell_width,
                                path.borrow()[0].y as f64 * cell_height + 0.5 * cell_height,
                            );

                            for coord in path.borrow()[1..].iter() {
                                ctx.line_to(
                                    coord.x as f64 * cell_width + 0.5 * cell_width,
                                    coord.y as f64 * cell_height + 0.5 * cell_height,
                                );
                            }

                            ctx.stroke();
                        }
                    }

                    ctx.set_fill_style(&config.color_start.as_str().into());
                    ctx.fill_rect(
                        start.x as f64 * cell_width,
                        start.y as f64 * cell_height,
                        cell_width,
                        cell_height,
                    );

                    ctx.set_fill_style(&config.color_end.as_str().into());
                    ctx.fill_rect(
                        end.x as f64 * cell_width,
                        end.y as f64 * cell_height,
                        cell_width,
                        cell_height,
                    );
                }
            }
        }
    };

    let draw_walls = {
        let walls = walls.clone();
        let canvas = wall_canvas.clone();
        let ctx = wall_ctx.clone();

        move || {
            if let Some(canvas) = canvas.as_ref() as Option<&HtmlCanvasElement> {
                if let Some(ctx) = ctx.as_ref() {
                    let canvas_width = canvas.width() as f64;
                    let canvas_height = canvas.height() as f64;
                    let cell_width = canvas_width / width as f64;
                    let cell_height = canvas_height / height as f64;

                    ctx.clear_rect(0.0, 0.0, canvas_width, canvas_height);

                    ctx.begin_path();
                    ctx.set_fill_style(&config.color_wall.as_str().into());
                    for vertex in walls.borrow().iter() {
                        ctx.rect(
                            vertex.x as f64 * cell_width,
                            vertex.y as f64 * cell_height,
                            cell_width,
                            cell_height,
                        );
                    }
                    ctx.fill();
                }
            }
        }
    };

    if (*background_canvas).is_none() {
        if let Some(canvas_el) = background_canvas_ref.cast::<HtmlCanvasElement>() {
            background_ctx.set(Some(
                canvas_el
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into()
                    .unwrap(),
            ));

            background_canvas.set(Some(canvas_el));
        }
    }

    if (*foreground_canvas).is_none() {
        if let Some(canvas_el) = foreground_canvas_ref.cast::<HtmlCanvasElement>() {
            foreground_ctx.set(Some(
                canvas_el
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into()
                    .unwrap(),
            ));

            foreground_canvas.set(Some(canvas_el));
        }
    }

    if (*wall_canvas).is_none() {
        if let Some(canvas_el) = wall_canvas_ref.cast::<HtmlCanvasElement>() {
            wall_ctx.set(Some(
                canvas_el
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into()
                    .unwrap(),
            ));

            wall_canvas.set(Some(canvas_el));
        }
    }

    {
        let draw_background = draw_background.clone();

        use_effect_with_deps(
            move |_| {
                draw_background();
                || ()
            },
            graph.borrow().clone(),
        );
    }

    {
        let draw_foreground = draw_foreground.clone();

        use_effect_with_deps(
            move |_| {
                draw_foreground();
                || ()
            },
            (path.map_or(vec![], |p| p.borrow().clone()), start, end),
        );
    }

    {
        let draw_walls = draw_walls.clone();

        use_effect_with_deps(
            move |_| {
                draw_walls();
                || ()
            },
            walls.borrow().clone(),
        );
    }

    use_effect_with_deps(
        move |_| {
            draw_background();
            draw_foreground();
            draw_walls();
            || ()
        },
        (width, height, background_canvas_size),
    );

    let onmouseover = {
        let canvas_ref = wall_canvas_ref.clone();

        move |e: MouseEvent| {
            let el = canvas_ref.get().unwrap().dyn_into::<Element>().unwrap();
            let (x_px, y_px) = ((e.offset_x()) as f32, (e.offset_y()) as f32);
            let (rect_width_px, rect_height_px) = (
                el.client_width() as f32 / width as f32,
                el.client_height() as f32 / height as f32,
            );
            let (x, y) = (
                (x_px / rect_width_px).floor() as isize,
                (y_px / rect_height_px).floor() as isize,
            );
            oncellclick.emit((e, x, y))
        }
    };

    html! {
        <div
            ref={canvas_container_ref}
            class="path-grid"
            style={format!("aspect-ratio: {}/{}", width, height)}
        >
            // Background (visited cells etc.)
            <canvas
                ref={background_canvas_ref}
                style={format!("z-index: 1; aspect-ratio: {} / {}; max-width: {}px; max-height: {}px", width, height, canvas_container_size.0, canvas_container_size.1)}
                width={background_canvas_size.0.to_string()}
                height={background_canvas_size.1.to_string()}
            >
            </canvas>

            // Grid pattern
            <svg
                xmlns="http://www.w3.org/2000/svg"
                style={"z-index: 2"}
                width={background_canvas_size.0.to_string()}
                height={background_canvas_size.1.to_string()}
            >
                <defs>
                    <pattern
                        id="gridPattern"
                        width={(background_canvas_size.0 as f32 / width as f32).to_string()}
                        height={(background_canvas_size.1 as f32 / height as f32).to_string()}
                        patternUnits="userSpaceOnUse"
                    >
                        <path d={format!(
                            "M {} 0 L 0 0 0 {}",
                            background_canvas_size.0 as f32 / width as f32,
                            background_canvas_size.1 as f32 / height as f32
                        )} />
                    </pattern>
                </defs>

                <rect
                    class="grid"
                    fill="url(#gridPattern)"
                    // + 1.5 to make the right and bottom borders visible
                    width={(background_canvas_size.0 as f32 + 1.5).to_string()}
                    height={(background_canvas_size.1 as f32 + 1.5).to_string()}
                />
            </svg>

            <canvas
                ref={foreground_canvas_ref}
                style={format!("z-index: 3; aspect-ratio: {} / {};", width, height, )}
                width={background_canvas_size.0.to_string()}
                height={background_canvas_size.1.to_string()}
            >
            </canvas>
            <canvas
                ref={wall_canvas_ref}
                style={format!("z-index: 4; aspect-ratio: {} / {};", width, height, )}
                width={background_canvas_size.0.to_string()}
                height={background_canvas_size.1.to_string()}
                onmouseup={move |_| on_draw_end.emit(())}
                onmousedown={onmouseover.clone()}
                onmousemove={onmouseover}
            >
            </canvas>
        </div>
    }
}
