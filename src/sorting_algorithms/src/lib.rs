pub mod bubble_sort;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct SortResult {
    pub arr: Vec<i32>,
    pub steps: Vec<Vec<i32>>,
}
