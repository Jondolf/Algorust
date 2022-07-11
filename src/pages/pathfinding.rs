use crate::components::{
    collapsible::Collapsible,
    pathfinding::{toolbar::*, *},
    sidebar::Sidebar,
    step_slider::StepSlider,
};
use pathfinding::{
    algorithms, generate_graph,
    graph::AdjacencyList,
    maze_generation::{recursive_division, MazeGenerationResult, MazeGenerationStep},
    run_pathfinding, Coord, Edge, PathfindingResult, PathfindingStep, PathfindingSteps, Vertex,
    VertexState,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    rc::Rc,
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
            graph_width: 25,
            graph_height: 25,
            move_diagonally: false,
            start: Coord::new(2, 2),
            end: Coord::new(22, 22),
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

    let path = use_mut_ref(Vec::<Coord>::new);
    let walls = use_mut_ref(BTreeSet::new);

    let graph = use_mut_ref(|| {
        generate_graph::<EdgeType>(
            config.graph_width,
            config.graph_height,
            config.move_diagonally,
            &walls.borrow(),
        )
    });

    let pathfinding_steps = use_mut_ref(|| PathfindingSteps::<Coord>::new(vec![]));
    let maze_gen_steps = use_mut_ref(Vec::<MazeGenerationStep>::new);

    let graph_at_pathfinding_step = use_mut_ref(BTreeMap::<Coord, VertexState>::new);
    let walls_at_maze_gen_step = use_mut_ref(BTreeSet::new);

    let pathfinding_step_index = use_state(|| 0);
    let maze_gen_step_index = use_state(|| 0);

    let paused = use_state(|| false);

    // This should only be shown after maze generation when the user hasn't drawn any new walls.
    let show_maze_gen_slider = use_state(|| false);

    // If the new step count is lower than the current step index, the step index is set to the step count. Otherwise it will reset to the given index.
    let update_or_reset_pathfinding_step_index = {
        let step_count = pathfinding_steps.borrow().len();
        let pathfinding_step_index = pathfinding_step_index.clone();

        move |new_step_count: usize, reset_to: usize| {
            let new_active_step_index = if *pathfinding_step_index >= step_count {
                new_step_count
            } else {
                reset_to
            };
            pathfinding_step_index.set(new_active_step_index);
            new_active_step_index
        }
    };

    // If the new step count is lower than the current step index, the step index is set to the step count. Otherwise it will reset to the given index.
    let update_or_reset_maze_gen_step_index = {
        let step_count = maze_gen_steps.borrow().len();
        let maze_gen_step_index = maze_gen_step_index.clone();

        move |new_step_count: usize, reset_to: usize| {
            let new_active_step_index = if *maze_gen_step_index >= step_count {
                new_step_count
            } else {
                reset_to
            };
            maze_gen_step_index.set(new_active_step_index);
            new_active_step_index
        }
    };

    let update_pathfinding_step = {
        let graph = Rc::clone(&graph);
        let pathfinding_steps = Rc::clone(&pathfinding_steps);
        let old_pathfinding_step_index = *pathfinding_step_index;
        let path = Rc::clone(&path);
        let (start, end) = (config.start, config.end);
        let graph_at_pathfinding_step = graph_at_pathfinding_step.clone();
        let update_or_reset_pathfinding_step_index = update_or_reset_pathfinding_step_index.clone();

        move |config: &PathfindingConfig<EdgeType>| {
            let (res, _) = config.algorithm.find_path(&graph.borrow(), start, end);
            *path.borrow_mut() = res.path;
            *pathfinding_steps.borrow_mut() = res.steps;

            let new_active_step_index =
                update_or_reset_pathfinding_step_index(pathfinding_steps.borrow().len(), 0);

            update_graph_at_pathfinding_step(
                &mut graph_at_pathfinding_step.borrow_mut(),
                &pathfinding_steps.borrow().steps,
                new_active_step_index,
                old_pathfinding_step_index.min(pathfinding_steps.borrow().len()),
                true,
            );
        }
    };

    let update_config = {
        let config = config.clone();
        let graph = Rc::clone(&graph);
        let walls = Rc::clone(&walls);
        let pathfinding_steps = Rc::clone(&pathfinding_steps);
        let update_or_reset_pathfinding_step_index = update_or_reset_pathfinding_step_index.clone();
        let update_pathfinding_step = update_pathfinding_step.clone();

        Callback::from(
            move |(new_config, update_type): (
                PathfindingConfig<EdgeType>,
                PathfindingConfigUpdate,
            )| {
                match update_type {
                    PathfindingConfigUpdate::UpdatePath => {
                        update_pathfinding_step(&new_config);
                        update_or_reset_pathfinding_step_index(0, 0);
                    }
                    PathfindingConfigUpdate::UpdatePathAndGraph => {
                        let new_graph = generate_graph(
                            new_config.graph_width,
                            new_config.graph_height,
                            new_config.move_diagonally,
                            &walls.borrow(),
                        );
                        *graph.borrow_mut() = new_graph;

                        update_pathfinding_step(&new_config);
                        update_or_reset_pathfinding_step_index(pathfinding_steps.borrow().len(), 0);
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
        let update_pathfinding_step = update_pathfinding_step.clone();

        use_effect_with_deps(
            move |_| {
                update_pathfinding_step(&config);
                || ()
            },
            (start, end, algorithm),
        )
    }

    let route = use_route::<PathfindingRoute>();

    {
        let config = config.clone();
        let update_config = update_config.clone();
        let graph_at_pathfinding_step = graph_at_pathfinding_step.clone();
        let pathfinding_steps = Rc::clone(&pathfinding_steps);

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
                    update_graph_at_pathfinding_step(
                        &mut graph_at_pathfinding_step.borrow_mut(),
                        &pathfinding_steps.borrow().steps,
                        0,
                        0,
                        true,
                    );
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
        let graph_at_pathfinding_step = graph_at_pathfinding_step.clone();
        let pathfinding_steps = Rc::clone(&pathfinding_steps);
        let pathfinding_step_index = pathfinding_step_index.clone();

        Callback::from(move |val| {
            update_graph_at_pathfinding_step(
                &mut graph_at_pathfinding_step.borrow_mut(),
                &pathfinding_steps.borrow().steps,
                val,
                *pathfinding_step_index,
                false,
            );
            pathfinding_step_index.set(val);
        })
    };

    let change_maze_gen_step = {
        let maze_gen_steps = Rc::clone(&maze_gen_steps);
        let maze_gen_step_index = maze_gen_step_index.clone();
        let walls_at_maze_gen_step = Rc::clone(&walls_at_maze_gen_step);

        Callback::from(move |val: usize| {
            *walls_at_maze_gen_step.borrow_mut() = maze_gen_steps.borrow()
                [val.min(maze_gen_steps.borrow().len() - 1)]
            .walls
            .clone();
            maze_gen_step_index.set(val);
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
        let graph = Rc::clone(&graph);
        let walls = Rc::clone(&walls);
        let paused = paused.clone();
        let show_maze_gen_slider = show_maze_gen_slider.clone();

        Callback::from(move |vertex| {
            if vertex != config.start && vertex != config.end {
                if graph.borrow().hash_map.contains_key(&vertex) {
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
                            walls.borrow_mut().insert(vertex);
                            paused.set(true);
                            show_maze_gen_slider.set(false);
                        }
                    }
                } else if config.active_tool == PathTool::Wall {
                    walls.borrow_mut().remove(&vertex);
                    paused.set(true);
                    show_maze_gen_slider.set(false);
                }
            }
        })
    };

    let on_draw_end = {
        let config = config.clone();
        let graph = Rc::clone(&graph);
        let walls = Rc::clone(&walls);
        let pathfinding_steps = Rc::clone(&pathfinding_steps);
        let paused = paused.clone();
        let update_pathfinding_step = update_pathfinding_step.clone();
        let update_or_reset_pathfinding_step_index = update_or_reset_pathfinding_step_index.clone();

        Callback::from(move |_| {
            if config.active_tool == PathTool::Wall {
                let new_graph = generate_graph(
                    config.graph_width,
                    config.graph_height,
                    config.move_diagonally,
                    &walls.borrow(),
                );
                *graph.borrow_mut() = new_graph;

                update_pathfinding_step(&config);
                update_or_reset_pathfinding_step_index(pathfinding_steps.borrow().len(), 0);
            }
            paused.set(false);
        })
    };

    let on_clear_walls = {
        let config = config.clone();
        let graph = Rc::clone(&graph);
        let walls = Rc::clone(&walls);
        let walls_at_maze_gen_step = Rc::clone(&walls_at_maze_gen_step);
        let pathfinding_steps = Rc::clone(&pathfinding_steps);
        let maze_gen_steps = Rc::clone(&maze_gen_steps);
        let update_pathfinding_step = update_pathfinding_step.clone();
        let show_maze_gen_slider = show_maze_gen_slider.clone();

        Callback::from(move |_| {
            walls.borrow_mut().clear();
            walls_at_maze_gen_step.borrow_mut().clear();
            maze_gen_steps.borrow_mut().clear();

            let new_graph = generate_graph(
                config.graph_width,
                config.graph_height,
                config.move_diagonally,
                &walls.borrow(),
            );
            *graph.borrow_mut() = new_graph;

            update_pathfinding_step(&config);
            update_or_reset_pathfinding_step_index(pathfinding_steps.borrow().len(), 0);

            show_maze_gen_slider.set(false);
        })
    };

    let on_generate_maze = {
        let config = config.clone();
        let walls = Rc::clone(&walls);
        let show_maze_gen_slider = show_maze_gen_slider.clone();
        let walls_at_maze_gen_step = walls_at_maze_gen_step.clone();
        let maze_gen_steps = Rc::clone(&maze_gen_steps);

        Callback::from(move |_| {
            // Generate maze
            let res = generate_maze(config.clone());
            *walls.borrow_mut() = res.walls;
            *maze_gen_steps.borrow_mut() = res.steps;

            let new_maze_gen_step_index = update_or_reset_maze_gen_step_index(
                maze_gen_steps.borrow().len(),
                maze_gen_steps.borrow().len(),
            );
            *walls_at_maze_gen_step.borrow_mut() = maze_gen_steps.borrow()
                [new_maze_gen_step_index.min(maze_gen_steps.borrow().len() - 1)]
            .walls
            .clone();

            let new_graph = generate_graph(
                config.graph_width,
                config.graph_height,
                config.move_diagonally,
                &walls.borrow(),
            );
            *graph.borrow_mut() = new_graph;

            update_pathfinding_step(&config);

            show_maze_gen_slider.set(true);
        })
    };

    html! {
        <div class="page" id="Pathfinding">
            <Sidebar>
                <h2>{"Config"}</h2>

                <Collapsible title="General" open={true} class="config-section">
                    <PathfindingControls<EdgeType> config={(*config).clone()} {update_config} />
                </Collapsible>

                <Collapsible title="Walls" open={true} class="config-section">
                    <WallControls {on_clear_walls} {on_generate_maze} />
                </Collapsible>
            </Sidebar>

            <main>
                <div class="visualization">
                    <PathToolbar active_tool={config.active_tool} {on_tool_change} />
                    <PathGrid
                        width={config.graph_width}
                        height={config.graph_height}
                        graph={Rc::clone(&graph_at_pathfinding_step)}
                        walls={
                            if *show_maze_gen_slider {
                                Rc::clone(&walls_at_maze_gen_step)
                            } else {
                                Rc::clone(&walls)
                            }
                        }
                        path={
                            if *pathfinding_step_index >= pathfinding_steps.borrow().len() && path.borrow().len() > 0 {
                                Some(Rc::clone(&path))
                            } else {
                                None
                            }
                        }
                        start={config.start}
                        end={config.end}
                        {on_cell_click}
                        {on_draw_end}
                    />

                    <StepSlider
                        label={format!("Pathfinding steps ({}/{})", *pathfinding_step_index, pathfinding_steps.borrow().len())}
                        active_step_index={*pathfinding_step_index}
                        max={pathfinding_steps.borrow().len()}
                        on_change={change_step}
                        playback_time={config.playback_time}
                        disabled={*paused}
                    />

                    {
                        if *show_maze_gen_slider {
                            html! {
                                <StepSlider
                                    label={format!("Maze generation steps ({}/{})", *maze_gen_step_index, maze_gen_steps.borrow().len())}
                                    active_step_index={*maze_gen_step_index}
                                    max={maze_gen_steps.borrow().len()}
                                    on_change={change_maze_gen_step}
                                    playback_time={config.playback_time}
                                    disabled={*paused}
                                />
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
            </main>
        </div>
    }
}

fn update_graph_at_pathfinding_step<V: Vertex>(
    graph: &mut BTreeMap<V, VertexState>,
    steps: &[PathfindingStep<V>],
    step_i: usize,
    prev_step_i: usize,
    force_from_start: bool,
) {
    // We use a reference to a slice to avoid cloning
    let steps_to_execute: &[PathfindingStep<V>];
    let run_from_start = force_from_start || step_i.abs_diff(0) < step_i.abs_diff(prev_step_i);
    let mut execute_in_reverse = false;

    // Get steps to execute
    if !run_from_start {
        // Execute steps between previous and current step indices
        execute_in_reverse = step_i < prev_step_i;
        if !execute_in_reverse {
            // Going forwards in steps
            steps_to_execute = &steps[prev_step_i..step_i];
        } else {
            // Going backwards in steps
            steps_to_execute = &steps[step_i..prev_step_i];
        }
    } else {
        // Execute all steps from start to current step index
        graph.clear();
        steps_to_execute = &steps[0..step_i];
    }

    execute_steps(graph, steps_to_execute, execute_in_reverse);
}

fn execute_steps<V: Vertex>(
    graph: &mut BTreeMap<V, VertexState>,
    steps: &[PathfindingStep<V>],
    reverse: bool,
) {
    if !reverse {
        for step in steps.iter() {
            for (vertex, state) in step.states.iter() {
                graph.insert(*vertex, *state);
            }
        }
    } else {
        for step in steps.iter().rev() {
            for (vertex, state) in step.states.iter() {
                // Because we are going backwards in steps, we use the "previous" vertex states
                let state = match *state {
                    VertexState::NewVisited => VertexState::NotVisited,
                    VertexState::NotVisited => VertexState::Visited,
                    VertexState::Visited => VertexState::NewVisited,
                };
                graph.insert(*vertex, state);
            }
        }
    }
}

fn generate_maze<E: Edge>(config: UseStateHandle<PathfindingConfig<E>>) -> MazeGenerationResult {
    // Generate actual maze
    let mut maze = recursive_division(
        config.graph_width,
        config.graph_height,
        vec![MazeGenerationStep::new(BTreeSet::new())], // Empty initial step
    );

    // Remove walls around start and end cells to make sure the path isn't blocked.

    maze.walls.remove(&config.start);
    maze.walls.remove(&config.end);

    for neighbor in config.start.adjacent(config.move_diagonally) {
        maze.walls.remove(&neighbor);
    }

    for neighbor in config.end.adjacent(config.move_diagonally) {
        maze.walls.remove(&neighbor);
    }

    // I count removing the walls as a step here even though it's not really a part of the actual maze generation
    maze.steps.push(MazeGenerationStep::new(maze.walls.clone()));

    maze
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
