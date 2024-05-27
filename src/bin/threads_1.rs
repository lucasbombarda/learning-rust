use std::thread;

fn main() {
    let mut handles = Vec::new();

    for x in 0..10 {
        handles.push(thread::spawn(move || {
            println!("Hello, from thread! {}", x);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
