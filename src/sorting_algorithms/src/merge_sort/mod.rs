use crate::SortResult;

/// # Merge sort
///
/// Sorts a list of values of type T with the merge sort algorithm.
///
/// ## Example
///
/// ```rust
/// use sorting_algorithms::{merge_sort::sort, SortResult};
///
/// let items = vec![6, 4, 0, 9, 3, 5, 8, 1];
/// let target = vec![0, 1, 3, 4, 5, 6, 8, 9];
/// let sorted = sort(&items).items;
/// assert_eq!(target, sorted);
/// ```
pub fn sort<T: Clone + Copy + PartialOrd>(items: Vec<T>) -> SortResult<T> {
    let mut items = items.to_vec();
    let mut steps: Vec<Vec<T>> = vec![items.clone()];
    let step_count = (items.len() as f32).log(2.0).ceil() as usize;
    for _ in 0..step_count {
        steps.push(vec![]);
    }
    let start = instant::Instant::now();
    items = merge_sort(items.clone(), &mut steps, 0);
    let duration = start.elapsed();
    SortResult::new(items, Some(duration), steps)
}

fn merge_sort<T: Copy + Clone + PartialOrd>(
    mut items: Vec<T>,
    mut steps: &mut Vec<Vec<T>>,
    depth: usize,
) -> Vec<T> {
    if items.len() > 1 {
        let middle = items.len() / 2;
        let left_half = merge_sort(items[0..middle].to_vec(), &mut steps, depth + 1);
        let right_half = merge_sort(items[middle..].to_vec(), &mut steps, depth + 1);
        items = merge(left_half, right_half);
        let step_index = steps.len() - depth - 1;
        steps[step_index].extend(&items);
    }
    items
}

fn merge<T: Copy + Clone + PartialOrd>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let size = a.len() + b.len();
    let mut merged: Vec<T> = Vec::with_capacity(size);

    let mut i = 0; // Idx for a
    let mut j = 0; // Idx for b

    // Loop through a and b, adding the smallest values between them to `merged`
    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            merged.push(a[i]);
            i += 1;
        } else {
            merged.push(b[j]);
            j += 1;
        }
    }

    // Add all remaining values
    while i < a.len() {
        merged.push(a[i]);
        i += 1;
    }
    while j < b.len() {
        merged.push(b[j]);
        j += 1;
    }

    merged
}
