mod util;

use std::sync::mpsc;
use std::thread;
use util::random::generate_random_number;

fn main() {
    let (tx, rx) = mpsc::channel();

    for _ in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            let number = generate_random_number(1, 10);
            tx.send(number).unwrap();
        });
    }

    let mut numbers = Vec::new();
    for _ in 0..10 {
        let n = rx.recv().unwrap();
        numbers.push(n * n);
    }
    println!("{:?}", numbers);
}
