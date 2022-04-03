use std::{
    collections::{BTreeMap, BTreeSet},
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
) -> PathfindingResult<V> {
    let path = _iterative_dfs(adjacency_list, start, end, &mut steps);
    PathfindingResult::new(steps, path, BTreeMap::new())
}

fn _iterative_dfs<V: Copy + Clone + Debug + Display + Ord + Hash, E: Clone>(
    adjacency_list: AdjacencyList<V, E>,
    start: Vertex<V>,
    end: Vertex<V>,
    steps: &mut PathfindingSteps<V>,
) -> BTreeSet<Vertex<V>> {
    let mut stack = vec![start];
    let mut visited = BTreeSet::new();

    while !stack.is_empty() {
        let vertex = stack.pop().unwrap();

        if vertex == end {
            return visited;
        }

        if !visited.contains(&vertex) {
            visited.insert(vertex);

            steps.init_step();
            steps.insert_state_to_last_step(vertex, VertexState::NewVisited);

            let neighbors = adjacency_list.get_neighbors(&vertex).unwrap().clone();

            for (neighbor, _) in neighbors.iter().rev() {
                if !visited.contains(neighbor) {
                    stack.push(*neighbor);
                }
            }
        }
    }

    visited
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
