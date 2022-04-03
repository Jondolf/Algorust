use crate::SortCommand;

pub fn insertion_sort<T: Clone + Copy + Ord>(
    mut items: Vec<T>,
    mut steps: Vec<Vec<SortCommand<T>>>,
) -> (Vec<T>, Vec<Vec<SortCommand<T>>>) {
    for i in 1..items.len() {
        let mut j = i;
        while j > 0 && items[(j - 1)] > items[j] {
            items.swap(j - 1, j);
            steps.push(vec![SortCommand::Swap(j - 1, j)]);
            j -= 1;
        }
    }
    (items, steps)
}
