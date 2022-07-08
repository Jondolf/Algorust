use pathfinding::{Coord, VertexState};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
};
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew_hooks::use_size;

use yew::prelude::*;

const SCALE: usize = 100;

#[derive(Properties, Clone, PartialEq)]
pub struct PathGridProps {
    pub width: usize,
    pub height: usize,
    pub graph: BTreeMap<Coord, VertexState>,
    pub walls: BTreeSet<Coord>,
    pub path: Vec<Coord>,
    pub start: Coord,
    pub end: Coord,
    pub on_cell_click: Callback<Coord>,
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
        on_cell_click,
        on_draw_end,
        ..
    } = props.clone();
    let (start, end) = (props.start, props.end);

    let path_grid_container_ref = use_node_ref();
    let path_grid_container_size = use_size(path_grid_container_ref.clone());

    let path_grid_ref = use_node_ref();

    let onmouseover = Callback::from(move |(e, x, y): (MouseEvent, isize, isize)| {
        e.prevent_default();
        if e.buttons() == 1 {
            on_cell_click.emit(Coord::new(x, y));
        }
    });
    let path_str = use_state_eq(String::new);

    {
        let path_str = path_str.clone();

        use_effect_with_deps(
            move |path| {
                let mut new_path_str = String::new();

                if !path.is_empty() {
                    let _ = write!(
                        &mut new_path_str,
                        "M {} {}",
                        path[0].x as usize * SCALE + SCALE / 2,
                        path[0].y as usize * SCALE + SCALE / 2
                    );

                    for vertex in path[1..path.len()].iter() {
                        let vertex = *vertex;
                        let (x, y) = (vertex.x as usize, vertex.y as usize);
                        let _ = write!(
                            &mut new_path_str,
                            " L {} {}",
                            x * SCALE + SCALE / 2,
                            y * SCALE + SCALE / 2
                        );
                    }
                }

                path_str.set(new_path_str);

                || ()
            },
            path,
        );
    }

    let onmouseover = {
        let path_grid_ref = path_grid_ref.clone();

        move |e: MouseEvent| {
            let el = path_grid_ref.get().unwrap().dyn_into::<Element>().unwrap();
            let (x_px, y_px) = ((e.offset_x()) as f32, (e.offset_y()) as f32);
            let (rect_width_px, rect_height_px) = (
                el.client_width() as f32 / width as f32,
                el.client_height() as f32 / height as f32,
            );
            let (x, y) = (
                (x_px / rect_width_px).floor() as isize,
                (y_px / rect_height_px).floor() as isize,
            );
            onmouseover.emit((e, x, y))
        }
    };

    let wall_cells = walls
        .iter()
        .map(|vertex| html! { <GridCell class="wall" x={vertex.x} y={vertex.y } key={format!("{}, {}", vertex.x, vertex.y)} /> })
        .collect::<Html>();

    let visited_cells = (0..height as isize)
        .map(|y| {
            (0..width as isize)
                .map(|x| {
                    if let Some((vertex, state)) = graph.get_key_value(&Coord::new(x, y)) {
                        if *vertex != props.start && *vertex != props.end {
                            match state {
                                VertexState::NewVisited => html! {
                                    <GridCell class="new-visited" {x} {y} key={format!("{}, {}", vertex.x, vertex.y)} />
                                },
                                VertexState::Visited => html! {
                                    <GridCell class="visited" {x} {y} key={format!("{}, {}", vertex.x, vertex.y)} />
                                },
                                _ => html! {},
                            }
                        } else {
                            html! {}
                        }
                    } else {
                        html! {}
                    }
                })
                .collect::<Html>()
        })
        .collect::<Html>();

    html! {
        <div
            ref={path_grid_container_ref}
            class="path-grid"
            style={format!("aspect-ratio: {}/{}", width, height)}
        >
            <svg
                ref={path_grid_ref}
                viewBox={format!("0 0 {} {}", width * SCALE, height * SCALE)}
                xmlns="http://www.w3.org/2000/svg"
                style={format!("max-width: {}px; max-height: {}px", path_grid_container_size.0, path_grid_container_size.1)}
                onmouseup={move |_| on_draw_end.emit(())}
                onmousedown={onmouseover.clone()}
                onmousemove={onmouseover}
            >
                { visited_cells }

                { wall_cells }

                // Grid pattern for the background
                <defs>
                    <pattern id="gridPattern" width={(SCALE).to_string()} height={(SCALE).to_string()} patternUnits="userSpaceOnUse">
                        <path d={format!("M {} 0 L 0 0 0 {}", SCALE, SCALE)} />
                    </pattern>
                </defs>

                // Background grid
                <rect class="grid" fill="url(#gridPattern)" width={(width * SCALE).to_string()} height={(height * SCALE).to_string()} />

                // The path found by the pathfinding algorithm
                <path class="path" d={(*path_str).clone()} stroke-width={(SCALE / 2).to_string()} />

                // Start and end cells
                <GridCell class="start" x={start.x} y={start.y} />
                <GridCell class="end" x={end.x} y={end.y} />
            </svg>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct GridCellProps {
    x: isize,
    y: isize,
    #[prop_or(Callback::from(|_|()))]
    onmouseover: Callback<(MouseEvent, isize, isize)>,
    class: Classes,
}

#[function_component(GridCell)]
fn grid_cell(props: &GridCellProps) -> Html {
    let (x, y) = (props.x, props.y);
    let onmouseover = {
        let onmouseover = props.onmouseover.clone();
        move |e| onmouseover.emit((e, x, y))
    };

    html! {
        <rect
            class={classes!("grid-cell", props.class.clone())}
            key={format!("{},{}", x, y)}
            x={(x as usize * SCALE).to_string()}
            y={(y as usize * SCALE).to_string()}
            width={SCALE.to_string()}
            height={SCALE.to_string()}
            onmousedown={onmouseover.clone()}
            onmouseover={onmouseover}
        />
    }
}
