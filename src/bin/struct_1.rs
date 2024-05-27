mod util;
use util::random::generate_random_number;

#[derive(Debug)]
struct Person<'a> {
    id: u32,
    name: &'a str,
}

impl Person<'_> {
    fn new(name: &str) -> Person {
        let id = generate_random_number(1, 100000) as u32;
        Person { id, name }
    }

    fn show(&self) {
        println!("[{}] {} is a person", self.id, self.name);
    }
}

fn main() {
    let person_1 = Person::new("John Doe");
    person_1.show();

    let person_2 = Person::new("Jane Doe");
    person_2.show();
}
