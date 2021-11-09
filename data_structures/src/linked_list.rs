use std::{cell::RefCell, fs::OpenOptions, rc::Rc};

#[derive(Debug)]
struct LinkedList {
    head: Option<Node>,
    tail: Option<Node>,
    length: u32,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn test(&self) {
        println!("this is me {:?}", self);
    }
}

#[derive(Debug)]
struct Node {
    value: String,
    next: Option<Rc<RefCell<Node>>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let node_none = Node {
            value: String::from("this is the end"),
            next: None,
        };
        let node1 = Node {
            value: String::from("test"),
            next: Some(Rc::new(RefCell::new(node_none))),
        };

        let tail_value = (*node1.next.unwrap().borrow_mut()).value.clone();

        assert_eq!(tail_value, "this is the end".to_owned());
    }

    #[test]
    fn test_init() {
        let mut list = LinkedList::new();
        assert_eq!(list.length, 0);
        assert!(list.head.is_none());
        assert!(list.tail.is_none());

        let node_none = Node {
            value: String::from("this is the end"),
            next: None,
        };

        list.test();
        list.tail = Some(node_none);
        list.test();
    }
}
