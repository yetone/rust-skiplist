use std;

#[derive(Debug)]
pub(crate) struct Node<K> {
    forward_: std::vec::Vec<*mut Node<K>>,
    key_: K,
}

impl<K> Node<K> {
    // Node of height 0 means it has only one pointer to the next node, node of
    // height 1 means it keeps a pointer to the next node, and to the next
    // height 1 node, and so on and so forth.
    pub fn new(key: K, height: usize) -> Node<K> {
        Node {
            forward_: vec![std::ptr::null_mut(); height + 1],
            key_: key,
        }
    }

    pub fn height(&self) -> usize {
        self.forward_.len() - 1
    }

    // Returns a reference to the underlying node at the given height
    pub fn next(&self, height: usize) -> Option<&Node<K>> {
        match self.forward_.get(height) {
            None => None,
            Some(ptr) => {
                if unlikely!(ptr.is_null()) {
                    None
                } else {
                    Some(unsafe { &**ptr })
                }
            }
        }
    }

    pub fn mut_next(&mut self, height: usize) -> Option<&mut Node<K>> {
        match self.forward_.get(height) {
            None => None,
            Some(ptr) => {
                if unlikely!(ptr.is_null()) {
                    None
                } else {
                    Some(unsafe { &mut **ptr })
                }
            }
        }
    }

    pub fn link_to(&mut self, height: usize, destination: *mut Node<K>) {
        debug_assert!(height <= self.height());
        unsafe {
            *(self.forward_.get_unchecked_mut(height)) = destination;
        }
    }

    pub fn link_to_next(&mut self, height: usize, node: &Node<K>) {
        debug_assert!(height <= self.height());
        debug_assert!(height <= node.height());
        unsafe {
            *(self.forward_.get_unchecked_mut(height)) = *(node.forward_.get_unchecked(height));
        }
    }

    pub fn key(&self) -> &K {
        &self.key_
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let key = 3;
        let height = 5;
        let node = Node::new(key, height);
        assert_eq!(*node.key(), key);
        assert_eq!(node.height(), height);
    }

    #[test]
    fn next_out_of_bounds() {
        let key = 3;
        let height = 5;
        let mut node = Node::new(key, height);
        assert!(node.next(10).is_none());
        assert!(node.mut_next(10).is_none());
    }

    #[test]
    fn next_empty() {
        let key = 3;
        let height = 5;
        let mut node = Node::new(key, height);
        for height in 0..height {
            assert!(node.next(height).is_none());
            assert!(node.mut_next(height).is_none());
        }
    }

    #[test]
    fn link_singleton() {
        let k_node_key = 4;
        let k_node_height = 5;
        let k_node_set_height = 0;

        let mut node = Node::new(k_node_key, k_node_height);
        let next_node = Box::into_raw(Box::new(Node::new(k_node_key, k_node_height)));
        node.link_to(k_node_set_height, next_node);

        for h in 0..node.height() {
            let next = node.mut_next(h);

            if h == k_node_set_height {
                let next_ptr = next.unwrap();
                assert_eq!(next_ptr.key(), unsafe { (*next_node).key() });
            } else {
                assert!(next.is_none());
        }
        }

        unsafe {
            Box::from_raw(next_node);
        }
    }

    #[test]
    fn link_out_of_range() {

    }

    #[test]
    fn link_next_singleton() {

    }

    #[test]
    fn link_next_out_of_range() {

    }
}
