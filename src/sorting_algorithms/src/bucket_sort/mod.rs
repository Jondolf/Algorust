use crate::{insertion_sort, SortCommand};

pub fn bucket_sort(
    mut items: Vec<u32>,
    mut steps: Vec<Vec<SortCommand<u32>>>,
) -> (Vec<u32>, Vec<Vec<SortCommand<u32>>>) {
    let size = items.len();
    let k = (size as f32).sqrt().ceil() as usize; // Number of buckets
    let mut buckets: Vec<Vec<u32>> = vec![Vec::with_capacity(size / k); k];
    let max_val = items.clone().into_iter().max().unwrap();

    // Add items to their respective buckets
    for i in 0..size {
        let item = items[i];
        let bucket_i = ((item as f32 - 1.0) / (max_val as f32 / k as f32)).floor() as usize;
        steps.push(vec![SortCommand::Set(
            (bucket_i as f32 * size as f32 / k as f32).ceil() as usize + buckets[bucket_i].len(),
            item,
        )]);
        buckets[bucket_i].push(item);
    }

    // The absolute index of the first element of the current bucket
    let mut bucket_start_i = 0;

    // Sort all buckets individually
    for i in 0..k {
        // Sort the bucket
        let (output, sub_steps) = insertion_sort(buckets[i].clone(), vec![]);

        // Offset the sort steps' indices to match their real positions in `items`
        let sub_steps = add_offset_to_step_indices(&sub_steps, bucket_start_i);
        steps.extend(sub_steps);

        bucket_start_i += buckets[i].len();
        buckets[i] = output;
    }

    // Concatenate the buckets
    items = buckets.into_iter().flatten().collect();

    (items, steps)
}

/// Adds a given offset to the indices in [`SortCommand`]s. Useful when running a sorting algorithm inside another sorting algorithm.
fn add_offset_to_step_indices<T: Clone + Copy + Ord>(
    steps: &Vec<Vec<SortCommand<T>>>,
    offset: usize,
) -> Vec<Vec<SortCommand<T>>> {
    steps
        .into_iter()
        .map(|step| {
            step.into_iter()
                .map(|command| match command {
                    SortCommand::Swap(from, to) => SortCommand::Swap(from + offset, to + offset),
                    SortCommand::Set(i, val) => SortCommand::Set(i + offset, *val),
                })
                .collect::<Vec<SortCommand<T>>>()
        })
        .collect()
}
