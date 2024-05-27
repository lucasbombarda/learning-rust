mod util;
use util::random::generate_random_array;

fn softmax(arr: Vec<f64>) -> Vec<f64> {
    let sum: f64 = arr.iter().sum();
    arr.iter().map(|&x| x / sum).collect()
}

fn main() {
    let arr = generate_random_array(4, 0.0, 1.0);
    println!("Input: {:?}", arr);
    println!("Softmax: {:?}", softmax(arr));
}
