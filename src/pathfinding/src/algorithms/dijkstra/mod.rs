use std::{
    cmp::Ordering,
    collections::{BTreeMap, BinaryHeap, HashSet},
    fmt::Debug,
};

use crate::{graph::AdjacencyList, Edge, PathfindingResult, PathfindingSteps, Vertex, VertexState};

pub fn dijkstra<V: Vertex, E: Edge>(
    adjacency_list: AdjacencyList<V, E>,
    start: V,
    end: V,
    mut steps: PathfindingSteps<V>,
) -> PathfindingResult<V, E> {
    let mut distances = BTreeMap::<V, E>::new();
    let mut visited = HashSet::new();
    let mut to_visit = BinaryHeap::new();

    distances.insert(start, E::zero());
    to_visit.push(Visit {
        vertex: start,
        distance: 0,
    });

    while let Some(Visit { vertex, distance }) = to_visit.pop() {
        steps.init_step();
        steps.insert_state_to_last_step(vertex, VertexState::NewVisited);

        if !visited.insert(vertex) {
            continue;
        }

        if let Some(neighbors) = adjacency_list.get_neighbors(&vertex) {
            for (neighbor, cost) in neighbors {
                let new_distance = E::from(distance).unwrap() + cost.to_owned();
                let is_shorter = distances
                    .get(neighbor)
                    .map_or(true, |current| new_distance < current.to_owned());

                if *neighbor == end {
                    distances.insert(*neighbor, new_distance);
                    return PathfindingResult::new(
                        steps,
                        distance_map_shortest_path(&adjacency_list, &distances, start, end),
                        distances,
                    );
                }

                if is_shorter {
                    distances.insert(*neighbor, new_distance.to_owned().into());
                    to_visit.push(Visit {
                        vertex: *neighbor,
                        distance: new_distance.to_isize().unwrap(),
                    });
                }
            }
        }
    }

    PathfindingResult::new(steps, vec![], distances)
}
#[derive(Debug)]
struct Visit<V, E: Ord> {
    vertex: V,
    distance: E,
}

impl<V, E: Ord> Ord for Visit<V, E> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<V, E: Ord> PartialOrd for Visit<V, E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V, E: Ord> PartialEq for Visit<V, E> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<V, E: Ord> Eq for Visit<V, E> {}

/// Finds the shortest path from start to end according to a given distance map.
fn distance_map_shortest_path<V: Vertex, E: Edge>(
    adjacency_list: &AdjacencyList<V, E>,
    distances: &BTreeMap<V, E>,
    start: V,
    end: V,
) -> Vec<V> {
    let mut shortest_path = vec![end];
    let mut curr_vertex = end;
    // If neighbor is not found in distance map
    let default_val = (&start, &E::max_value());

    while curr_vertex != start {
        if let Some(neighbors) = adjacency_list.get_neighbors(&curr_vertex) {
            let unvisited_neighbors = neighbors.iter().filter(|n| !shortest_path.contains(n.0));
            let closest_neighbor = unvisited_neighbors
                .map(|n| distances.get_key_value(n.0).unwrap_or(default_val))
                .min_by(|a, b| a.1.cmp(b.1));

            if let Some((new_vertex, _)) = closest_neighbor {
                shortest_path.push(*new_vertex);
                curr_vertex = *new_vertex;
            }
        }
        if curr_vertex == end {
            break;
        }
    }
    shortest_path.reverse();
    shortest_path
}
