pub mod audio;

use rand::{thread_rng, Rng};

/// Generate a sorted `Vec<u32>` with a given length.
/// The numbers start from 1, and are in order, e.g. 1, 2, 3...
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

/// Fetch from a given url.
pub async fn fetch(url: String, content_type: &str) -> Result<String, String> {
    let resp = reqwest::get(url.to_string()).await;
    Ok(match resp {
        Ok(resp) => {
            // Make sure the content-type is correct.
            if resp.headers().get("content-type").unwrap() == content_type {
                resp.text().await.unwrap()
            } else {
                "".to_string()
            }
        }
        Err(_) => format!("Could not get {}", url),
    })
}
