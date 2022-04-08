//! This crate contains my implementations of pathfinding utilities and algorithms.
//! I made them for my algorithm visualization website, so they most likely won't be suited for other projects.
pub mod algorithms;
pub mod graph;

use core::fmt;
use graph::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::{Debug, Display},
    hash::Hash,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VertexState {
    NotVisited,
    NewVisited,
    Visited,
}

pub type PathfindingFunc<V, E> =
    fn(AdjacencyList<V, E>, V, V, PathfindingSteps<V>) -> PathfindingResult<V, E>;

pub type GraphWeightMap<V, E> = BTreeMap<V, E>;

/// Tracks the duration of running the algorithm and returns a [`PathfindingResult`].
pub fn run_pathfinding<V: Copy + Debug + Display + Ord + Hash, E: Clone>(
    graph: &AdjacencyList<V, E>,
    start: V,
    end: V,
    algorithm: PathfindingFunc<V, E>,
) -> (PathfindingResult<V, E>, instant::Duration) {
    let start_time = instant::Instant::now();
    let res = algorithm(graph.clone(), start, end, PathfindingSteps::new(vec![]));
    let duration = start_time.elapsed();
    (res, duration)
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PathfindingSteps<T: Copy + Clone + Debug + Ord + Hash> {
    steps: Vec<PathfindingStep<T>>,
}
impl<T: Copy + Clone + Debug + Ord + Hash> PathfindingSteps<T> {
    pub fn new(steps: Vec<PathfindingStep<T>>) -> Self {
        Self { steps }
    }
    pub fn init_step(&mut self) {
        // Remove the old visited vertices from this step, and turn previously new visited to old visited.
        if !self.steps.is_empty() {
            let mut last_step = self.steps.last().unwrap().to_owned();
            self.steps.push(PathfindingStep::new(
                last_step
                    .remove_old_visited()
                    .new_to_old_visited()
                    .states
                    .to_owned(),
                BTreeSet::new(),
            ));
        } else {
            self.steps
                .push(PathfindingStep::new(BTreeMap::new(), BTreeSet::new()));
        }
    }
    pub fn insert_state_to_last_step(
        &mut self,
        vertex: T,
        state: VertexState,
    ) -> Option<VertexState> {
        if !self.steps.is_empty() {
            self.steps.last_mut().unwrap().states.insert(vertex, state)
        } else {
            None
        }
    }
    pub fn get_all(self) -> Vec<PathfindingStep<T>> {
        self.steps
    }
    pub fn len(&self) -> usize {
        self.steps.len()
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PathfindingStep<V: Copy + Clone + Debug + Ord + Hash> {
    pub states: BTreeMap<V, VertexState>,
    pub path: BTreeSet<V>,
}
impl<V: Copy + Clone + Debug + Ord + Hash> PathfindingStep<V> {
    pub fn new(states: BTreeMap<V, VertexState>, path: BTreeSet<V>) -> Self {
        Self { states, path }
    }
    pub fn new_to_old_visited(&mut self) -> &mut Self {
        for state in self.states.values_mut() {
            if *state == VertexState::NewVisited {
                *state = VertexState::Visited;
            }
        }
        self
    }
    pub fn remove_old_visited(&mut self) -> &mut Self {
        self.states
            .retain(|_, state| *state != VertexState::Visited);
        self
    }
    pub fn get(&self, vertex: V) -> Option<&VertexState> {
        self.states.get(&vertex)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}
impl Coord {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub fn adjacent(self, diagonals: bool) -> Vec<Self> {
        let Self { x, y } = self;
        if diagonals {
            vec![
                Coord::new(x - 1, y - 1),
                Coord::new(x, y - 1),
                Coord::new(x + 1, y - 1),
                Coord::new(x - 1, y),
                Coord::new(x + 1, y),
                Coord::new(x - 1, y + 1),
                Coord::new(x, y + 1),
                Coord::new(x + 1, y + 1),
            ]
        } else {
            vec![
                Coord::new(x, y - 1),
                Coord::new(x, y + 1),
                Coord::new(x - 1, y),
                Coord::new(x + 1, y),
            ]
        }
    }
}
impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

pub fn generate_graph(
    width: usize,
    height: usize,
    diagonals: bool,
    walls: BTreeSet<Coord>,
) -> AdjacencyList<Coord, isize> {
    let mut graph = AdjacencyList::<Coord, isize>::default();

    for y in 0..height as isize {
        for x in 0..width as isize {
            let vertex = Coord::new(x, y);
            if walls.contains(&vertex) {
                continue;
            }
            let vertex_cost = vertex.x + vertex.y;
            let mut neighbors = BTreeMap::<Coord, isize>::new();
            for coord in vertex.adjacent(diagonals) {
                if walls.contains(&coord) {
                    continue;
                }
                if coord.x >= 0
                    && coord.x < width as isize
                    && coord.y >= 0
                    && coord.y < height as isize
                {
                    neighbors.insert(coord, vertex_cost + coord.x + coord.y);
                }
            }
            graph.add_vertex_with_undirected_edges(vertex, neighbors);
        }
    }

    graph
}

#[derive(Clone, Debug, PartialEq)]
pub struct PathfindingResult<V: Clone + Copy + Debug + Ord + Hash, E> {
    pub steps: PathfindingSteps<V>,
    pub path: Vec<V>,
    pub costs: GraphWeightMap<V, E>,
}
impl<V: Clone + Copy + Debug + Ord + Hash, E> PathfindingResult<V, E> {
    pub fn new(steps: PathfindingSteps<V>, path: Vec<V>, costs: GraphWeightMap<V, E>) -> Self {
        Self { steps, path, costs }
    }
}
