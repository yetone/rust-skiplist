use node::Node;
use skiplist::SkipList;


pub struct Iter<'a, K: 'a> {
    current_: Option<&'a Node<K>>,
}

impl<'a, K> Iter<'a, K> {
    pub(crate) fn new(list: &'a SkipList<K>) -> Iter<'a, K> {
        Iter { current_: unsafe { (*list.head_).next(0) } }
    }
}

impl<K> SkipList<K> {
    pub fn iter(&self) -> Iter<K> {
        Iter::new(self)
    }
}

impl<'a, K: 'a> Iterator for Iter<'a, K> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: prefetch, likely
        let key = self.current_.map(|node| node.key());
        self.current_ = self.current_.and_then(|node| node.next(0));
        key
    }
}

// TODO: size hint
// TODO: first, last, binary_search

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
    }

    #[test]
    fn iter_empty() {
    }

    #[test]
    fn iter_single() {
    }

    #[test]
    fn iter_two() {
    }

    #[test]
    fn iter_multiple() {
    }

    #[test]
    fn iter_and_insert_simultaneously() {
    }
}