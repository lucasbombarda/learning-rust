use rayon::prelude::*;
use std::time::Instant;

fn is_prime(n: i32) -> bool {
    if n == 2 || n == 3 {
        return true;
    }

    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    (5..)
        .step_by(6)
        .take_while(|i| i * i <= n)
        .all(|i| n % i != 0 && n % (i + 2) != 0)
}

fn main() {
    let nums = (2..1999999).collect::<Vec<i32>>();

    let start_seq = Instant::now();
    let _: Vec<i32> = nums.iter().filter(|&&x| is_prime(x)).copied().collect();
    let duration_seq = start_seq.elapsed();

    let start_par = Instant::now();
    let _: Vec<i32> = nums.par_iter().filter(|&&x| is_prime(x)).copied().collect();
    let duration_par = start_par.elapsed();

    println!("Sequential:  {:?}", duration_seq);
    println!("Parallel:  {:?}", duration_par);
}
