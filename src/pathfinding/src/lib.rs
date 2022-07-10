//! This crate contains my implementations of pathfinding utilities and algorithms.
//! I made them for my algorithm visualization website, so they most likely won't be suited for other projects.
pub mod algorithms;
pub mod graph;
pub mod maze_generation;

use core::fmt;
use graph::*;
use num_traits::PrimInt;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::{Debug, Display},
    hash::Hash,
    ops::Add,
};

/// A trait for structs that can calculate the distance from a to b.
pub trait Distance {
    /// Get the distance from a to b.
    fn distance<T: PrimInt>(&self, from: Self) -> T;
}

pub trait Line<T: Distance> {
    fn new(from: T, to: T) -> Self;
    // Approximate the points of a straight line between two points.
    fn get_points(&self) -> Vec<T>;
    fn len(&self) -> usize;
}

pub struct Rect {
    start: Coord,
    end: Coord,
}
impl Rect {
    fn new(start: Coord, end: Coord) -> Self {
        Self { start, end }
    }
    fn get_perimeter(&self) -> Vec<Coord> {
        let mut coords = vec![];
        for x in self.start.x..=self.end.x {
            coords.push(Coord::new(x, self.start.y));
            coords.push(Coord::new(x, self.end.y));
        }
        for y in self.start.y + 1..self.end.y {
            coords.push(Coord::new(self.start.x, y));
            coords.push(Coord::new(self.end.x, y));
        }
        coords
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line2D {
    from: Coord,
    to: Coord,
}
impl Line<Coord> for Line2D {
    fn new(from: Coord, to: Coord) -> Self {
        Self { from, to }
    }
    /// Gets the points of a straight line between two points with the [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm). This version only works for integers.
    fn get_points(&self) -> Vec<Coord> {
        if (self.to.y - self.from.y).abs() < (self.to.x - self.from.x).abs() {
            if self.from.x > self.to.x {
                Self::get_points_low(self.to, self.from)
            } else {
                Self::get_points_low(self.from, self.to)
            }
        } else {
            if self.from.y > self.to.y {
                Self::get_points_high(self.to, self.from)
            } else {
                Self::get_points_high(self.from, self.to)
            }
        }
    }
    fn len(&self) -> usize {
        self.from.distance(self.to)
    }
}
impl Line2D {
    fn get_points_low(from: Coord, to: Coord) -> Vec<Coord> {
        let mut points = Vec::<Coord>::new();
        let (dx, mut dy) = (to.x - from.x, to.y - from.y);
        let mut yi = 1;
        if dy < 0 {
            yi = -1;
            dy = -dy;
        }
        let mut d = 2 * dy - dx;
        let mut y = from.y;

        for x in from.x..=to.x {
            points.push(Coord::new(x, y));
            if d > 0 {
                y += yi;
                d = d + 2 * (dy - dx);
            } else {
                d = d + 2 * dy;
            }
        }
        points
    }
    fn get_points_high(from: Coord, to: Coord) -> Vec<Coord> {
        let mut points = Vec::<Coord>::new();
        let (mut dx, dy) = (to.x - from.x, to.y - from.y);
        let mut xi = 1;
        if dx < 0 {
            xi = -1;
            dx = -dx;
        }
        let mut d = 2 * dx - dy;
        let mut x = from.x;

        for y in from.y..=to.y {
            points.push(Coord::new(x, y));
            if d > 0 {
                x += xi;
                d = d + 2 * (dx - dy);
            } else {
                d = d + 2 * dx;
            }
        }
        points
    }
}

pub trait Vertex: Distance + Copy + Debug + Display + Ord + Hash {}
pub trait Edge: PrimInt {}
impl Edge for u8 {}
impl Edge for u16 {}
impl Edge for u32 {}
impl Edge for u64 {}
impl Edge for usize {}
impl Edge for i8 {}
impl Edge for i16 {}
impl Edge for i32 {}
impl Edge for i64 {}
impl Edge for isize {}

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
impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Vertex for Coord {}
impl Distance for Coord {
    fn distance<T: PrimInt>(&self, from: Self) -> T {
        let x_diff = (from.x - self.x).abs();
        let y_diff = (from.y - self.y).abs();
        T::from(x_diff.max(y_diff)).unwrap()
    }
}

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
pub fn run_pathfinding<V: Vertex, E: Clone>(
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
pub struct PathfindingSteps<V: Vertex> {
    pub steps: Vec<PathfindingStep<V>>,
}
impl<V: Vertex> PathfindingSteps<V> {
    pub fn new(steps: Vec<PathfindingStep<V>>) -> Self {
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
        vertex: V,
        state: VertexState,
    ) -> Option<VertexState> {
        if !self.steps.is_empty() {
            self.steps.last_mut().unwrap().states.insert(vertex, state)
        } else {
            None
        }
    }
    pub fn get_all(self) -> Vec<PathfindingStep<V>> {
        self.steps
    }
    pub fn len(&self) -> usize {
        self.steps.len()
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PathfindingStep<V: Vertex> {
    pub states: BTreeMap<V, VertexState>,
    pub path: BTreeSet<V>,
}
impl<V: Vertex> PathfindingStep<V> {
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

pub fn generate_graph<E: Edge>(
    width: usize,
    height: usize,
    diagonals: bool,
    walls: &BTreeSet<Coord>,
) -> AdjacencyList<Coord, E> {
    let mut graph = AdjacencyList::<Coord, E>::new(BTreeMap::new());

    for y in 0..height as isize {
        for x in 0..width as isize {
            let vertex = Coord::new(x, y);
            if walls.contains(&vertex) {
                continue;
            }
            let mut neighbors = BTreeMap::<Coord, E>::new();
            for coord in vertex.adjacent(diagonals) {
                if walls.contains(&coord) {
                    continue;
                }
                if coord.x >= 0
                    && coord.x < width as isize
                    && coord.y >= 0
                    && coord.y < height as isize
                {
                    let a_diff = (coord.x - vertex.x).abs();
                    let b_diff = (coord.y - vertex.y).abs();
                    if a_diff == 0 || b_diff == 0 {
                        // Horizontal or vertical costs 1
                        neighbors.insert(coord, E::from(1).unwrap());
                    } else {
                        // Diagonal costs 2 to reduce "zigzags"
                        neighbors.insert(coord, E::from(2).unwrap());
                    }
                }
            }
            graph.add_vertex_with_undirected_edges(vertex, neighbors);
        }
    }

    graph
}

#[derive(Clone, Debug, PartialEq)]
pub struct PathfindingResult<V: Vertex, E> {
    pub steps: PathfindingSteps<V>,
    pub path: Vec<V>,
    pub costs: GraphWeightMap<V, E>,
}
impl<V: Vertex, E> PathfindingResult<V, E> {
    pub fn new(steps: PathfindingSteps<V>, path: Vec<V>, costs: GraphWeightMap<V, E>) -> Self {
        Self { steps, path, costs }
    }
}
