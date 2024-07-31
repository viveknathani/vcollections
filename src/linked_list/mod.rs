use std::fmt::Debug;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct VLinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: u64,
}

impl<T> VLinkedList<T> {
    pub fn new() -> VLinkedList<T> {
        VLinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn add_to_start(&mut self, value: T) {
        let node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(node);
        self.size += 1;
    }

    pub fn add_to_end(&mut self, value: T) {
        // create the node
        let node = Box::new(Node { value, next: None });

        // if it is the first node, just assign and return early
        if self.head.is_none() {
            self.head = Some(node);
            self.size += 1;
            return;
        }

        // else, we need to walk till the end of the list
        let mut last_node = &mut self.head;
        while let Some(walker) = last_node {
            if walker.next.is_none() {
                walker.next = Some(node);
                self.size += 1;
                break;
            }
            last_node = &mut walker.next;
        }
    }

    pub fn remove_from_start(&mut self) {
        match self.head.take() {
            None => {}
            Some(node) => {
                self.head = node.next;
                self.size -= 1;
            }
        }
    }

    pub fn remove_from_end(&mut self) {
        if self.head.is_none() {
            return;
        }

        if self.size == 1 {
            self.head.take();
            self.size -= 1;
            return;
        }

        let mut current = &mut self.head;
        for _ in 0..self.size - 2 {
            match current {
                None => {} // we should ideally never reach this case
                Some(ref mut node) => {
                    current = &mut node.next;
                }
            }
        }

        current.as_mut().unwrap().next.take();
        self.size -= 1;
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}

impl<T> VLinkedList<T>
where
    T: Clone,
{
    pub fn to_vector(&self) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();
        let mut current = &self.head;
        while let Some(walker) = current {
            result.push(walker.value.clone());
            current = &walker.next;
        }
        result
    }
}
