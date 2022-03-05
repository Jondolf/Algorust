use crate::{SortCommand, SortResult};

/// # Heapsort
///
/// Sorts a list of values of type T with the heapsort algorithm.
///
/// ## Example
///
/// ```rust
/// use sorting_algorithms::{heapsort::sort, SortResult};
///
/// let arr = vec![6, 4, 0, 9, 3, 5, 8, 1];
/// let items = vec![6, 4, 0, 9, 3, 5, 8, 1];
/// assert_eq!(sort(items).output, vec![0, 1, 3, 4, 5, 6, 8, 9]);
/// ```
pub fn sort<T: Clone + Copy + Ord>(mut items: Vec<T>) -> SortResult<T> {
    let mut steps: Vec<Vec<SortCommand<T>>> = vec![];
    let start = instant::Instant::now();

    heap_sort(&mut items, &mut steps);

    let duration = start.elapsed();
    SortResult::new(items, Some(duration), steps)
}

fn heap_sort<T: Clone + Copy + Ord>(
    mut items: &mut Vec<T>,
    mut steps: &mut Vec<Vec<SortCommand<T>>>,
) {
    let size = items.len();

    // Build heap
    for i in (0..=size / 2 - 1).rev() {
        heapify(&mut items, size, i, &mut steps);
    }

    // Extract elements from heap
    for i in (0..=size - 1).rev() {
        // Move root to end
        items.swap(0, i);
        steps.push(vec![SortCommand::Swap(0, i)]);

        // Max heapify the reduced heap
        heapify(&mut items, i, 0, &mut steps);
    }
}

fn heapify<T: Clone + Copy + Ord>(
    mut items: &mut Vec<T>,
    size: usize,
    i: usize,
    mut steps: &mut Vec<Vec<SortCommand<T>>>,
) {
    let mut largest = i; // Init largest as root
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    // See if left is new largest
    if left < size && items[left] > items[largest] {
        largest = left;
    }

    // See if right is new largest
    if right < size && items[right] > items[largest] {
        largest = right;
    }

    // If new largest found, swap previous largest with new largest
    if largest != i {
        items.swap(i, largest);
        steps.push(vec![SortCommand::Swap(i, largest)]);

        // Recursively heapify the sub-trees
        heapify(&mut items, size, largest, &mut steps);
    }
}
