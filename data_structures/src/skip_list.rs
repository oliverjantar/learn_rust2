use std::cell::{Ref, RefCell};
use std::rc::Rc;

#[derive(Clone, Debug)]
struct Node {
    pub next: Vec<Option<Rc<RefCell<Node>>>>,
    pub offset: u64,
    pub command: String,
}
impl Node {
    pub fn new(
        next: Vec<Option<Rc<RefCell<Node>>>>,
        offset: u64,
        command: String,
    ) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            next,
            offset,
            command,
        }))
    }
}

#[derive(Clone, Debug)]
struct SkipList {
    head: Option<Rc<RefCell<Node>>>,
    tails: Vec<Option<Rc<RefCell<Node>>>>,
    max_level: usize,
    pub length: u64,
}

impl SkipList {
    pub fn append(&mut self, offset: u64, value: String) {
        let level = 1 + if self.head.is_none() {
            self.max_level // use the max level for the first node
        } else {
            self.get_level() // determine the level by coin flip
        };

        let new = Node::new(vec![None; level], offset, value);

        //update the tails for each level

        for i in 0..level {
            if let Some(old) = self.tails[i].take() {
                let next = &mut old.borrow_mut().next;
                next[i] = Some(new.clone());
            }
            self.tails[i] = Some(new.clone());
        }

        //this is the first node in the list
        if self.head.is_none() {
            self.head = Some(new.clone());
        }
        self.length += 1;
    }

    fn get_level(&self) -> usize {
        let mut n = 0;
        //p(true) = 0.5
        while rand::random::<bool>() && n < self.max_level {
            n += 1;
        }
        n
    }

    pub fn new(max_level: usize) -> SkipList {
        SkipList {
            head: None,
            tails: vec![None; max_level + 1],
            max_level,
            length: 0,
        }
    }

    pub fn find(&self, offset: u64) -> Option<String> {
        match self.head {
            Some(ref head) => {
                let mut start_level = self.max_level;
                let node = head.clone();

                let mut result = None;
                loop {
                    if node.borrow().next[start_level].is_some() {
                        break;
                    }
                    start_level -= 1;
                }
                let mut n = node;
                for level in (0..=start_level).rev() {
                    loop {
                        let next = n.clone();
                        match next.borrow().next[level] {
                            Some(ref next) if next.borrow().offset <= offset => n = next.clone(),
                            _ => break,
                        };
                    }
                    if n.borrow().offset == offset {
                        result = Some(n.borrow().command.clone());
                        break;
                    }
                }
                result
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test1() {
        let mut list = SkipList::new(6);

        list.append(1, String::from("test1"));
        list.append(2, String::from("test2"));
        list.append(3, String::from("test3"));
        list.append(4, String::from("test4"));
        list.append(5, String::from("test5"));
        list.append(6, String::from("test6"));

        let x = list.find(3);

        assert_eq!(x, Some(String::from("test3")));

        //assert_eq!(list.head,Some(Rc::new(RefCell::new(

        //  println!("{:?}", list.tails);
    }
}
