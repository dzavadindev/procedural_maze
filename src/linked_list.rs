struct Node {
    next: Option<Box<Node>>,
    prev: Option<Box<Node>>,
}

pub struct LinkedList {
    head: Option<Box<Node>>,
    tail: Option<Box<Node>>,
    length: u32,
}

impl LinkedList {
    pub fn new(&self) -> Self {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn push_front(&self) {
        let new_head = Node {
            prev: None,
            next: self.head,
        };
        self.head = Box::new(new_head);
    }
}
