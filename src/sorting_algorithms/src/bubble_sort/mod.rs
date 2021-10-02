use crate::SortResult;

/// # Bubble sort
/// Sorts a list of values of type T by comparing ...
///
/// ## Example
/// ```rust
/// use sorting_algorithms::{bubble_sort::sort, SortResult};
///
/// let arr = vec![6, 4, 0, 9, 3, 5, 8, 1];
/// let target = vec![0, 1, 3, 4, 5, 6, 8, 9];
/// let sorted = sort(&arr).arr;
/// assert_eq!(target, sorted);
/// ```
pub fn sort<T: Clone + PartialOrd>(arr: &Vec<T>) -> SortResult<T> {
    let mut sorted: Vec<T> = arr.to_vec();
    let mut steps: Vec<Vec<T>> = vec![sorted.clone()];
    for i in 0..sorted.len() {
        let mut swapped = false;
        for j in 0..sorted.len() - i - 1 {
            if sorted[j] > sorted[j + 1] {
                sorted.swap(j, j + 1);
                swapped = true;
            }
            steps.push(sorted.clone());
        }
        if !swapped {
            break;
        }
    }
    SortResult { arr: sorted, steps }
}
