use std::thread;
use std::time::Duration;

fn main() {
    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(i as u64));
                println!("Thread {i} finished!");

                i * i
            })
        })
        .collect();

    let results: Vec<_> = handles.into_iter().map(|handle| handle.join().unwrap()).collect();
    println!("Results: {:?}", results);
}
