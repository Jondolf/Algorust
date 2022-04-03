use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashSet},
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::{
    graph::{AdjacencyList, Vertex},
    PathfindingResult, PathfindingSteps, VertexState,
};

pub fn dijkstra<V: Copy + Clone + Debug + Display + Ord + Hash, E: Clone>(
    adjacency_list: AdjacencyList<V, E>,
    start: Vertex<V>,
    end: Vertex<V>,
    mut steps: PathfindingSteps<V>,
) -> PathfindingResult<V>
where
    isize: From<E>,
{
    let mut distances = BTreeMap::new();
    let mut visited = HashSet::new();
    let mut to_visit = BinaryHeap::new();

    distances.insert(start, 0);
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
                let new_distance = distance + isize::from(cost.to_owned());
                let is_shorter = distances
                    .get(neighbor)
                    .map_or(true, |&current| new_distance < current);

                if *neighbor == end {
                    distances.insert(*neighbor, new_distance);
                    return PathfindingResult::new(
                        steps,
                        distance_map_shortest_path(&adjacency_list, &distances, start, end),
                        distances,
                    );
                }

                if is_shorter {
                    distances.insert(*neighbor, new_distance);
                    to_visit.push(Visit {
                        vertex: *neighbor,
                        distance: new_distance,
                    });
                }
            }
        }
    }

    PathfindingResult::new(
        steps,
        distance_map_shortest_path(&adjacency_list, &distances, start, end),
        distances,
    )
}
#[derive(Debug)]
struct Visit<V> {
    vertex: V,
    distance: isize,
}

impl<V> Ord for Visit<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<V> PartialOrd for Visit<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V> PartialEq for Visit<V> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<V> Eq for Visit<V> {}

/// Finds the shortest path from start to end according to a given distance map.
fn distance_map_shortest_path<V: Copy + Clone + Debug + Display + Ord + Hash, E: Clone>(
    adjacency_list: &AdjacencyList<V, E>,
    distances: &BTreeMap<Vertex<V>, isize>,
    start: Vertex<V>,
    end: Vertex<V>,
) -> BTreeSet<Vertex<V>> {
    let mut shortest_path = BTreeSet::<Vertex<V>>::new();
    let mut curr_vertex = end;
    while curr_vertex != start {
        if let Some(neighbors) = adjacency_list.get_neighbors(&curr_vertex) {
            let unvisited_neighbors = neighbors.iter().filter(|n| !shortest_path.contains(n.0));
            if let Some((new_vertex, _)) = unvisited_neighbors
                .map(|n| {
                    distances
                        .get_key_value(n.0)
                        .unwrap_or((&start, &isize::MAX))
                })
                .min_by(|a, b| a.1.cmp(b.1))
            {
                shortest_path.insert(*new_vertex);
                curr_vertex = *new_vertex;
            }
        }
        if curr_vertex == end {
            break;
        }
    }
    shortest_path.insert(end);
    shortest_path
}
