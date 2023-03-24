use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct LinkedQueue<T> {
    front: Link<T>,
    rear: Link<T>,
    len: usize,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
        }))
    }
}

impl<T> LinkedQueue<T> {
    pub fn new() -> Self {
        LinkedQueue {front: None, rear: None, len: 0}
    }

    pub fn enqueue(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.rear.take() {
            Some(old_node) => {
                old_node.borrow_mut().next = Some(new_node.clone());
                self.rear = Some(new_node);
            }
            None => {
                self.front = Some(new_node.clone());
                self.rear = Some(new_node)
            }
        }
        self.len += 1
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.front.take().map(|old_front| {
            match old_front.borrow_mut().next.take() {
                Some(new_front) => {
                    self.front = Some(new_front);
                }
                None => {
                    self.rear.take();
                }
            }
            self.len -= 1;
            Rc::try_unwrap(old_front).ok().unwrap().into_inner().value
        })
    }
}
