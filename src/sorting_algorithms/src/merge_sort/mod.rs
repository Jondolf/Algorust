use crate::SortCommand;

pub fn merge_sort<T: Copy + Clone + Ord>(
    mut items: Vec<T>,
    mut steps: Vec<Vec<SortCommand<T>>>,
) -> (Vec<T>, Vec<Vec<SortCommand<T>>>) {
    items = _merge_sort(items, &mut steps, 0);
    (items, steps)
}

fn _merge_sort<T: Copy + Clone + PartialOrd>(
    mut items: Vec<T>,
    mut steps: &mut Vec<Vec<SortCommand<T>>>,
    start_i: usize,
) -> Vec<T> {
    if items.len() > 1 {
        let middle = items.len() / 2;
        let left_half = _merge_sort(items[0..middle].to_vec(), &mut steps, start_i);
        let right_half = _merge_sort(items[middle..].to_vec(), &mut steps, start_i + middle);
        items = merge(left_half, right_half, &mut steps, start_i);
    }
    items
}

fn merge<T: Copy + Clone + PartialOrd>(
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
