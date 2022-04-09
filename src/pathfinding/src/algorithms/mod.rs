//! A collection of pathfinding algorithms.
mod a_star;
mod dfs;
mod dijkstra;

pub use a_star::a_star;
pub use dfs::dfs;
pub use dijkstra::dijkstra;
