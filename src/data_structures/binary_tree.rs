use std::cell::RefCell;
use std::rc::Rc;

/// A node in the binary tree.
#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

/// A binary tree data structure.
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Ord> Default for BinaryTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> BinaryTree<T> {
    /// Create a new empty binary tree.
    #[must_use]
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    /// Insert a value into the binary tree.
    pub fn insert(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            left: None,
            right: None,
        }));

        if let Some(root) = &self.root {
            Self::insert_recursive(Some(root.clone()), &new_node);
        } else {
            self.root = Some(new_node);
        }
    }

    /// Recursively insert a value into the binary tree.
    fn insert_recursive(current: Option<Rc<RefCell<Node<T>>>>, new_node: &Rc<RefCell<Node<T>>>) {
        if let Some(current) = current {
            let mut current_borrow = current.borrow_mut();
            match new_node.borrow().value.cmp(&current_borrow.value) {
                std::cmp::Ordering::Less => {
                    if current_borrow.left.is_none() {
                        current_borrow.left = Some(new_node.clone());
                    } else {
                        Self::insert_recursive(current_borrow.left.clone(), new_node);
                    }
                }
                std::cmp::Ordering::Greater => {
                    if current_borrow.right.is_none() {
                        current_borrow.right = Some(new_node.clone());
                    } else {
                        Self::insert_recursive(current_borrow.right.clone(), new_node);
                    }
                }
                std::cmp::Ordering::Equal => {}
            }
        }
    }

    /// Search for a value in the binary tree.
    pub fn search(&self, value: &T) -> bool {
        Self::search_recursive(self.root.clone(), value)
    }

    /// Recursively search for a value in the binary tree.
    fn search_recursive(current: Option<Rc<RefCell<Node<T>>>>, value: &T) -> bool {
        if let Some(current) = current {
            let current_borrow = current.borrow();
            match value.cmp(&current_borrow.value) {
                std::cmp::Ordering::Equal => true,
                std::cmp::Ordering::Less => {
                    Self::search_recursive(current_borrow.left.clone(), value)
                }
                std::cmp::Ordering::Greater => {
                    Self::search_recursive(current_borrow.right.clone(), value)
                }
            }
        } else {
            false
        }
    }

    /// Perform an in-order traversal of the binary tree.
    #[must_use]
    pub fn in_order_traversal(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut result = Vec::new();
        Self::in_order_traversal_recursive(self.root.clone(), &mut result);
        result
    }

    /// Recursively perform an in-order traversal.
    fn in_order_traversal_recursive(current: Option<Rc<RefCell<Node<T>>>>, result: &mut Vec<T>)
    where
        T: Clone,
    {
        if let Some(current) = current {
            let current_borrow = current.borrow();
            Self::in_order_traversal_recursive(current_borrow.left.clone(), result);
            result.push(current_borrow.value.clone());
            Self::in_order_traversal_recursive(current_borrow.right.clone(), result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut tree = BinaryTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);

        assert!(tree.search(&5));
        assert!(tree.search(&3));
        assert!(tree.search(&7));
        assert!(!tree.search(&10));
    }

    #[test]
    fn test_in_order_traversal() {
        let mut tree = BinaryTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        tree.insert(4);

        assert_eq!(tree.in_order_traversal(), vec![2, 3, 4, 5, 7]);
    }

    #[test]
    fn test_empty_tree() {
        let tree: BinaryTree<i32> = BinaryTree::new();
        assert!(!tree.search(&5));
        assert_eq!(tree.in_order_traversal(), vec![]);
    }
}
