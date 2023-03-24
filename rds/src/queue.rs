#[derive(Debug)]
pub struct Queue<T> {
    data: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { data: Vec::new() }
    }

    pub fn enqueue(&mut self, item: T) {
        self.data.push(item)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        Some(self.data.remove(0))
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
