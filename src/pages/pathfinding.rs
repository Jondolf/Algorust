use crate::components::{
    collapsible::Collapsible,
    pathfinding::{
        toolbar::{PathTool, PathToolbar},
        PathGrid, PathfindingControls,
    },
    sidebar::Sidebar,
    step_slider::StepSlider,
};
use pathfinding::{
    algorithms, generate_graph,
    graph::{AdjacencyList, Vertex},
    run_pathfinding, Coord, PathfindingResult, PathfindingStep, PathfindingSteps, VertexState,
};
use std::collections::{BTreeMap, BTreeSet};
use yew::prelude::*;
use yew_hooks::use_title;
use yew_router::prelude::*;

type PathfindingFunc = fn(
    AdjacencyList<Coord, isize>,
    Vertex<Coord>,
    Vertex<Coord>,
    PathfindingSteps<Coord>,
) -> PathfindingResult<Coord>;

#[derive(Clone, Debug, PartialEq)]
pub struct PathfindingAlgorithm {
    pub name: String,
    find_path: PathfindingFunc,
}
impl PathfindingAlgorithm {
    fn new(name: &str, find_path: PathfindingFunc) -> Self {
        Self {
            name: name.to_string(),
            find_path,
        }
    }
    /// Finds a path from `start` to `end`.
    /// The path is not guaranteed to be the shortest path depending on the algorithm.
    fn find_path(
        &self,
        graph: &AdjacencyList<Coord, isize>,
        start: Vertex<Coord>,
        end: Vertex<Coord>,
    ) -> (PathfindingResult<Coord>, instant::Duration) {
        run_pathfinding(graph, start, end, self.find_path)
    }
}
impl Default for PathfindingAlgorithm {
    fn default() -> Self {
        Self {
            name: String::from("Dijkstra"),
            find_path: algorithms::dijkstra::<Coord, isize>,
        }
    }
}

pub fn get_pathfinding_algorithms() -> BTreeMap<&'static str, PathfindingAlgorithm> {
    // `BTreeMap` because it keeps the order of the items.
    BTreeMap::from([
        (
            "dijkstra",
            PathfindingAlgorithm::new("Dijkstra", algorithms::dijkstra),
        ),
        ("dfs", PathfindingAlgorithm::new("DFS", algorithms::dfs)),
    ])
}

#[derive(Clone, Debug, Routable, PartialEq)]
pub enum PathfindingRoute {
    #[at("/pathfinding")]
    Pathfinding,
    #[at("/pathfinding/:algorithm")]
    PathfindingAlgorithm { algorithm: String },
}

pub fn switch_pathfinding(route: &PathfindingRoute) -> Html {
    match route {
        PathfindingRoute::Pathfinding => html! {
            <Redirect<PathfindingRoute> to={PathfindingRoute::PathfindingAlgorithm { algorithm: "dfs".to_string()} } />
        },
        PathfindingRoute::PathfindingAlgorithm { algorithm } => {
            if get_pathfinding_algorithms().contains_key(algorithm.as_str()) {
                html! {
                    <PathfindingPage algorithm={algorithm.to_string()} />
                }
            } else {
                html! {
                    <Pathfinding404Page algorithm={algorithm.to_string()} />
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PathfindingConfig {
    pub algorithm: PathfindingAlgorithm,
    pub graph_width: usize,
    pub graph_height: usize,
    pub move_diagonally: bool,
    pub start: Vertex<Coord>,
    pub end: Vertex<Coord>,
    pub active_tool: PathTool,
    pub playback_time: f32,
}
impl Default for PathfindingConfig {
    fn default() -> Self {
        Self {
            algorithm: PathfindingAlgorithm::default(),
            graph_width: 40,
            graph_height: 20,
            move_diagonally: false,
            start: Vertex::new(Coord::new(3, 4)),
            end: Vertex::new(Coord::new(19, 16)),
            active_tool: PathTool::Wall,
            playback_time: 5.0,
        }
    }
}

pub enum PathfindingConfigUpdate {
    UpdatePath,
    UpdatePathAndGraph,
    NoUpdate,
}

#[derive(Properties, Clone, PartialEq)]
pub struct PathfindingPageProps {
    #[prop_or("dijkstra".to_string())]
    pub algorithm: String,
}

#[function_component(PathfindingPage)]
pub fn pathfinding_algorithms_page(props: &PathfindingPageProps) -> Html {
    let config = {
        let algorithm_name = props.algorithm.to_string();

        use_state(|| {
            let mut config = PathfindingConfig::default();
            if let Some(algorithm) = get_pathfinding_algorithms().get(algorithm_name.as_str()) {
                config.algorithm = algorithm.to_owned();
            }
            config
        })
    };

    let graph = use_state(|| {
        generate_graph(
            config.graph_width,
            config.graph_height,
            config.move_diagonally,
        )
    });
    let path = use_state(BTreeSet::<Vertex<Coord>>::new);
    let steps = use_state(Vec::<PathfindingStep<Coord>>::new);
    let active_step_index = use_state(|| 0);
    let graph_at_active_step = use_state(BTreeMap::<Vertex<Coord>, VertexState>::new);

    let update_graph_at_active_step = {
        let steps = steps.clone();
        let graph_at_active_step = graph_at_active_step.clone();

        move |graph: &AdjacencyList<Coord, isize>, step_index| {
            let mut new = BTreeMap::new();
            for v in graph.hash_map.keys() {
                new.insert(*v, VertexState::NotVisited);
            }
            for step in steps[0..step_index].iter() {
                for (vertex, state) in step.states.iter() {
                    new.insert(*vertex, *state);
                }
            }
            graph_at_active_step.set(new);
        }
    };

    let update_path = {
        let steps = steps.clone();
        let path = path.clone();
        let (start, end) = (config.start, config.end);
        let active_step_index = active_step_index.clone();
        let update_graph_at_active_step = update_graph_at_active_step.clone();

        move |graph: &AdjacencyList<Coord, isize>, config: &PathfindingConfig| {
            let (res, _) = config.algorithm.find_path(graph, start, end);
            path.set(res.path);
            steps.set(res.steps.get_all());
            update_graph_at_active_step(graph, *active_step_index);
        }
    };

    let update_config = {
        let config = config.clone();
        let graph = graph.clone();
        let active_step_index = active_step_index.clone();
        let update_path = update_path.clone();
        let update_graph_at_active_step = update_graph_at_active_step.clone();

        Callback::from(
            move |(new_config, update_type): (PathfindingConfig, PathfindingConfigUpdate)| {
                match update_type {
                    PathfindingConfigUpdate::UpdatePath => {
                        update_path(&*graph, &new_config);
                        active_step_index.set(0);
                    }
                    PathfindingConfigUpdate::UpdatePathAndGraph => {
                        let new_graph = generate_graph(
                            new_config.graph_width,
                            new_config.graph_height,
                            new_config.move_diagonally,
                        );
                        update_path(&new_graph, &new_config);
                        update_graph_at_active_step(&new_graph, 0);
                        graph.set(new_graph);
                        active_step_index.set(0);
                    }
                    PathfindingConfigUpdate::NoUpdate => (),
                }
                config.set(new_config);
            },
        )
    };

    {
        let (start, end) = (config.start, config.end);
        let config = (*config).clone();
        let graph = graph.clone();
        let update_path = update_path.clone();
        let active_step_index = active_step_index.clone();
        let update_graph_at_active_step = update_graph_at_active_step.clone();

        use_effect_with_deps(
            move |_| {
                update_path(&*graph, &config);
                update_graph_at_active_step(&*graph, 0);
                active_step_index.set(0);
                || ()
            },
            (start, end),
        )
    }

    let route = use_route::<PathfindingRoute>();

    {
        let config = config.clone();
        let update_config = update_config.clone();
        let graph = graph.clone();
        let update_graph_at_active_step = update_graph_at_active_step.clone();

        use_effect_with_deps(
            move |route| {
                let algorithm_name = match route.as_ref().unwrap() {
                    PathfindingRoute::PathfindingAlgorithm { algorithm } => algorithm,
                    _ => "dijkstra",
                };
                if let Some(algorithm) = get_pathfinding_algorithms().get(algorithm_name) {
                    update_config.emit((
                        PathfindingConfig {
                            algorithm: algorithm.to_owned(),
                            ..*config
                        },
                        PathfindingConfigUpdate::UpdatePath,
                    ));
                    update_graph_at_active_step(&*graph, 0);
                } else {
                    update_config.emit(((*config).clone(), PathfindingConfigUpdate::NoUpdate));
                };
                || ()
            },
            route,
        );
    }

    use_title(format!("{} - Pathfinding", config.algorithm.name));

    let change_step = {
        let graph = graph.clone();
        let active_step_index = active_step_index.clone();
        let update_graph_at_active_step = update_graph_at_active_step.clone();

        Callback::from(move |val| {
            active_step_index.set(val);
            update_graph_at_active_step(&*graph, val);
        })
    };

    let on_tool_change = {
        let config = config.clone();
        Callback::from(move |active_tool| {
            config.set(PathfindingConfig {
                active_tool,
                ..(*config).clone()
            });
        })
    };

    let on_cell_click = {
        let config = config.clone();
        let active_step_index = active_step_index.clone();

        Callback::from(move |vertex| {
            if vertex != config.start && vertex != config.end {
                if graph.hash_map.contains_key(&vertex) {
                    match config.active_tool {
                        PathTool::Start => {
                            config.set(PathfindingConfig {
                                start: vertex,
                                ..(*config).clone()
                            });
                        }
                        PathTool::End => {
                            config.set(PathfindingConfig {
                                end: vertex,
                                ..(*config).clone()
                            });
                        }
                        PathTool::Wall => {
                            let mut new_graph = (*graph).clone();
                            new_graph.remove_vertex(&vertex);
                            update_path(&new_graph, &config);
                            update_graph_at_active_step(&new_graph, 0);
                            active_step_index.set(0);
                            graph.set(new_graph);
                        }
                    }
                } else if config.active_tool == PathTool::Wall {
                    let mut new_graph = (*graph).clone();
                    let vertex_cost = vertex.name.x + vertex.name.y;

                    let mut neighbors = BTreeMap::new();
                    for coord in vertex.name.adjacent(config.move_diagonally) {
                        if new_graph.hash_map.contains_key(&Vertex::new(coord)) {
                            neighbors.insert(Vertex::new(coord), vertex_cost + coord.x + coord.y);
                        }
                    }

                    new_graph.add_vertex_with_undirected_edges(vertex, neighbors);
                    update_path(&new_graph, &config);
                    update_graph_at_active_step(&new_graph, 0);
                    active_step_index.set(0);
                    graph.set(new_graph);
                }
            }
        })
    };

    html! {
        <div class="page" id="Pathfinding">
            <Sidebar>
                <h2>{"Config"}</h2>

                <Collapsible title="General" open={true} class="config-section">
                    <PathfindingControls config={(*config).clone()} {update_config} />
                </Collapsible>
            </Sidebar>

            <main>
                <div class="visualization">
                    <PathToolbar active_tool={config.active_tool} {on_tool_change} />
                    <PathGrid
                        width={config.graph_width}
                        height={config.graph_height}
                        graph={(*graph_at_active_step).clone()}
                        path={
                            if *active_step_index >= steps.len() {
                                (*path).clone()
                            } else {
                                BTreeSet::new()
                            }
                        }
                        start={config.start}
                        end={config.end}
                        {on_cell_click}
                    />

                    <StepSlider
                        active_step_index={*active_step_index}
                        max={steps.len()}
                        on_change={change_step}
                        playback_time={config.playback_time}
                    />
                </div>
            </main>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct Pathfinding404PageProps {
    algorithm: String,
}

#[function_component(Pathfinding404Page)]
fn pathfinding_404_page(props: &Pathfinding404PageProps) -> Html {
    use_title("404 - Pathfinding algorithms".to_string());

    html! {
        <>
            <h1>{ "404" }</h1>
            <p>{ format!("The algorithm \"{}\" was not found.", props.algorithm) }</p>
            <Link<PathfindingRoute> to={PathfindingRoute::Pathfinding}>
                { "Back to pathfinding algorithms" }
            </Link<PathfindingRoute>>
        </>
    }
}
