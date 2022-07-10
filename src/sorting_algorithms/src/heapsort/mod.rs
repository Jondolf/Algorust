use crate::SortCommand;

pub fn heapsort<T: Clone + Copy + Ord>(items: &mut Vec<T>, steps: &mut Vec<Vec<SortCommand<T>>>) {
    let size = items.len();

    // Build heap
    for i in (0..=size / 2 - 1).rev() {
        heapify(items, size, i, steps);
    }

    // Extract elements from heap
    for i in (0..=size - 1).rev() {
        // Move root to end
        items.swap(0, i);
        steps.push(vec![SortCommand::Swap(0, i)]);

        // Max heapify the reduced heap
        heapify(items, i, 0, steps);
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
