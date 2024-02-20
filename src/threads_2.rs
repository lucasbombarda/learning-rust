use std::thread;

fn main() {
    println!("Hello from the main thread");

    let handle = thread::spawn(|| {
        println!("Hello from a thread");
    });

    handle.join().unwrap();
}
