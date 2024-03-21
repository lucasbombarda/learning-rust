mod util;

use std::sync::mpsc;
use std::thread;
use util::random::generate_random_number;

fn main() {
    let (tx, rx) = mpsc::channel();
    let half_sec = std::time::Duration::from_millis(500);

    thread::spawn(move || {
        for _ in 0..10 {
            let tx = tx.clone();
            let number = generate_random_number(1, 100);
            tx.send(number).unwrap();
            println!("Sent: {}", number);
            thread::sleep(half_sec);
        }
    });

    for _ in 0..10 {
        let n = rx.recv().unwrap();
        println!("Received and power of two: {} - {}", n, n * n);
    }
}
