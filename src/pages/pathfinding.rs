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
    algorithms, generate_graph, graph::AdjacencyList, run_pathfinding, Coord, Edge,
    PathfindingResult, PathfindingStep, PathfindingSteps, Vertex, VertexState,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};
use yew::prelude::*;
use yew_hooks::use_title;
use yew_router::prelude::*;

pub type EdgeType = isize;

type PathfindingFunc<V, E> =
    fn(AdjacencyList<V, E>, V, V, PathfindingSteps<V>) -> PathfindingResult<V, E>;

#[derive(Clone, Debug, PartialEq)]
pub struct PathfindingAlgorithm<V: Vertex, E: Edge> {
    pub name: String,
    find_path: PathfindingFunc<V, E>,
}
impl<V: Vertex, E: Edge> PathfindingAlgorithm<V, E> {
    pub fn new(name: &str, find_path: PathfindingFunc<V, E>) -> Self {
        Self {
            name: name.to_string(),
            find_path,
        }
    }
    /// Finds a path from `start` to `end`.
    /// The path is not guaranteed to be the shortest path depending on the algorithm.
    pub fn find_path(
        &self,
        graph: &AdjacencyList<V, E>,
        start: V,
        end: V,
    ) -> (PathfindingResult<V, E>, instant::Duration) {
        run_pathfinding(graph, start, end, self.find_path)
    }
}
impl<E: Edge> Default for PathfindingAlgorithm<Coord, E> {
    fn default() -> Self {
        Self {
            name: String::from("Dijkstra"),
            find_path: algorithms::dijkstra::<Coord, E>,
        }
    }
}

pub fn get_pathfinding_algorithms<V: Vertex, E: Edge>(
) -> BTreeMap<&'static str, PathfindingAlgorithm<V, E>> {
    // `BTreeMap` because it keeps the order of the items.
    BTreeMap::from([
        (
            "a*",
            PathfindingAlgorithm::new("A*", algorithms::a_star::<V, E>),
        ),
        (
            "dijkstra",
            PathfindingAlgorithm::new("Dijkstra", algorithms::dijkstra::<V, E>),
        ),
        (
            "dfs",
            PathfindingAlgorithm::new("DFS", algorithms::dfs::<V, E>),
        ),
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
            if get_pathfinding_algorithms::<Coord, EdgeType>().contains_key(algorithm.as_str()) {
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
pub struct PathfindingConfig<E: Edge> {
    pub algorithm: PathfindingAlgorithm<Coord, E>,
    pub graph_width: usize,
    pub graph_height: usize,
    pub move_diagonally: bool,
    pub start: Coord,
    pub end: Coord,
    pub active_tool: PathTool,
    pub playback_time: f32,
}
impl<E: Edge> Default for PathfindingConfig<E> {
    fn default() -> Self {
        Self {
            algorithm: PathfindingAlgorithm::default(),
            graph_width: 40,
            graph_height: 20,
            move_diagonally: false,
            start: Coord::new(3, 4),
            end: Coord::new(19, 16),
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

    let walls = use_state(BTreeSet::<Coord>::new);
    let graph = use_state(|| {
        generate_graph::<EdgeType>(
            config.graph_width,
            config.graph_height,
            config.move_diagonally,
            (*walls).clone(),
        )
    });
    let path = use_state(Vec::<Coord>::new);
    let steps = use_state(Vec::<PathfindingStep<Coord>>::new);
    let active_step_index = use_state(|| 0);
    let graph_at_active_step = use_state(BTreeMap::<Coord, VertexState>::new);
    let paused = use_state_eq(|| false);

    // If the new step count is lower than the current step index, the step index is set to the step count. Otherwise it will reset to the given index.
    let update_or_reset_step_index = {
        let step_count = steps.len();
        let active_step_index = active_step_index.clone();

        move |new_step_count: usize, reset_to: usize| {
            let new_active_step_index = if *active_step_index >= step_count {
                new_step_count
            } else {
                reset_to
            };
            active_step_index.set(new_active_step_index);
            new_active_step_index
        }
    };

    let update_path = {
        let steps = steps.clone();
        let path = path.clone();
        let (start, end) = (config.start, config.end);
        let graph_at_active_step = graph_at_active_step.clone();
        let update_or_reset_step_index = update_or_reset_step_index.clone();

        move |graph: &AdjacencyList<Coord, EdgeType>, config: &PathfindingConfig<EdgeType>| {
            let (res, _) = config.algorithm.find_path(graph, start, end);
            let new_steps = res.steps.clone().get_all();
            let new_active_step_index = update_or_reset_step_index(new_steps.len(), 0);
            graph_at_active_step.set(get_graph_at_step(graph, &new_steps, new_active_step_index));
            path.set(res.path.clone());
            steps.set(new_steps);
            res
        }
    };

    let update_config = {
        let config = config.clone();
        let graph = graph.clone();
        let walls = walls.clone();
        let steps = steps.clone();
        let active_step_index = active_step_index.clone();
        let graph_at_active_step = graph_at_active_step.clone();
        let update_or_reset_step_index = update_or_reset_step_index.clone();
        let update_path = update_path.clone();

        Callback::from(
            move |(new_config, update_type): (
                PathfindingConfig<EdgeType>,
                PathfindingConfigUpdate,
            )| {
                match update_type {
                    PathfindingConfigUpdate::UpdatePath => {
                        update_path(&graph, &new_config);
                        active_step_index.set(0);
                    }
                    PathfindingConfigUpdate::UpdatePathAndGraph => {
                        let new_graph = generate_graph(
                            new_config.graph_width,
                            new_config.graph_height,
                            new_config.move_diagonally,
                            (*walls).clone(),
                        );
                        let res = update_path(&new_graph, &new_config);
                        let new_active_step_index = update_or_reset_step_index(res.steps.len(), 0);

                        graph_at_active_step.set(get_graph_at_step(
                            &new_graph,
                            &*steps,
                            new_active_step_index,
                        ));
                        graph.set(new_graph);
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
        let algorithm = config.algorithm.clone();
        let graph = graph.clone();
        let update_path = update_path.clone();

        use_effect_with_deps(
            move |_| {
                update_path(&*graph, &config);
                || ()
            },
            (start, end, algorithm),
        )
    }

    let route = use_route::<PathfindingRoute>();

    {
        let config = config.clone();
        let update_config = update_config.clone();
        let graph = graph.clone();
        let graph_at_active_step = graph_at_active_step.clone();
        let steps = steps.clone();

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
                    graph_at_active_step.set(get_graph_at_step(&*graph, &*steps, 0));
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
        let graph_at_active_step = graph_at_active_step.clone();
        let steps = steps.clone();
        let active_step_index = active_step_index.clone();

        Callback::from(move |val| {
            active_step_index.set(val);
            graph_at_active_step.set(get_graph_at_step(&graph, &*steps, val));
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
        let graph = graph.clone();
        let walls = walls.clone();
        let paused = paused.clone();

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
                            let mut new_walls = (*walls).clone();
                            new_walls.insert(vertex);
                            walls.set(new_walls);
                            paused.set(true);
                        }
                    }
                } else if config.active_tool == PathTool::Wall {
                    let mut new_walls = (*walls).clone();
                    new_walls.remove(&vertex);
                    walls.set(new_walls);
                    paused.set(true);
                }
            }
        })
    };
    let on_draw_end = {
        let config = config.clone();
        let steps = steps.clone();
        let graph_at_active_step = graph_at_active_step.clone();
        let walls = walls.clone();
        let paused = paused.clone();

        Callback::from(move |_| {
            if config.active_tool == PathTool::Wall {
                let new_graph = generate_graph(
                    config.graph_width,
                    config.graph_height,
                    config.move_diagonally,
                    (*walls).clone(),
                );

                let res = update_path(&new_graph, &config);
                let new_active_step_index = update_or_reset_step_index(res.steps.len(), 0);

                graph_at_active_step.set(get_graph_at_step(
                    &new_graph,
                    &*steps,
                    new_active_step_index,
                ));
                graph.set(new_graph);
            }
            paused.set(false);
        })
    };

    html! {
        <div class="page" id="Pathfinding">
            <Sidebar>
                <h2>{"Config"}</h2>

                <Collapsible title="General" open={true} class="config-section">
                    <PathfindingControls<EdgeType> config={(*config).clone()} {update_config} />
                </Collapsible>
            </Sidebar>

            <main>
                <div class="visualization">
                    <PathToolbar active_tool={config.active_tool} {on_tool_change} />
                    <PathGrid
                        width={config.graph_width}
                        height={config.graph_height}
                        graph={(*graph_at_active_step).clone()}
                        walls={(*walls).clone()}
                        path={
                            if *active_step_index >= steps.len() {
                                (*path).clone()
                            } else {
                                Vec::new()
                            }
                        }
                        start={config.start}
                        end={config.end}
                        {on_cell_click}
                        {on_draw_end}
                    />

                    <StepSlider
                        active_step_index={*active_step_index}
                        max={steps.len()}
                        on_change={change_step}
                        playback_time={config.playback_time}
                        disabled={*paused}
                    />
                </div>
            </main>
        </div>
    }
}

fn get_graph_at_step<V: Vertex, E: Clone>(
    graph: &AdjacencyList<V, E>,
    steps: &[PathfindingStep<V>],
    step_index: usize,
) -> BTreeMap<V, VertexState> {
    let mut new = BTreeMap::new();
    for v in graph.hash_map.keys() {
        new.insert(*v, VertexState::NotVisited);
    }
    for step in steps[0..step_index.min(steps.len())].iter() {
        for (vertex, state) in step.states.iter() {
            new.insert(*vertex, *state);
        }
    }
    new
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
