use pathfinding::{graph::Vertex, Coord, VertexState};

use std::collections::{BTreeMap, BTreeSet};

use yew::prelude::*;

const SCALE: usize = 100;

#[derive(Properties, Clone, PartialEq)]
pub struct PathGridProps {
    pub width: usize,
    pub height: usize,
    pub graph: BTreeMap<Vertex<Coord>, VertexState>,
    pub walls: BTreeSet<Vertex<Coord>>,
    pub path: Vec<Vertex<Coord>>,
    pub start: Vertex<Coord>,
    pub end: Vertex<Coord>,
    pub on_cell_click: Callback<Vertex<Coord>>,
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
    let (start, end) = (props.start.name, props.end.name);
    let onmouseover = Callback::from(move |(e, x, y): (MouseEvent, isize, isize)| {
        e.prevent_default();
        if e.buttons() == 1 {
            on_cell_click.emit(Vertex::new(Coord::new(x, y)));
        }
    });
    let path_str = use_state_eq(String::new);

    {
        let path_str = path_str.clone();

        use_effect_with_deps(
            move |path| {
                let mut new_path_str = String::new();

                if !path.is_empty() {
                    new_path_str += &format!(
                        "M {} {}",
                        path[0].name.x as usize * SCALE + SCALE / 2,
                        path[0].name.y as usize * SCALE + SCALE / 2
                    );

                    for vertex in path[1..path.len()].iter() {
                        let vertex = *vertex;
                        let (x, y) = (vertex.name.x as usize, vertex.name.y as usize);
                        new_path_str +=
                            &format!(" L {} {}", x * SCALE + SCALE / 2, y * SCALE + SCALE / 2);
                    }
                }

                path_str.set(new_path_str);

                || ()
            },
            path,
        );
    }

    html! {
        <div class="path-grid" style={format!("aspect-ratio: {}/{}", width, height)} onmouseup={move |_| on_draw_end.emit(())}>
            <svg viewBox={format!("0 0 {} {}", width * SCALE, height * SCALE)} xmlns="http://www.w3.org/2000/svg">
                // Start, end and visited cells
                {
                    for (0..height as isize).map(|y| html! {
                        for (0..width as isize).map(|x| {
                            if let Some((vertex, state)) = graph.get_key_value(&Vertex::new(Coord::new(x, y))) {
                                if *vertex != props.start && *vertex != props.end {
                                    match state {
                                        VertexState::NotVisited => html! {
                                            <GridCell class="unvisited" {x} {y} onmouseover={onmouseover.clone()} />
                                        },
                                        VertexState::NewVisited => html! {
                                            <GridCell class="new-visited" {x} {y} onmouseover={onmouseover.clone()} />
                                        },
                                        VertexState::Visited => html! {
                                            <GridCell class="visited" {x} {y} onmouseover={onmouseover.clone()} />
                                        },
                                    }
                                } else {
                                    html! { }
                                }
                            } else {
                                html! { }
                            }
                        })
                    })
                }

                // Walls
                {
                    for walls.into_iter().map(|vertex| {
                        let (x, y) = (vertex.name.x, vertex.name.y);
                        html! {
                            <GridCell class="wall" {x} {y} onmouseover={onmouseover.clone()} />
                        }
                    })
                }

                <path class="path" d={(*path_str).clone()} stroke-width={(SCALE  / 2).to_string()} />

                <GridCell class="start" x={start.x} y={start.y} onmouseover={onmouseover.clone()} />
                <GridCell class="end" x={end.x} y={end.y} onmouseover={onmouseover.clone()} />
            </svg>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct GridCellProps {
    x: isize,
    y: isize,
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
