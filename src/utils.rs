use rand::{thread_rng, Rng};

pub fn gen_u32_vec(len: usize) -> Vec<u32> {
    (1..=len as u32).collect::<Vec<u32>>()
}

/// Shuffle a vector with the Fisher-Yates shuffle, aka Knuth shuffle.
pub fn knuth_shuffle<T>(mut items: Vec<T>) -> Vec<T> {
    let mut curr_i = items.len();
    let mut rand_i: usize;

    while curr_i != 0 {
        rand_i = thread_rng().gen_range(0..curr_i);
        curr_i -= 1;

        items.swap(curr_i, rand_i);
    }

    items
}
