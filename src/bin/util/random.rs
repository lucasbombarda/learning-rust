use rand::Rng;

pub fn generate_random_array<T>(size: usize, min_number: T, max_number: T) -> Vec<T>
where
    T: std::cmp::PartialOrd + rand::distributions::uniform::SampleUniform + Copy,
{
    let mut rng = rand::thread_rng();
    (0..size)
        .map(|_| rng.gen_range(min_number..=max_number))
        .collect()
}

pub fn generate_random_number(min_number: isize, max_number: isize) -> isize {
    let mut rng = rand::thread_rng();
    rng.gen_range(min_number..=max_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_array() {
        let size = 10;
        let result = generate_random_array(size, 1, 10);
        assert_eq!(result.len(), size);
    }

    #[test]
    fn test_generate_random_number() {
        let result = generate_random_number(1, 10);
        assert!(result >= 1 && result < 10);
    }

    #[test]
    fn test_generate_random_number_with_same_min_max() {
        let result = generate_random_number(1, 1);
        assert_eq!(result, 1);
    }
}
