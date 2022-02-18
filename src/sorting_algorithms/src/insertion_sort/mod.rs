use crate::{SortCommand, SortResult};

/// # Insertion sort
///
/// Sorts a list of values of type T with the insertion sort algorithm.
///
/// ## Example
///
/// ```rust
/// use sorting_algorithms::{insertion_sort::sort, SortResult};
///
/// let items = vec![6, 4, 0, 9, 3, 5, 8, 1];
/// assert_eq!(sort(items).output, vec![0, 1, 3, 4, 5, 6, 8, 9]);
/// ```
pub fn sort<T: Clone + Copy + PartialOrd>(items: Vec<T>) -> SortResult<T> {
    let mut items = items.clone();
    let mut steps: Vec<Vec<SortCommand<T>>> = vec![];
    let start = instant::Instant::now();
    for i in 1..items.len() {
        let mut j = i;
        while j > 0 && items[(j - 1)] > items[j] {
            items.swap(j - 1, j);
            steps.push(vec![SortCommand::Swap(j - 1, j)]);
            j -= 1;
        }
    }
    let duration = start.elapsed();
    SortResult::new(items, Some(duration), steps)
}
