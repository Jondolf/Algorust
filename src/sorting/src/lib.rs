pub mod sorting_algorithms;
pub use sorting_algorithms::*;

use std::{cell::RefCell, rc::Rc};

/// Runs a given sorting algorithm on given items.
/// Tracks the duration of running the algorithm and returns a [`SortResult`].
pub fn run_sort<T: Clone + Copy + Ord>(
    items: Rc<RefCell<Vec<T>>>,
    algorithm: fn(&mut Vec<T>, &mut Vec<Vec<SortCommand<T>>>),
) -> SortResult<T> {
    let mut steps = vec![];
    let start = instant::Instant::now();
    algorithm(&mut items.borrow_mut(), &mut steps);
    let duration = start.elapsed();
    SortResult::new(Some(duration), steps)
}

/// A command used when sorting a collection in steps.
///
/// ## Example
///
/// You can run steps of `SortCommand`s with `run_sort_steps`.
///
/// ```rust
/// use sorting_algorithms::{SortCommand, run_sort_steps};
///
/// let mut items: Vec<u32> = vec![3, 4, 1];
/// let steps: Vec<Vec<SortCommand<u32>>> = vec![
///     vec![SortCommand::Swap(0, 2)], // Swap 3 and 1
///     vec![SortCommand::Set(1, 2)], // Set 4 to 2
/// ];
///
/// run_sort_steps(&mut items, &steps);
///
/// assert_eq!(items, vec![1, 2, 3]);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum SortCommand<T> {
    /// Swap items in a collection by index: `(from_index, to_index)`
    Swap(usize, usize),
    /// Set the value of an item in a collection by index: `(index, value)`
    Set(usize, T),
}

/// Runs given sorting operations on a vector of type T.
pub fn run_sort_steps<T: Clone + Copy>(items: &mut [T], steps: &[Vec<SortCommand<T>>]) {
    for step in steps {
        for command in step {
            match command {
                SortCommand::Swap(from, to) => items.swap(*from, *to),
                SortCommand::Set(index, value) => items[*index] = *value,
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct SortResult<T: Clone + Copy + PartialEq + PartialOrd> {
    pub duration: Option<instant::Duration>,
    pub steps: Vec<Vec<SortCommand<T>>>,
}
impl<T: Clone + Copy + PartialEq + PartialOrd> SortResult<T> {
    pub fn new(duration: Option<instant::Duration>, steps: Vec<Vec<SortCommand<T>>>) -> Self {
        Self { duration, steps }
    }
}
