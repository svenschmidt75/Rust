use std::collections::VecDeque;
use std::ops::Index;

pub(crate) struct PriorityQueue {
    data: Vec<(i64, u64)>,
}

impl PriorityQueue {
    pub fn new() -> PriorityQueue {
        PriorityQueue { data: vec![] }
    }

    pub fn enqueue(&mut self, priority: i64, element: u64) {
        self.data.push((priority, element));
        self.data.sort_by_key(|(p, _)| *p);
    }

    pub(crate) fn dequeue(&mut self) -> (i64, u64) {
        let (priority, element) = self.data[0];
        self.data.remove(0);
        (priority, element)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::naive_priority_queue::PriorityQueue;

    #[test]
    fn test_pq() {
        // Arrange
        let mut pq = PriorityQueue::new();
        pq.enqueue(7, 1);
        pq.enqueue(19, 2);
        pq.enqueue(2, 3);

        // Act
        // Assert
        let (priority, element) = pq.dequeue();
        assert_eq!(element, 3);

        let (priority, element) = pq.dequeue();
        assert_eq!(element, 1);

        let (priority, element) = pq.dequeue();
        assert_eq!(element, 2);
    }
}
