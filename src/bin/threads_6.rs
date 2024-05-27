mod util;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    let mut threads = Vec::new();
    for _ in 0..5 {
        let tx = tx.clone();
        threads.push(std::thread::spawn(move || {
            let number = util::random::generate_random_number(1, 100);
            tx.send(number * number).unwrap();
        }));
    }

    for t in threads {
        t.join().unwrap();
    }

    for _ in 0..5 {
        let n = rx.recv().unwrap();
        println!("Received: {}", n);
    }
}
