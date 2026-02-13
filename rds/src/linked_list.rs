
#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            next: None,
        }
    }

    pub fn append(&mut self, next: Link<T>) {
        self.next = next;
    }

    pub fn append_next(&mut self, value: T) {
        match self.next.as_mut() {
            Some(next) => {
                next.append_next(value);
            }
            None => {
                self.append(Some(Box::new(Node::new(value))));
            }
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push(&mut self, value: T) {
        match self.head.as_mut() {
            Some(current) => {
                current.append_next(value);
            }
            None => {
                self.insert(value);
            }
        }
    }

    pub fn insert(&mut self, value: T) {
        let mut new_node = Node::new(value);
        new_node.append(self.head.take());
        self.head = Some(Box::new(new_node));
    }

    // pub fn insert_at(&mut self, value: T, at: u32) {
    // }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|mut old_head| {
            match old_head.next.take() {
                Some(new_head) => {
                    self.head = Some(new_head);
                }
                None => {
                    self.head.take();
                }
            }
            old_head.value
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}