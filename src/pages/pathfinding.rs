use crate::components::{
    algo_desc::AlgoDesc,
    collapsible::Collapsible,
    pathfinding::{toolbar::*, *},
    sidebar::Sidebar,
    step_slider::StepSlider,
};
use pathfinding::{
    generate_graph,
    graph::AdjacencyList,
    maze_generation::{recursive_division, MazeGenerationResult, MazeGenerationStep},
    pathfinding_algorithms, run_pathfinding, Coord, Edge, PathfindingResult, PathfindingStep,
    PathfindingSteps, Vertex, VertexState,
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
            find_path: pathfinding_algorithms::dijkstra::<Coord, E>,
        }
    }
}

pub fn get_pathfinding_algorithms<V: Vertex, E: Edge>(
) -> BTreeMap<&'static str, PathfindingAlgorithm<V, E>> {
    // `BTreeMap` because it keeps the order of the items.
    BTreeMap::from([
        (
            "a*",
            PathfindingAlgorithm::new("A*", pathfinding_algorithms::a_star::<V, E>),
        ),
        (
            "dijkstra",
            PathfindingAlgorithm::new("Dijkstra", pathfinding_algorithms::dijkstra::<V, E>),
        ),
        (
            "dfs",
            PathfindingAlgorithm::new("DFS", pathfinding_algorithms::dfs::<V, E>),
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
    pub playback_time: f32,
}
impl<E: Edge> Default for PathfindingConfig<E> {
    fn default() -> Self {
        Self {
            algorithm: PathfindingAlgorithm::default(),
            graph_width: 25,
            graph_height: 25,
            move_diagonally: false,
            playback_time: 5.0,
        }
    }
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

        use_mut_ref(|| {
            let mut config = PathfindingConfig::default();
            if let Some(algorithm) = get_pathfinding_algorithms().get(algorithm_name.as_str()) {
                config.algorithm = algorithm.to_owned();
            }
            config
        })
    };

    let start = use_state_eq(|| Coord::new(2, 2));
    let end = use_state_eq(|| Coord::new(22, 22));

    let active_tool = use_state_eq(|| PathTool::Wall);

    let path = use_mut_ref(Vec::<Coord>::new);
    let walls = use_mut_ref(BTreeSet::new);

    let graph = use_mut_ref(|| {
        generate_graph::<EdgeType>(
            config.borrow().graph_width,
            config.borrow().graph_height,
            config.borrow().move_diagonally,
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

    let find_path = {
        let config = config.clone();
        let graph = Rc::clone(&graph);

        move |start: Coord, end: Coord| {
            config
                .borrow()
                .algorithm
                .find_path(&graph.borrow(), start, end)
                .0
        }
    };

    let update_pathfinding_step = {
        let steps = Rc::clone(&pathfinding_steps);
        let path = Rc::clone(&path);
        let graph_at_step = graph_at_pathfinding_step.clone();
        let step_i = pathfinding_step_index.clone();

        move |pathfinding_result: PathfindingResult<Coord, EdgeType>| {
            let old_step_count = steps.borrow().len();

            *path.borrow_mut() = pathfinding_result.path;
            *steps.borrow_mut() = pathfinding_result.steps;

            let old_step_i = *step_i;
            let new_step_i = if old_step_i >= old_step_count {
                steps.borrow().len()
            } else {
                0
            };

            update_graph_at_pathfinding_step(
                &mut graph_at_step.borrow_mut(),
                &steps.borrow().steps,
                new_step_i,
                old_step_i.min(steps.borrow().len()),
                true,
            );

            step_i.set(new_step_i);
        }
    };

    // Config has been updated, update grah, path etc.
    let on_update_config = {
        let config = config.clone();
        let (start, end) = (*start, *end);
        let graph = Rc::clone(&graph);
        let walls = Rc::clone(&walls);
        let find_path = find_path.clone();
        let update_pathfinding_step = update_pathfinding_step.clone();

        Callback::from(move |_| {
            let new_graph = generate_graph(
                config.borrow().graph_width,
                config.borrow().graph_height,
                config.borrow().move_diagonally,
                &walls.borrow(),
            );
            *graph.borrow_mut() = new_graph;

            update_pathfinding_step(find_path(start, end));
        })
    };

    let route = use_route::<PathfindingRoute>();

    {
        let config = Rc::clone(&config);
        let (start, end) = (*start, *end);
        let find_path = find_path.clone();
        let update_pathfinding_step = update_pathfinding_step.clone();

        use_effect_with_deps(
            move |route| {
                let algorithm_name = match route.as_ref().unwrap() {
                    PathfindingRoute::PathfindingAlgorithm { algorithm } => algorithm,
                    _ => "dijkstra",
                };
                if let Some(algorithm) = get_pathfinding_algorithms().get(algorithm_name) {
                    config.borrow_mut().algorithm = algorithm.clone();

                    update_pathfinding_step(find_path(start, end));
                }
                || ()
            },
            route,
        );
    }

    use_title(format!("{} - Pathfinding", config.borrow().algorithm.name));

    let on_change_pathfinding_step = {
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

    let on_change_maze_gen_step = {
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
        let active_tool = active_tool.clone();

        Callback::from(move |new_active_tool| {
            active_tool.set(new_active_tool);
        })
    };

    let on_click_cell = {
        let (start, end) = (start.clone(), end.clone());
        let active_tool = *active_tool;
        let graph = Rc::clone(&graph);
        let walls = Rc::clone(&walls);
        let paused = paused.clone();
        let show_maze_gen_slider = show_maze_gen_slider.clone();
        let update_pathfinding_step = update_pathfinding_step.clone();
        let find_path = find_path.clone();

        Callback::from(move |vertex| {
            if vertex != *start && vertex != *end {
                if graph.borrow().hash_map.contains_key(&vertex) {
                    match active_tool {
                        PathTool::Start => {
                            start.set(vertex);
                            update_pathfinding_step(find_path(vertex, *end));
                        }
                        PathTool::End => {
                            end.set(vertex);
                            update_pathfinding_step(find_path(*start, vertex));
                        }
                        PathTool::Wall => {
                            walls.borrow_mut().insert(vertex);
                            paused.set(true);
                            show_maze_gen_slider.set(false);
                        }
                    }
                } else if active_tool == PathTool::Wall {
                    walls.borrow_mut().remove(&vertex);
                    paused.set(true);
                    show_maze_gen_slider.set(false);
                }
            }
        })
    };

    let on_draw_end = {
        let config = config.clone();
        let (start, end) = (*start, *end);
        let active_tool = *active_tool;
        let graph = Rc::clone(&graph);
        let walls = Rc::clone(&walls);
        let paused = paused.clone();
        let update_pathfinding_step = update_pathfinding_step.clone();
        let find_path = find_path.clone();

        Callback::from(move |_| {
            if active_tool == PathTool::Wall {
                let new_graph = generate_graph(
                    config.borrow().graph_width,
                    config.borrow().graph_height,
                    config.borrow().move_diagonally,
                    &walls.borrow(),
                );
                *graph.borrow_mut() = new_graph;

                update_pathfinding_step(find_path(start, end));

                paused.set(false);
            }
        })
    };

    let on_clear_walls = {
        let config = config.clone();
        let (start, end) = (*start, *end);
        let graph = Rc::clone(&graph);
        let walls = Rc::clone(&walls);
        let walls_at_maze_gen_step = Rc::clone(&walls_at_maze_gen_step);
        let maze_gen_steps = Rc::clone(&maze_gen_steps);
        let update_pathfinding_step = update_pathfinding_step.clone();
        let find_path = find_path.clone();
        let show_maze_gen_slider = show_maze_gen_slider.clone();

        Callback::from(move |_| {
            walls.borrow_mut().clear();
            walls_at_maze_gen_step.borrow_mut().clear();
            maze_gen_steps.borrow_mut().clear();

            let new_graph = generate_graph(
                config.borrow().graph_width,
                config.borrow().graph_height,
                config.borrow().move_diagonally,
                &walls.borrow(),
            );
            *graph.borrow_mut() = new_graph;

            update_pathfinding_step(find_path(start, end));

            show_maze_gen_slider.set(false);
        })
    };

    let on_generate_maze = {
        let config = config.clone();
        let (start, end) = (*start, *end);
        let walls = Rc::clone(&walls);
        let show_maze_gen_slider = show_maze_gen_slider.clone();
        let walls_at_maze_gen_step = walls_at_maze_gen_step.clone();
        let maze_gen_steps = Rc::clone(&maze_gen_steps);
        let maze_gen_step_index = maze_gen_step_index.clone();

        Callback::from(move |_| {
            // Generate maze
            let res = generate_maze(&config.borrow(), start, end);
            *walls.borrow_mut() = res.walls;
            *maze_gen_steps.borrow_mut() = res.steps;

            maze_gen_step_index.set(maze_gen_steps.borrow().len());
            *walls_at_maze_gen_step.borrow_mut() = maze_gen_steps.borrow()[maze_gen_steps
                .borrow()
                .len()
                .min(maze_gen_steps.borrow().len() - 1)]
            .walls
            .clone();

            let new_graph = generate_graph(
                config.borrow().graph_width,
                config.borrow().graph_height,
                config.borrow().move_diagonally,
                &walls.borrow(),
            );
            *graph.borrow_mut() = new_graph;

            update_pathfinding_step(find_path(start, end));

            show_maze_gen_slider.set(true);
        })
    };

    html! {
        <div class="page" id="Pathfinding">
            <Sidebar>
                <h2>{"Config"}</h2>

                <Collapsible title="General" open={true} class="config-section">
                    <PathfindingControls<EdgeType> config={Rc::clone(&config)} {on_update_config} />
                </Collapsible>

                <Collapsible title="Walls" open={true} class="config-section">
                    <WallControls {on_clear_walls} {on_generate_maze} />
                </Collapsible>
            </Sidebar>

            <main>
                <div class="visualization">
                    <PathToolbar active_tool={*active_tool} {on_tool_change} />
                    <PathGrid
                        width={config.borrow().graph_width}
                        height={config.borrow().graph_height}
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
                        start={*start}
                        end={*end}
                        {on_click_cell}
                        {on_draw_end}
                    />

                    <StepSlider
                        label={format!("Pathfinding steps ({}/{})", *pathfinding_step_index, pathfinding_steps.borrow().len())}
                        active_step_index={*pathfinding_step_index}
                        max={pathfinding_steps.borrow().len()}
                        on_change={on_change_pathfinding_step}
                        playback_time={config.borrow().playback_time}
                        disabled={*paused}
                    />

                    {
                        if *show_maze_gen_slider {
                            html! {
                                <StepSlider
                                    label={format!("Maze generation steps ({}/{})", *maze_gen_step_index, maze_gen_steps.borrow().len())}
                                    active_step_index={*maze_gen_step_index}
                                    max={maze_gen_steps.borrow().len()}
                                    on_change={on_change_maze_gen_step}
                                    playback_time={config.borrow().playback_time}
                                    disabled={*paused}
                                />
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>

                <AlgoDesc algorithm={config.borrow().algorithm.name.clone()} />
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

fn generate_maze<E: Edge>(
    config: &PathfindingConfig<E>,
    start: Coord,
    end: Coord,
) -> MazeGenerationResult {
    // Generate actual maze
    let mut maze = recursive_division(
        config.graph_width,
        config.graph_height,
        vec![MazeGenerationStep::new(BTreeSet::new())], // Empty initial step
    );

    // Remove walls around start and end cells to make sure the path isn't blocked.

    maze.walls.remove(&start);
    maze.walls.remove(&end);

    for neighbor in start.adjacent(config.move_diagonally) {
        maze.walls.remove(&neighbor);
    }

    for neighbor in end.adjacent(config.move_diagonally) {
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
