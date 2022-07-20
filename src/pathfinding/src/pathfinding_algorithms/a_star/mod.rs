use std::{
    cmp::Ordering,
    collections::{BTreeMap, BinaryHeap, HashMap},
    hash::Hash,
};

use crate::{graph::AdjacencyList, *};

pub fn a_star<V: Vertex, E: Edge>(
    adjacency_list: AdjacencyList<V, E>,
    start: V,
    end: V,
    mut steps: PathfindingSteps<V>,
) -> PathfindingResult<V, E> {
    // Discovered vertices that may need to be expanded, beginning with source vertex
    let mut open_set = BinaryHeap::new();
    open_set.push(VertexWithPriority::new(start, start.distance::<E>(end)));

    // Stores the cost values and "parents" of vertices
    let mut cells = HashMap::<V, Cell<V, E>>::new();

    // Initialize "cells" with no parents and infinite distance values from source vertex
    for vertex in adjacency_list.hash_map.keys() {
        cells.insert(
            *vertex,
            Cell::new(*vertex, None, E::max_value(), vertex.distance::<E>(end)),
        );
    }

    // Initialize source vertex costs correctly
    cells.insert(
        start,
        Cell::new(start, None, E::zero(), start.distance::<E>(end)),
    );

    // At each step we get the vertex with the smallest estimated cost from the `open_set`
    while let Some(curr) = open_set.pop() {
        // Return when target is found
        if curr.vertex == end {
            return PathfindingResult::new(
                steps,
                reconstruct_path::<V, E>(cells, curr.vertex),
                BTreeMap::new(),
            );
        }

        if let Some(neighbors) = adjacency_list.get_neighbors(&curr.vertex) {
            let curr_g = cells.get(&curr.vertex).unwrap().g;

            for (neighbor, cost) in neighbors.clone() {
                // Distance from start to neighbor
                let tentative_g_dist = curr_g + cost;
                let neighbor_cell = cells.get_mut(&neighbor).unwrap();

                // Shorter path from start to neighbor found
                if tentative_g_dist < neighbor_cell.g {
                    // Update neighbor's `parent` and `g` values
                    neighbor_cell.set_parent(Some(curr.vertex));
                    neighbor_cell.set_g(tentative_g_dist.into());

                    steps.init_step();
                    steps.insert_state_to_last_step(neighbor, VertexState::NewVisited);

                    // Add neighbor to `open_set`
                    open_set.push(VertexWithPriority::new(
                        neighbor_cell.vertex,
                        (tentative_g_dist.to_owned() + neighbor_cell.h.to_owned()).into(),
                    ));
                }
            }
        }
    }

    // `open_set` is empty and target was never reached
    PathfindingResult::new(steps, vec![], BTreeMap::new())
}

fn reconstruct_path<V: Vertex, E: Edge>(cells: HashMap<V, Cell<V, E>>, mut curr: V) -> Vec<V> {
    let mut path = vec![curr];
    while let Some(parent) = cells.get(&curr).unwrap().parent {
        curr = parent;
        path.push(curr);
    }
    path
}

#[derive(Clone, PartialEq)]
struct VertexWithPriority<V: Ord + Eq + Hash, E: PartialOrd> {
    vertex: V,
    priority: E,
}
impl<V: Ord + Eq + Hash, E: PartialOrd> VertexWithPriority<V, E> {
    fn new(vertex: V, priority: E) -> Self {
        Self { vertex, priority }
    }
}
impl<V: Ord + Eq + Hash, E: PartialOrd> Eq for VertexWithPriority<V, E> {}
impl<V: Ord + Eq + Hash, E: PartialOrd> PartialOrd for VertexWithPriority<V, E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.priority.partial_cmp(&self.priority)
    }
}
impl<V: Ord + Eq + Hash, E: PartialOrd> Ord for VertexWithPriority<V, E> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.partial_cmp(&self.priority).unwrap()
    }
}

#[derive(Clone, PartialEq)]
struct Cell<V: Ord + Hash, E: Copy + PartialOrd> {
    vertex: V,
    parent: Option<V>,
    /// `g` is the cost of the cheapest path from the source vertex to this vertex.
    g: E,
    /// `h` is the result of the heuristic function, i.e. the estimated cost of the cheapest path from this vertex to the target vertex.
    h: E,
}
impl<V: Ord + Hash, E: Copy + PartialOrd> Cell<V, E> {
    fn new(vertex: V, parent: Option<V>, g: E, h: E) -> Self {
        Self {
            vertex,
            parent,
            g,
            h,
        }
    }
    fn set_parent(&mut self, parent: Option<V>) {
        self.parent = parent;
    }
    fn set_g(&mut self, g: E) {
        self.g = g;
    }
}
