use std::time::Duration;

pub mod bubble_sort;
pub mod insertion_sort;
pub mod merge_sort;

#[derive(Clone, PartialEq)]
pub struct SortResult<T> {
    pub value: Vec<T>,
    pub duration: Option<Duration>,
    pub steps: Vec<Vec<T>>,
}

impl<T> SortResult<T> {
    pub fn new(value: Vec<T>, duration: Option<Duration>, steps: Vec<Vec<T>>) -> Self {
        SortResult {
            value,
            duration,
            steps,
        }
    }
}
