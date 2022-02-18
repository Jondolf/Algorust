use std::fmt::Debug;

use crate::{SortCommand, SortResult};

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
/// assert_eq!(sort(items).output, vec![0, 1, 3, 4, 5, 6, 8, 9]);
/// ```
pub fn sort<T: Clone + Copy + Debug + PartialOrd>(items: Vec<T>) -> SortResult<T> {
    let mut sorted_items = items.to_vec();
    let mut steps: Vec<Vec<SortCommand<T>>> = vec![];

    let start = instant::Instant::now();
    sorted_items = merge_sort(sorted_items.clone(), &mut steps, 0);
    let duration = start.elapsed();

    SortResult::new(sorted_items, Some(duration), steps)
}

fn merge_sort<T: Copy + Clone + Debug + PartialOrd>(
    mut items: Vec<T>,
    mut steps: &mut Vec<Vec<SortCommand<T>>>,
    start_i: usize,
) -> Vec<T> {
    if items.len() > 1 {
        let middle = items.len() / 2;
        let left_half = merge_sort(items[0..middle].to_vec(), &mut steps, start_i);
        let right_half = merge_sort(items[middle..].to_vec(), &mut steps, start_i + middle);
        items = merge(left_half, right_half, &mut steps, start_i);
    }
    items
}

fn merge<T: Copy + Clone + Debug + PartialOrd>(
    a: Vec<T>,
    b: Vec<T>,
    steps: &mut Vec<Vec<SortCommand<T>>>,
    start_i: usize,
) -> Vec<T> {
    let size = a.len() + b.len();
    let mut merged: Vec<T> = Vec::with_capacity(size);

    let mut i = 0; // Idx for a
    let mut j = 0; // Idx for b

    // Loop through a and b, adding the smallest values between them to `merged`
    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            merged.push(a[i]);
            steps.push(vec![SortCommand::Set(start_i + merged.len() - 1, a[i])]);
            i += 1;
        } else {
            merged.push(b[j]);
            steps.push(vec![SortCommand::Set(start_i + merged.len() - 1, b[j])]); //1,0
            j += 1;
        }
    }

    // Add all remaining values
    while i < a.len() {
        merged.push(a[i]);
        steps.push(vec![SortCommand::Set(start_i + merged.len() - 1, a[i])]);
        i += 1;
    }
    while j < b.len() {
        merged.push(b[j]);
        steps.push(vec![SortCommand::Set(start_i + merged.len() - 1, b[j])]);
        j += 1;
    }

    merged
}
