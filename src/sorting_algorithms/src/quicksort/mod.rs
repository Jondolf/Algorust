use crate::{SortCommand, SortResult};

/// # Quicksort
///
/// Sorts a list of values of type T with the quicksort algorithm.
///
/// ## Example
///
/// ```rust
/// use sorting_algorithms::{quicksort::sort, SortResult};
///
/// let arr = vec![6, 4, 0, 9, 3, 5, 8, 1];
/// let items = vec![6, 4, 0, 9, 3, 5, 8, 1];
/// assert_eq!(sort(items).output, vec![0, 1, 3, 4, 5, 6, 8, 9]);
/// ```
pub fn sort<T: Clone + Copy + Ord>(mut items: Vec<T>) -> SortResult<T> {
    let mut steps: Vec<Vec<SortCommand<T>>> = vec![];
    let start = instant::Instant::now();

    let high = items.len() as isize - 1;
    quicksort(&mut items, &mut steps, 0, high);

    let duration = start.elapsed();
    SortResult::new(items, Some(duration), steps)
}

fn quicksort<T: Clone + Copy + Ord>(
    mut items: &mut Vec<T>,
    mut steps: &mut Vec<Vec<SortCommand<T>>>,
    low: isize,
    high: isize,
) {
    if low < high {
        let pivot_i = partition(&mut items, &mut steps, low, high);
        quicksort(&mut items, &mut steps, low, pivot_i - 1);
        quicksort(&mut items, &mut steps, pivot_i + 1, high);
    }
}

fn partition<T: Clone + Copy + Ord>(
    items: &mut Vec<T>,
    steps: &mut Vec<Vec<SortCommand<T>>>,
    low: isize,
    high: isize,
) -> isize {
    let pivot = items[high as usize];
    let mut i = low - 1; // Left index
    let mut j = high; // Right index

    loop {
        i += 1;
        while items[i as usize] < pivot {
            i += 1;
        }
        j -= 1;
        while j >= 0 && items[j as usize] > pivot {
            j -= 1;
        }
        if i < j {
            items.swap(i as usize, j as usize);
            steps.push(vec![SortCommand::Swap(i as usize, j as usize)]);
        } else {
            break;
        }
    }
    items.swap(i as usize, high as usize);
    steps.push(vec![SortCommand::Swap(i as usize, high as usize)]);
    i
}
