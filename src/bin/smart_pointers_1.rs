#[derive(Debug)]
enum List {
    Cons(i32, Option<Box<List>>),
}

fn main() {
    let x = 10;
    // Box is a smart pointer that points to heap memory
    let y = Box::new(x);
    println!("y - Box: {}", y);

    let list = List::Cons(
        1,
        Some(Box::new(List::Cons(2, Some(Box::new(List::Cons(3, None)))))),
    );
    println!("List - {:?}", list);
}
