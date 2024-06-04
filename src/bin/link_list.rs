#[derive(Debug)]
struct LinkList {
    head: Pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: Pointer,
}

type Pointer = Option<Box<Node>>;

impl LinkList {
    fn new() -> Self {
        Self { head: None }
    }

    fn add(&mut self, element: i32) {
        let previous_head = self.head.take();
        let new_head = Some(Box::new(Node {
            element,
            next: previous_head,
        }));
        self.head = new_head;
    }

    fn remove(&mut self) -> Option<i32> {
        match self.head.take() {
            Some(previous_head) => {
                self.head = previous_head.next;
                Some(previous_head.element)
            }
            None => None,
        }
    }

    fn print(&self) {
        let mut list_traversal = &self.head;

        while list_traversal.is_some() {
            println!("{:?}", list_traversal.as_ref().unwrap());
            list_traversal = &list_traversal.as_ref().unwrap().next;
        }
    }
}

fn main() {
    let mut list = LinkList::new();
    list.add(1);
    list.add(3);
    list.add(2);
    list.add(5);
    list.add(100);

    println!("LIST: {list:?}");
    println!("LAST ELEMENT: {:?}", list.remove().unwrap());
    list.print();
}
