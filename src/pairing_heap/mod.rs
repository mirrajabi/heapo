//! A minimal implementation of a min-oriented Pairing Heap.
//!
//! Complexity:
//! - insert: O(1)
//! - find-min: O(1)
//! - delete-min / pop: O(log n) amortized

#[derive(Debug, Clone)]
struct Node<T> {
    elem: T,
    children: Vec<Box<Node<T>>>,
}

#[derive(Debug, Clone)]
/// A min-oriented pairing heap.
///
/// Stores elements of type `T` where `T: Ord`.
///
/// # Example
/// ```rust
/// # use heapo::pairing_heap::PairingHeap;
/// let mut h = PairingHeap::new();
/// h.insert(10);
/// h.insert(3);
/// assert_eq!(h.peek(), Some(&3));
/// assert_eq!(h.pop(), Some(3));
/// assert_eq!(h.pop(), Some(10));
/// assert!(h.is_empty());
/// ```
pub struct PairingHeap<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> PairingHeap<T> {
    /// Creates an empty pairing heap.
    ///
    /// # Example
    /// ```rust
    /// # use heapo::pairing_heap::PairingHeap;
    /// let h = PairingHeap::<i32>::new();
    /// assert!(h.is_empty());
    /// ```
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Returns `true` if the heap contains no elements.
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Returns a reference to the current minimum element without removing it.
    pub fn peek(&self) -> Option<&T> {
        self.root.as_ref().map(|n| &n.elem)
    }

    /// Inserts a new element into the heap.
    ///
    /// Amortized O(1).
    pub fn insert(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            children: Vec::new(),
        });
        let old_root = self.root.take();
        self.root = Some(Self::merge_nodes(old_root, Some(new_node)))
    }

    /// Removes (deletes) the minimum element, discarding its value.
    ///
    /// Does nothing if the heap is empty.
    pub fn delete(&mut self) {
        if let Some(node) = self.root.take() {
            self.root = Self::merge_pairs(node.children)
        }
    }

    /// Removes and returns the minimum element, or `None` if empty.
    pub fn pop(&mut self) -> Option<T> {
        match self.root.take() {
            None => None,
            Some(node) => {
                let elem = node.elem;
                self.root = Self::merge_pairs(node.children);
                Some(elem)
            }
        }
    }

    fn merge_nodes(n1: Option<Box<Node<T>>>, n2: Option<Box<Node<T>>>) -> Box<Node<T>> {
        match (n1, n2) {
            (Some(x), None) | (None, Some(x)) => x,
            (Some(mut a), Some(mut b)) => {
                if a.elem < b.elem {
                    a.children.push(b);
                    a
                } else {
                    b.children.push(a);
                    b
                }
            }
            (None, None) => unreachable!(),
        }
    }

    fn merge_pairs(mut heaps: Vec<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        match heaps.len() {
            0 => None,
            1 => Some(heaps.pop().unwrap()),
            _ => {
                let a = heaps.pop();
                let b = heaps.pop();
                let merged = Self::merge_nodes(a, b);

                let rest = Self::merge_pairs(heaps);
                Some(Self::merge_nodes(Some(merged), rest))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::i32;

    use rand::Rng;

    use super::*;

    #[test]
    fn empty_heap_has_is_empty_true() {
        let heap = PairingHeap::<i32>::new();
        assert_eq!(heap.is_empty(), true);
    }

    #[test]
    fn non_empty_heap_has_is_empty_false() {
        let mut heap = PairingHeap::new();
        heap.insert(42);
        assert_eq!(heap.is_empty(), false);
    }

    #[test]
    fn insert_rearranges_the_heap() {
        let mut min = i32::MAX;
        let mut rng = rand::rng();

        let mut heap = PairingHeap::new();
        for _ in 0..1000 {
            let rand_num = rng.random_range(-52335235..832932);
            if rand_num < min {
                min = rand_num;
            }

            heap.insert(rand_num);
            assert_eq!(heap.peek().unwrap(), &min);
        }
    }

    #[test]
    fn delete_min_actually_deletes_min() {
        let mut heap = PairingHeap::new();
        heap.insert(24);
        heap.insert(5);
        heap.insert(14);
        assert_eq!(heap.peek(), Some(&5));
        heap.delete();
        assert_eq!(heap.peek(), Some(&14));
        heap.delete();
        assert_eq!(heap.peek(), Some(&24));
        heap.delete();
        assert_eq!(heap.peek(), None);
        assert_eq!(heap.is_empty(), true);
    }

    #[test]
    fn pop_min_actually_pops_min() {
        let mut heap = PairingHeap::new();
        heap.insert(253);
        heap.insert(1231);
        heap.insert(65);
        assert_eq!(heap.pop(), Some(65));
        assert_eq!(heap.pop(), Some(253));
        assert_eq!(heap.pop(), Some(1231));
        assert_eq!(heap.peek(), None);
        assert_eq!(heap.pop(), None);
        assert_eq!(heap.is_empty(), true);
    }
}
