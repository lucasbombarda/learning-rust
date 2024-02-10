use rand::Rng;

pub fn generate_random_array(
    size: usize,
    min_number: isize,
    max_number: isize) -> Vec<isize> {

    let mut rng = rand::thread_rng();
    (0..size)
        .map(|_| rng.gen_range(min_number..max_number))
        .collect()
}
