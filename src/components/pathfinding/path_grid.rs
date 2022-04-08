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
    let onmouseover = move |e: MouseEvent, x, y| {
        e.prevent_default();
        if e.buttons() == 1 {
            on_cell_click.emit(Vertex::new(Coord::new(x, y)));
        }
    };
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
                    for (0..height as usize).map(|y| html! {
                        for (0..width as usize).map(|x| {
                            let onmouseover = {
                                let onmouseover = onmouseover.clone();
                                move |e| onmouseover(e, x as isize, y as isize)
                            };
                            if let Some((vertex, state)) = graph.get_key_value(&Vertex::new(Coord::new(x as isize, y as isize))) {
                                if *vertex != props.start && *vertex != props.end {
                                    match state {
                                        VertexState::NotVisited => html! {
                                            <rect
                                                class="grid-cell unvisited"
                                                key={format!("{},{}", x, y)}
                                                x={(x * SCALE).to_string()}
                                                y={(y * SCALE).to_string()}
                                                width={SCALE.to_string()}
                                                height={SCALE.to_string()}
                                                onmousedown={onmouseover.clone()}
                                                onmouseover={onmouseover.clone()}
                                            />
                                        },
                                        VertexState::NewVisited => html! {
                                            <rect
                                                class="grid-cell new-visited"
                                                key={format!("{},{}", x, y)}
                                                x={(x * SCALE).to_string()}
                                                y={(y * SCALE).to_string()}
                                                width={SCALE.to_string()}
                                                height={SCALE.to_string()}
                                                onmousedown={onmouseover.clone()}
                                                onmouseover={onmouseover.clone()}
                                            />
                                        },
                                        VertexState::Visited => html! {
                                            <rect
                                                class="grid-cell visited"
                                                key={format!("{},{}", x, y)}
                                                x={(x * SCALE).to_string()}
                                                y={(y * SCALE).to_string()}
                                                width={SCALE.to_string()}
                                                height={SCALE.to_string()}
                                                onmousedown={onmouseover.clone()}
                                                onmouseover={onmouseover.clone()}
                                            />
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
                        let (x, y) = (vertex.name.x as usize, vertex.name.y as usize);
                        let onmouseover = {
                            let onmouseover = onmouseover.clone();
                            move |e| onmouseover(e, x as isize, y as isize)
                        };

                        html! {
                            <rect
                                class="grid-cell wall"
                                key={format!("{},{}", x, y)}
                                x={(x * SCALE).to_string()}
                                y={(y * SCALE).to_string()}
                                width={SCALE.to_string()}
                                height={SCALE.to_string()}
                                onmousedown={onmouseover.clone()}
                                onmouseover={onmouseover.clone()}
                            />
                        }
                    })
                }

                <path class="path" d={(*path_str).clone()} stroke-width={(SCALE  / 2).to_string()} />

                <rect
                    class="grid-cell start"
                    key={format!("{},{}", start.x, start.y)}
                    x={(start.x as usize * SCALE).to_string()}
                    y={(start.y as usize * SCALE).to_string()}
                    width={SCALE.to_string()}
                    height={SCALE.to_string()}
                    onmousedown={{
                            let onmouseover = onmouseover.clone();
                            move |e| onmouseover(e, start.x, start.y)
                    }}
                    onmouseover={{
                            let onmouseover = onmouseover.clone();
                            move |e| onmouseover(e, start.x, start.y)
                    }}
                />
                <rect
                    class="grid-cell end"
                    key={format!("{},{}", end.x, end.y)}
                    x={(end.x as usize * SCALE).to_string()}
                    y={(end.y as usize * SCALE).to_string()}
                    width={SCALE.to_string()}
                    height={SCALE.to_string()}
                    onmousedown={{
                            let onmouseover = onmouseover.clone();
                            move |e| onmouseover(e, end.x, end.y)
                    }}
                    onmouseover={{
                            let onmouseover = onmouseover.clone();
                            move |e| onmouseover(e, end.x, end.y)
                    }}
                />
            </svg>
        </div>
    }
}
