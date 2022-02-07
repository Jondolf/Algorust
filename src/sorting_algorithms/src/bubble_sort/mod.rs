use crate::SortResult;

/// # Bubble sort algorithm
///
/// Sorts a list of values of type T with the bubble sort algorithm.
///
/// ## Example
///
/// ```rust
/// use sorting_algorithms::{bubble_sort::sort, SortResult};
///
/// let arr = vec![6, 4, 0, 9, 3, 5, 8, 1];
/// let target = vec![0, 1, 3, 4, 5, 6, 8, 9];
/// let sorted = sort(&arr).arr;
/// assert_eq!(target, sorted);
/// ```
pub fn sort<T: Clone + Ord>(items: Vec<T>) -> SortResult<T> {
    let mut items = items.clone();
    let mut steps: Vec<Vec<T>> = vec![];
    let start = instant::Instant::now();
    for i in 0..items.len() {
        let mut swapped = false;
        for j in 0..items.len() - i - 1 {
            if items[j] > items[j + 1] {
                items.swap(j, j + 1);
                swapped = true;
            }
        }
        steps.push(items.clone());
        if !swapped {
            break;
        }
    }
    let duration = start.elapsed();
    SortResult::new(items, Some(duration), steps)
}
