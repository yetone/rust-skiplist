use node::Node;

use std;
use std::mem;
use std::fmt::{Display};

extern crate rand;
use self::rand::{random, Open01};

#[derive(Debug)]
pub struct SkipList<K> {
    // TODO(jbayardo): pub(crate) syntax when unstable is removed
    #[doc(hidden)]
    pub head_: *mut Node<K>,

    length_: usize,
    upgrade_probability_: f64,
    max_height_: usize,
    height_: usize,
}

impl<K> SkipList<K> {
    // TODO(jbayardo): find some way to remove placeholder
    #[inline(always)]
    pub fn new(upgrade_probability: f64, max_height: usize, placeholder: K)
      -> SkipList<K> {
        // This assertion is here because using Zero Sized Types requires
        // special handling which hasn't been implemented yet.
        assert!(mem::size_of::<K>() != 0, "We're not ready to handle ZSTs");

        assert!(upgrade_probability > 0.0);
        assert!(upgrade_probability < 1.0);
        assert!(max_height > 0);

        SkipList {
            head_: Box::into_raw(Box::new(Node::new(placeholder, max_height))),
            length_: 0,
            upgrade_probability_: upgrade_probability,
            max_height_: max_height,
            height_: 0,
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.length_
    }

    #[inline(always)]
    pub fn upgrade_probability(&self) -> f64 {
        self.upgrade_probability_
    }

    #[inline(always)]
    pub fn max_height(&self) -> usize {
        self.max_height_
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.length_ == 0
    }

    pub fn clear(&mut self) {

    }

    // Simulates a random variate with geometric distribution. The idea is that
    // we are modelling number of successes until first failure, where success
    // probability
    fn random_height(&self) -> usize {
        let mut h = 0;

        while h < self.max_height_ {
            let Open01(throw) = random::<Open01<f64>>();
            if throw >= self.upgrade_probability_ {
                return h;
            }

            h += 1;
        }

        h
    }
}

impl<K: Ord> SkipList<K> {
    fn find_lower_bound(&self, key: &K) -> &mut Node<K> {
        unsafe {
            let mut current = self.head_;
            for height in (0..self.height_).rev() {
                while (*current).has_next(height) &&
                      (*current).next(height).key() < key {
                    current = (*current).mut_ptr_next(height);
                }
            }

            &mut *current
        }
    }

    fn find_lower_bound_with_updates(&self, key: &K)
      -> (&mut Node<K>, Vec<&mut Node<K>>) {
        unsafe {
            let mut updates = Vec::with_capacity(self.max_height_);
            // Initialization for the 'updates' vector starts from the back and
            // moves into the front. We set the length of the uninitialized
            // vector to the actual value we are going to use, so that we can do
            // this initialization efficiently
            updates.set_len(self.max_height_);
            for height in self.height_..self.max_height_ {
                updates[height] = &mut *self.head_;
            }

            let mut current = self.head_;
            for height in (0..self.height_).rev() {
                while (*current).has_next(height) &&
                      (*current).next(height).key() < key {
                    current = (*current).mut_ptr_next(height);
                }

                updates[height] = &mut *current;
            }

            (&mut *current, updates)
        }
    }

    pub fn insert(&mut self, key: K) -> bool {
        let height;

        unsafe {
            let (current, mut updates)
                = self.find_lower_bound_with_updates(&key);

            // The lower bound's next node, if present, could be the same as the
            // key we are looking for, so we could abort early here
            if current.has_next(0) {
                if current.next(0).key() == &key {
                    return false;
                }
            }

            height = self.random_height();

            // Generate the node. All memory allocation is done using Box so
            // that we can actually free it using Box later
            let node = Box::into_raw(Box::new(Node::new(key, height)));
            for h in 0..height + 1 {
                if updates[h].has_next(h) {
                    (*node).set_next(h, updates[h].mut_ptr_next(h));
                }

                updates[h].set_next(h, node);
            }
        }

        self.height_ = std::cmp::max(self.height_, height);
        self.length_ += 1;
        true
    }

    pub fn get(&self, key: &K) -> Option<&K> {
        let mut node : &Node<K> = self.find_lower_bound(key);
        if node.has_next(0) {
            node = node.next(0);
            if node.key() == key {
                return Some(node.key());
            }
        }

        None
    }

    #[inline(always)]
    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn remove(&mut self, key: &K) -> bool {
        unsafe {
            let (current, mut updates)
                = self.find_lower_bound_with_updates(&key);

            // 'current' is the lower bound to the node, so if it doesn't have a
            // next node at level 0, it means that 'key' is not present. If it
            // does exist, then there is a possibility that it may be greater
            // than the actual key we are looking for
            if !current.has_next(0) {
                return false
            }

            let next = current.mut_ptr_next(0);
            // If the key is not the one that we are looking for, then that
            // means we are done too
            if (*next).key() != key {
                return false
            }

            // Since the range is a [,) operation, we need to add one to the
            // height
            for h in 0..(*next).height() + 1 {
                updates[h].set_next(h, (*next).mut_ptr_next(h));
            }

            // Free the memory for the 'next' pointer
            Box::from_raw(next);
            return true
        }
    }

    pub fn replace(&mut self, key: K) -> Option<K> {
        let current = self.find_lower_bound(&key);

        // 'current' is the lower bound to the node, so if it doesn't have a
        // next node at level 0, it means that 'key' is not present. If it
        // does exist, then there is a possibility that it may be greater
        // than the actual key we are looking for
        if !current.has_next(0) {
            return None
        }

        let next = current.mut_next(0);
        // If the key is not the one that we are looking for, then that
        // means we are done too
        if next.key() != &key {
            return None
        }

        return Some(next.replace_key(key))
    }

    // TODO(jbayardo): implement range
}

impl<K: Display + Copy> Display for SkipList<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[").unwrap();

        for key in self.iter() {
            write!(f, "{} ", key).unwrap();
        }

        write!(f, "]").unwrap();
        std::result::Result::Ok(())
    }

}

impl<K> Drop for SkipList<K> {
    fn drop(&mut self) {
        unsafe {
            let mut current = self.head_;
            while (*current).has_next(0) {
                let next = (*current).mut_ptr_next(0);
                Box::from_raw(current);
                current = next;
            }
        }
    }
}