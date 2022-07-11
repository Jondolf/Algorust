use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::{graph::AdjacencyList, Edge, PathfindingResult, PathfindingSteps, Vertex, VertexState};

pub fn dfs<V: Vertex, E: Edge>(
    adjacency_list: AdjacencyList<V, E>,
    start: V,
    end: V,
    mut steps: PathfindingSteps<V>,
) -> PathfindingResult<V, E> {
    let path = _iterative_dfs(adjacency_list, start, end, &mut steps);
    PathfindingResult::new(steps, path, BTreeMap::new())
}

fn _iterative_dfs<V: Vertex, E: Edge>(
    adjacency_list: AdjacencyList<V, E>,
    start: V,
    end: V,
    steps: &mut PathfindingSteps<V>,
) -> Vec<V> {
    let mut stack = vec![start];
    // Map of the path's vertices and their parents
    let mut vertex_parents = HashMap::<V, V>::new();
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

            if let Some(neighbors) = adjacency_list.get_neighbors(&vertex) {
                for (neighbor, _) in neighbors.iter().rev() {
                    if !visited.contains(neighbor) {
                        stack.push(*neighbor);
                        vertex_parents.insert(*neighbor, vertex);
                    }
                }
            }
        }
    }

    vec![]
}

fn get_path<V: Vertex>(vertex_parents: HashMap<V, V>, mut vertex: V) -> Vec<V> {
    let mut path = vec![];

    while vertex_parents.contains_key(&vertex) {
        vertex = *vertex_parents.get(&vertex).unwrap();
        path.push(vertex);
    }
    path.reverse();
    path
}

fn _recursive_dfs<V: Vertex, E: Edge>(
    adjacency_list: AdjacencyList<V, E>,
    (vertex, cost): (V, E),
    mut visited: BTreeMap<V, E>,
    end: V,
    mut steps: PathfindingSteps<V>,
) -> (BTreeMap<V, E>, PathfindingSteps<V>) {
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
