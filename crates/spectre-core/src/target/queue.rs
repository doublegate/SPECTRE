//! Target priority queue implementation
//!
//! Provides a priority-based queue for ordering targets during scans.
//! Higher priority targets are dequeued first.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use super::EnhancedTarget;

/// Wrapper for priority queue ordering
struct PrioritizedTarget {
    target: EnhancedTarget,
    /// Insertion order for stable sorting of equal priorities
    insertion_order: usize,
}

impl PartialEq for PrioritizedTarget {
    fn eq(&self, other: &Self) -> bool {
        self.target.priority == other.target.priority
            && self.insertion_order == other.insertion_order
    }
}

impl Eq for PrioritizedTarget {}

impl PartialOrd for PrioritizedTarget {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PrioritizedTarget {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher priority first, then earlier insertion order
        self.target
            .priority
            .cmp(&other.target.priority)
            .then_with(|| other.insertion_order.cmp(&self.insertion_order))
    }
}

/// Priority queue for managing scan targets
///
/// Targets are ordered by priority (descending), with ties broken
/// by insertion order (FIFO).
///
/// # Example
///
/// ```
/// use spectre_core::target::{TargetQueue, EnhancedTarget};
///
/// let mut queue = TargetQueue::new();
/// queue.push(EnhancedTarget::from_ip("10.0.0.1".parse().unwrap()));
/// queue.push(EnhancedTarget::from_ip("10.0.0.2".parse().unwrap()).with_priority(10));
///
/// // Higher priority target comes first
/// let next = queue.pop().unwrap();
/// assert_eq!(next.priority, 10);
/// ```
pub struct TargetQueue {
    heap: BinaryHeap<PrioritizedTarget>,
    counter: usize,
    total_added: usize,
    total_completed: usize,
    total_skipped: usize,
    total_errors: usize,
}

impl TargetQueue {
    /// Create a new empty target queue
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            counter: 0,
            total_added: 0,
            total_completed: 0,
            total_skipped: 0,
            total_errors: 0,
        }
    }

    /// Create a queue pre-populated with targets
    pub fn from_targets(targets: Vec<EnhancedTarget>) -> Self {
        let mut queue = Self::new();
        for target in targets {
            queue.push(target);
        }
        queue
    }

    /// Add a target to the queue
    pub fn push(&mut self, target: EnhancedTarget) {
        self.total_added += 1;
        let entry = PrioritizedTarget {
            target,
            insertion_order: self.counter,
        };
        self.counter += 1;
        self.heap.push(entry);
    }

    /// Remove and return the highest-priority target
    pub fn pop(&mut self) -> Option<EnhancedTarget> {
        self.heap.pop().map(|entry| entry.target)
    }

    /// Peek at the highest-priority target without removing it
    pub fn peek(&self) -> Option<&EnhancedTarget> {
        self.heap.peek().map(|entry| &entry.target)
    }

    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Get the number of remaining targets
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Record that a target was completed
    pub fn record_complete(&mut self) {
        self.total_completed += 1;
    }

    /// Record that a target was skipped
    pub fn record_skipped(&mut self) {
        self.total_skipped += 1;
    }

    /// Record that a target errored
    pub fn record_error(&mut self) {
        self.total_errors += 1;
    }

    /// Get total targets added
    pub fn total_added(&self) -> usize {
        self.total_added
    }

    /// Get total targets completed
    pub fn total_completed(&self) -> usize {
        self.total_completed
    }

    /// Get total targets skipped
    pub fn total_skipped(&self) -> usize {
        self.total_skipped
    }

    /// Get total errors
    pub fn total_errors(&self) -> usize {
        self.total_errors
    }

    /// Get progress as a fraction (0.0 - 1.0)
    pub fn progress(&self) -> f64 {
        if self.total_added == 0 {
            return 0.0;
        }
        let processed = self.total_completed + self.total_skipped + self.total_errors;
        processed as f64 / self.total_added as f64
    }
}

impl Default for TargetQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Streaming iterator for large target lists
///
/// Yields targets one at a time from the queue, useful for
/// processing very large target lists without loading all into memory.
pub struct TargetIterator {
    queue: TargetQueue,
}

impl TargetIterator {
    /// Create a new target iterator from a queue
    pub fn new(queue: TargetQueue) -> Self {
        Self { queue }
    }

    /// Get the number of remaining targets
    pub fn remaining(&self) -> usize {
        self.queue.len()
    }
}

impl Iterator for TargetIterator {
    type Item = EnhancedTarget;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.queue.len();
        (len, Some(len))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_new() {
        let queue = TargetQueue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_queue_push_pop() {
        let mut queue = TargetQueue::new();
        queue.push(EnhancedTarget::from_ip("10.0.0.1".parse().unwrap()));
        queue.push(EnhancedTarget::from_ip("10.0.0.2".parse().unwrap()));

        assert_eq!(queue.len(), 2);
        assert!(!queue.is_empty());

        let first = queue.pop().unwrap();
        assert_eq!(first.address.to_string(), "10.0.0.1");

        let second = queue.pop().unwrap();
        assert_eq!(second.address.to_string(), "10.0.0.2");

        assert!(queue.is_empty());
    }

    #[test]
    fn test_queue_priority_ordering() {
        let mut queue = TargetQueue::new();
        queue.push(EnhancedTarget::from_ip("10.0.0.1".parse().unwrap()).with_priority(1));
        queue.push(EnhancedTarget::from_ip("10.0.0.2".parse().unwrap()).with_priority(10));
        queue.push(EnhancedTarget::from_ip("10.0.0.3".parse().unwrap()).with_priority(5));

        let first = queue.pop().unwrap();
        assert_eq!(first.priority, 10);
        assert_eq!(first.address.to_string(), "10.0.0.2");

        let second = queue.pop().unwrap();
        assert_eq!(second.priority, 5);

        let third = queue.pop().unwrap();
        assert_eq!(third.priority, 1);
    }

    #[test]
    fn test_queue_fifo_for_equal_priority() {
        let mut queue = TargetQueue::new();
        queue.push(EnhancedTarget::from_ip("10.0.0.1".parse().unwrap()));
        queue.push(EnhancedTarget::from_ip("10.0.0.2".parse().unwrap()));
        queue.push(EnhancedTarget::from_ip("10.0.0.3".parse().unwrap()));

        assert_eq!(queue.pop().unwrap().address.to_string(), "10.0.0.1");
        assert_eq!(queue.pop().unwrap().address.to_string(), "10.0.0.2");
        assert_eq!(queue.pop().unwrap().address.to_string(), "10.0.0.3");
    }

    #[test]
    fn test_queue_from_targets() {
        let targets = vec![
            EnhancedTarget::from_ip("10.0.0.1".parse().unwrap()),
            EnhancedTarget::from_ip("10.0.0.2".parse().unwrap()),
        ];
        let queue = TargetQueue::from_targets(targets);
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.total_added(), 2);
    }

    #[test]
    fn test_queue_peek() {
        let mut queue = TargetQueue::new();
        assert!(queue.peek().is_none());

        queue.push(EnhancedTarget::from_ip("10.0.0.1".parse().unwrap()));
        assert_eq!(queue.peek().unwrap().address.to_string(), "10.0.0.1");
        assert_eq!(queue.len(), 1); // peek doesn't remove
    }

    #[test]
    fn test_queue_statistics() {
        let mut queue = TargetQueue::new();
        queue.push(EnhancedTarget::from_ip("10.0.0.1".parse().unwrap()));
        queue.push(EnhancedTarget::from_ip("10.0.0.2".parse().unwrap()));
        queue.push(EnhancedTarget::from_ip("10.0.0.3".parse().unwrap()));
        queue.push(EnhancedTarget::from_ip("10.0.0.4".parse().unwrap()));

        assert_eq!(queue.total_added(), 4);
        assert_eq!(queue.progress(), 0.0);

        queue.pop();
        queue.record_complete();
        assert_eq!(queue.total_completed(), 1);

        queue.pop();
        queue.record_skipped();
        assert_eq!(queue.total_skipped(), 1);

        queue.pop();
        queue.record_error();
        assert_eq!(queue.total_errors(), 1);

        assert!((queue.progress() - 0.75).abs() < f64::EPSILON);
    }

    #[test]
    fn test_target_iterator() {
        let mut queue = TargetQueue::new();
        queue.push(EnhancedTarget::from_ip("10.0.0.1".parse().unwrap()));
        queue.push(EnhancedTarget::from_ip("10.0.0.2".parse().unwrap()));
        queue.push(EnhancedTarget::from_ip("10.0.0.3".parse().unwrap()));

        let iter = TargetIterator::new(queue);
        assert_eq!(iter.remaining(), 3);

        let collected: Vec<_> = iter.collect();
        assert_eq!(collected.len(), 3);
    }

    #[test]
    fn test_target_iterator_size_hint() {
        let mut queue = TargetQueue::new();
        queue.push(EnhancedTarget::from_ip("10.0.0.1".parse().unwrap()));
        queue.push(EnhancedTarget::from_ip("10.0.0.2".parse().unwrap()));

        let iter = TargetIterator::new(queue);
        assert_eq!(iter.size_hint(), (2, Some(2)));
    }
}
