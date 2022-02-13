use std::time::Duration;

pub mod bubble_sort;
pub mod insertion_sort;
pub mod merge_sort;

#[derive(Clone, PartialEq)]
pub struct SortResult<T: Clone + Copy + PartialEq + PartialOrd> {
    pub value: Vec<T>,
    pub duration: Option<Duration>,
    pub steps: Vec<Step<T>>,
}

impl<T: Clone + Copy + PartialEq + PartialOrd> SortResult<T> {
    /// Creates a new `SortResult` from a vector of `Step` structs.
    pub fn new(value: Vec<T>, duration: Option<Duration>, steps: Vec<Step<T>>) -> Self {
        Self {
            value,
            duration,
            steps,
        }
    }
    /// Creates a new `SortResult` from a `Vec<Vec<T>>` instead of a vector of `Step` structs.
    pub fn new_from_values(value: Vec<T>, duration: Option<Duration>, steps: Vec<Vec<T>>) -> Self {
        let mut arr: Vec<Step<T>> = vec![];
        for (i, values) in steps.iter().enumerate() {
            let prev_values: Vec<T> = if i == 0 {
                vec![]
            } else {
                steps[i - 1].to_owned()
            };
            arr.push(Step::new(values.to_vec(), prev_values));
        }
        Self {
            value,
            duration,
            steps: arr,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Step<T: Clone + Copy + PartialEq + PartialOrd> {
    pub values: Vec<T>,
    /// Indices of values that have changed from the previous step
    pub changed_indices: Vec<usize>,
}
impl<T: Clone + Copy + PartialEq + PartialOrd> Step<T> {
    pub fn new(values: Vec<T>, prev_values: Vec<T>) -> Self {
        let mut changed_indices: Vec<usize> = vec![];
        for i in 0..values.len() {
            if i < prev_values.len() && values[i] != prev_values[i] {
                changed_indices.push(i);
            }
        }
        Self {
            values,
            changed_indices,
        }
    }
}
