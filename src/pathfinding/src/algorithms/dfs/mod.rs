use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::{
    graph::{AdjacencyList, Vertex},
    PathfindingResult, PathfindingSteps, VertexState,
};

pub fn dfs<V: Copy + Clone + Debug + Display + Ord + Hash, E: Clone>(
    adjacency_list: AdjacencyList<V, E>,
    start: Vertex<V>,
    end: Vertex<V>,
    mut steps: PathfindingSteps<V>,
) -> PathfindingResult<V, E> {
    let path = _iterative_dfs(adjacency_list, start, end, &mut steps);
    PathfindingResult::new(steps, path, BTreeMap::new())
}

fn _iterative_dfs<V: Copy + Clone + Debug + Display + Ord + Hash, E: Clone>(
    adjacency_list: AdjacencyList<V, E>,
    start: Vertex<V>,
    end: Vertex<V>,
    steps: &mut PathfindingSteps<V>,
) -> Vec<Vertex<V>> {
    let mut stack = vec![start];
    // Map of the path's vertices and their parents
    let mut vertex_parents = HashMap::<Vertex<V>, Vertex<V>>::new();
    let mut visited = BTreeSet::new();

    while !stack.is_empty() {
        let vertex = stack.pop().unwrap();

        if vertex == end {
            let mut path = get_path(vertex_parents, vertex);
            path.insert(0, start);
            path.push(end);
            return path;
        }

        if !visited.contains(&vertex) {
            visited.insert(vertex);

            steps.init_step();
            steps.insert_state_to_last_step(vertex, VertexState::NewVisited);

            let neighbors = adjacency_list.get_neighbors(&vertex).unwrap().clone();

            for (neighbor, _) in neighbors.iter().rev() {
                if !visited.contains(neighbor) {
                    stack.push(*neighbor);
                    vertex_parents.insert(*neighbor, vertex);
                }
            }
        }
    }

    vec![]
}

fn get_path<V: Copy + Clone + Debug + Display + Ord + Hash>(
    vertex_parents: HashMap<Vertex<V>, Vertex<V>>,
    mut vertex: Vertex<V>,
) -> Vec<Vertex<V>> {
    let mut path = vec![];

    while vertex_parents.contains_key(&vertex) {
        vertex = *vertex_parents.get(&vertex).unwrap();
        path.push(vertex);
    }
    path.reverse();
    path
}

fn _recursive_dfs<V: Copy + Clone + Debug + Display + Ord + Hash, E: Clone>(
    adjacency_list: AdjacencyList<V, E>,
    (vertex, cost): (Vertex<V>, E),
    mut visited: BTreeMap<Vertex<V>, E>,
    end: Vertex<V>,
    mut steps: PathfindingSteps<V>,
) -> (BTreeMap<Vertex<V>, E>, PathfindingSteps<V>) {
    visited.insert(vertex, cost);

    if vertex == end {
        return (visited, steps);
    }

    steps.init_step();

    let neighbors = adjacency_list.get_neighbors(&vertex).unwrap().clone();

    for (neighbor, weight) in neighbors.into_iter() {
        if !visited.contains_key(&neighbor) {
            steps.insert_state_to_last_step(neighbor, VertexState::NewVisited);

            return _recursive_dfs(
                adjacency_list,
                (neighbor, weight.clone()),
                visited,
                end,
                steps,
            );
        }
    }

    (visited, steps)
}
