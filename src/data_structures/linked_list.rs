//! Singly linked list implementation

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    #[must_use]
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    /// Insert at front (O(1))
    pub fn push_front(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: self.head.take(),
        }));
        self.head = Some(new_node);
    }

    /// Insert after existing node (O(1))
    pub fn insert_after(&mut self, prev_node: &Rc<RefCell<Node<T>>>, value: T) {
        let mut prev_borrow = prev_node.borrow_mut();
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: prev_borrow.next.take(),
        }));
        prev_borrow.next = Some(new_node);
    }

    /// Append to end (O(n))
    pub fn push_back(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node { value, next: None }));

        if let Some(tail) = self.tail() {
            tail.borrow_mut().next = Some(new_node);
        } else {
            self.head = Some(new_node);
        }
    }

    /// Delete first node (O(1))
    pub fn pop_front(&mut self) {
        self.head = self
            .head
            .take()
            .and_then(|node| node.borrow_mut().next.take());
    }

    /// Find last node
    fn tail(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.head.as_ref().and_then(|node| {
            let mut current = node.clone();
            loop {
                let next = current.borrow().next.clone();
                match next {
                    Some(next_node) => current = next_node,
                    None => break Some(current.clone()),
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertions() {
        let mut list = LinkedList::new();
        list.push_front(2);
        list.push_front(4);
        list.push_back(10);

        // Check head and next node
        let head = list.head.as_ref().unwrap();
        assert_eq!(head.borrow().value, 4);
        assert_eq!(head.borrow().next.as_ref().unwrap().borrow().value, 2);

        // Insert after head
        let head_clone = head.clone();
        list.insert_after(&head_clone, 5);

        // Verify insertion
        let head = list.head.as_ref().unwrap();
        assert_eq!(head.borrow().next.as_ref().unwrap().borrow().value, 5);
    }

    #[test]
    fn test_deletion() {
        let mut list = LinkedList::new();
        list.push_front(2);
        list.push_front(4);
        list.pop_front();
        assert_eq!(list.head.as_ref().unwrap().borrow().value, 2);
    }
}
