use rand::Rng;

pub fn gen_i32_vec(len: usize, min: isize, max: isize) -> Vec<i32> {
    let mut vec = vec![];
    if min < max {
        for _ in 0..len {
            vec.push(rand::thread_rng().gen_range(min..=max) as i32);
        }
    }
    vec
}
