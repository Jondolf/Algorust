use crate::SortCommand;

pub fn bubble_sort<T: Clone + Copy + Ord>(
    items: &mut Vec<T>,
    steps: &mut Vec<Vec<SortCommand<T>>>,
) {
    for i in 0..items.len() {
        let mut swapped = false;
        for j in 0..items.len() - i - 1 {
            if items[j] > items[j + 1] {
                items.swap(j, j + 1);
                steps.push(vec![SortCommand::Swap(j, j + 1)]);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}
