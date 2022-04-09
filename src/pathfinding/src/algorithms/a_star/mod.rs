use std::{
    cmp::Ordering,
    collections::{BTreeMap, BinaryHeap, HashMap},
    hash::Hash,
};

use num_traits::PrimInt;

use crate::{graph::AdjacencyList, Distance, PathfindingResult, PathfindingSteps};

pub fn a_star<V: Distance, E: PrimInt>(
    adjacency_list: AdjacencyList<V, E>,
    start: V,
    end: V,
    mut steps: PathfindingSteps<V>,
) -> PathfindingResult<V, E> {
    let start_cell = Cell::new(start, None, E::zero(), start.distance::<E>(end));
    let mut visited = vec![];
    let mut open_set = BinaryHeap::from([VertexWithPriority::new(E::zero(), start_cell.vertex)]);
    let mut cells = HashMap::<V, Cell<V, E>>::new();

    for vertex in adjacency_list.hash_map.keys() {
        cells.insert(
            *vertex,
            Cell::new(*vertex, None, E::max_value(), vertex.distance::<E>(end)),
        );
    }

    cells.insert(start, start_cell);

    while let Some(curr) = open_set.pop() {
        let curr_vertex = curr.vertex.to_owned();
        if curr_vertex == end {
            return PathfindingResult::new(
                steps,
                reconstruct_path::<V, E>(cells, curr_vertex),
                BTreeMap::new(),
            );
        }
        if visited.contains(&curr_vertex) {
            continue;
        }
        visited.push(curr_vertex);
        let curr_g = cells.get(&curr_vertex).unwrap().g.to_owned();

        for (neighbor, cost) in adjacency_list.get_neighbors(&curr_vertex).unwrap().clone() {
            let tentative_g_dist = curr_g.to_owned() + cost.to_owned();
            let neighbor_cell = cells.get_mut(&neighbor).unwrap();

            if tentative_g_dist < neighbor_cell.g.to_owned().into() {
                neighbor_cell.set_parent(Some(curr_vertex));
                neighbor_cell.set_g(tentative_g_dist.into());

                steps.init_step();
                steps.insert_state_to_last_step(neighbor, crate::VertexState::NewVisited);
                open_set.push(VertexWithPriority::new(
                    (tentative_g_dist.to_owned() + neighbor_cell.h.to_owned()).into(),
                    neighbor_cell.vertex,
                ));
            }
        }
    }

    PathfindingResult::new(steps, vec![], BTreeMap::new())
}

fn reconstruct_path<V: Distance, E: PrimInt>(cells: HashMap<V, Cell<V, E>>, mut curr: V) -> Vec<V> {
    let mut path = vec![curr];
    while let Some(parent) = cells.get(&curr).unwrap().parent {
        curr = parent;
        path.push(curr);
    }
    path
}

#[derive(Clone, PartialEq, Eq)]
struct VertexWithPriority<V: Ord + Hash, E: Ord> {
    vertex: V,
    priority: E,
}
impl<V: Ord + Hash, E: Ord> VertexWithPriority<V, E> {
    fn new(priority: E, vertex: V) -> Self {
        Self { priority, vertex }
    }
}
impl<V: Ord + Hash, E: Ord> PartialOrd for VertexWithPriority<V, E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.priority.partial_cmp(&self.priority)
    }
}
impl<V: Ord + Hash, E: Ord> Ord for VertexWithPriority<V, E> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

#[derive(Clone, PartialEq)]
struct Cell<V: Ord + Hash, E: Copy + Ord> {
    vertex: V,
    parent: Option<V>,
    g: E,
    h: E,
}
impl<V: Ord + Hash, E: Copy + Ord> Cell<V, E> {
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
