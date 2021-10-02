pub mod bubble_sort;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct SortResult<T: Clone> {
    pub arr: Vec<T>,
    pub steps: Vec<Vec<T>>,
}
