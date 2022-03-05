use crate::SortCommand;

pub fn quicksort<T: Clone + Copy + Ord>(
    mut items: Vec<T>,
    mut steps: Vec<Vec<SortCommand<T>>>,
) -> (Vec<T>, Vec<Vec<SortCommand<T>>>) {
    let high = items.len() as isize - 1;
    _quicksort(&mut items, &mut steps, 0, high);
    (items, steps)
}

fn _quicksort<T: Clone + Copy + Ord>(
    mut items: &mut Vec<T>,
    mut steps: &mut Vec<Vec<SortCommand<T>>>,
    low: isize,
    high: isize,
) {
    if low < high {
        let pivot_i = partition(&mut items, &mut steps, low, high);
        _quicksort(&mut items, &mut steps, low, pivot_i - 1);
        _quicksort(&mut items, &mut steps, pivot_i + 1, high);
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
