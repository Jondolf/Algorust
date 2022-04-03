pub mod bubble_sort;
pub mod bucket_sort;
pub mod heapsort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quicksort;

pub use bubble_sort::bubble_sort;
pub use bucket_sort::bucket_sort;
pub use heapsort::heapsort;
pub use insertion_sort::insertion_sort;
pub use merge_sort::merge_sort;
pub use quicksort::quicksort;

/// Runs a given sorting algorithm on given items.
/// Tracks the duration of running the algorithm and returns a [`SortResult`].
pub fn run_sort<T: Clone + Copy + Ord>(
    items: Vec<T>,
    algorithm: fn(Vec<T>, Vec<Vec<SortCommand<T>>>) -> (Vec<T>, Vec<Vec<SortCommand<T>>>),
) -> SortResult<T> {
    let start = instant::Instant::now();
    let (items, steps) = algorithm(items, vec![]);
    let duration = start.elapsed();
    SortResult::new(items, Some(duration), steps)
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
    pub output: Vec<T>,
    pub duration: Option<instant::Duration>,
    pub steps: Vec<Vec<SortCommand<T>>>,
}
impl<T: Clone + Copy + PartialEq + PartialOrd> SortResult<T> {
    pub fn new(
        value: Vec<T>,
        duration: Option<instant::Duration>,
        steps: Vec<Vec<SortCommand<T>>>,
    ) -> Self {
        Self {
            output: value,
            duration,
            steps,
        }
    }
}
