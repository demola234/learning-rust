pub struct Queue<T> {
    data: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { data: Vec::new() }
    }

    pub fn enqueue(&mut self, data: T) {
        self.data.push(data);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        } else {
            return Some(self.data.remove(0));
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn remove(&mut self) -> Option<T> {
        return if self.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_operations() {
        let mut queue = Queue::new();

        assert!(queue.is_empty());

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.size(), 3);

        assert_eq!(queue.peek(), Some(&1));

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.peek(), Some(&2));

        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert!(queue.is_empty());
        assert_eq!(queue.size(), 0);    

        queue.remove();
    }
}