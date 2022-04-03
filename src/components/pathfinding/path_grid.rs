use pathfinding::{graph::Vertex, Coord, VertexState};

use std::collections::{BTreeMap, BTreeSet};

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PathGridProps {
    pub width: usize,
    pub height: usize,
    pub graph: BTreeMap<Vertex<Coord>, VertexState>,
    pub path: BTreeSet<Vertex<Coord>>,
    pub start: Vertex<Coord>,
    pub end: Vertex<Coord>,
    pub on_cell_click: Callback<Vertex<Coord>>,
}

#[function_component(PathGrid)]
pub fn path_grid(props: &PathGridProps) -> Html {
    let PathGridProps {
        width,
        height,
        graph,
        path,
        on_cell_click,
        ..
    } = props.clone();
    let onmouseover = move |e: MouseEvent, x, y| {
        e.prevent_default();
        if e.buttons() == 1 {
            on_cell_click.emit(Vertex::new(Coord::new(x, y)));
        }
    };

    html! {
        <div class="path-grid" style={format!("aspect-ratio: {}/{}", width, height)}>
            {
                for (0..height as isize).map(|y| html! {
                    for (0..width as isize).map(|x| {
                        let onmouseover = {
                            let onmouseover = onmouseover.clone();
                            move |e| onmouseover(e, x, y)
                        };
                        if let Some((vertex, state)) = graph.get_key_value(&Vertex::new(Coord::new(x, y))) {
                            html! {
                                <div
                                    class={classes!(
                                        "grid-cell",
                                        if *vertex == props.start {
                                            "start"
                                        } else if *vertex == props.end {
                                            "end"
                                        } else if path.contains(vertex) {
                                            "path"
                                        } else {
                                            match state {
                                                VertexState::NotVisited => "unvisited",
                                                VertexState::NewVisited => "new-visited",
                                                VertexState::Visited => "visited",
                                            }
                                        }
                                    )}
                                    key={format!("{},{}", x, y)}
                                    style={format!("grid-column-start: {}; grid-column-end: {}; grid-row-start: {}; grid-row-end: {};",
                                    x + 1,
                                    x + 2,
                                    y + 1,
                                    y + 2)}
                                    onmousedown={onmouseover.clone()}
                                    onmouseover={onmouseover.clone()}
                                ></div>
                            }
                        } else {
                            html! {
                                <div
                                    class="grid-cell wall"
                                    key={format!("{},{}", x, y)}
                                    style={format!("grid-column-start: {}; grid-column-end: {}; grid-row-start: {}; grid-row-end: {};",
                                    x + 1,
                                    x + 2,
                                    y + 1,
                                    y + 2)}
                                    onmousedown={onmouseover.clone()}
                                    onmouseover={onmouseover.clone()}
                                ></div>
                            }
                        }
                    })
                })
            }
        </div>
    }
}
