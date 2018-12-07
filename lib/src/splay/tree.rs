use super::node::Node;
use std::cell::UnsafeCell;
use std::cmp::Ordering;
use std::fmt;
use std::mem;
use std::ops::{Index, IndexMut};

pub struct SplayTree<K, V, C>
where
    C: Fn(&K, &K) -> Ordering,
{
    comparator: C,
    root: UnsafeCell<Option<Box<Node<K, V>>>>,
    size: usize,
}

impl<K, V, C> SplayTree<K, V, C>
where
    C: Fn(&K, &K) -> Ordering,
{
    pub fn new(comparator: C) -> SplayTree<K, V, C> {
        SplayTree {
            comparator,
            root: UnsafeCell::new(None),
            size: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        self.root_mut().take();
        self.size = 0;
    }

    pub fn contains(&self, key: &K) -> bool {
        self.find_key(key).is_some()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        // Splay trees are self-modifying, which is the cause of this ugly mess
        match self.root_mut() {
            Some(ref mut root) => {
                splay(key, root, &self.comparator);
                if (self.comparator)(key, &root.key) == Ordering::Equal {
                    Some(&root.value)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /// Return a mutable reference to the value corresponding to the key
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        // Splay trees are self-modifying, which is the cause of this ugly mess
        match self.root_mut() {
            Some(ref mut root) => {
                splay(key, root, &self.comparator);
                if (self.comparator)(key, &root.key) == Ordering::Equal {
                    Some(&mut root.value)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn find_key(&self, key: &K) -> Option<&K> {
        // Splay trees are self-modifying, which is the cause of this ugly mess
        match self.root_mut() {
            Some(ref mut root) => {
                splay(key, root, &self.comparator);
                if (self.comparator)(key, &root.key) == Ordering::Equal {
                    Some(&root.key)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn next(&self, key: &K) -> Option<(&K, &V)> {
        // Splay trees are self-modifying, which is the cause of this ugly mess
        let mut node: &Node<K, V> = match self.root_mut() {
            Some(ref mut root) => {
                splay(key, root, &self.comparator);
                root
            }
            None => return None,
        };

        let mut successor: Option<(&K, &V)> = None;

        loop {
            match (self.comparator)(key, &node.key) {
                Ordering::Less => {
                    successor = Some((&node.key, &node.value));
                    match node.left {
                        Some(ref left) => node = left,
                        None => break,
                    }
                }
                Ordering::Equal | Ordering::Greater => match node.right {
                    Some(ref right) => node = right,
                    None => break,
                },
            }
        }

        successor
    }

    pub fn prev(&self, key: &K) -> Option<(&K, &V)> {
        // Splay trees are self-modifying, which is the cause of this ugly mess
        let mut node: &Node<K, V> = match self.root_mut() {
            Some(ref mut root) => {
                splay(key, root, &self.comparator);
                root
            }
            None => return None,
        };

        let mut predecessor: Option<(&K, &V)> = None;

        loop {
            match (self.comparator)(key, &node.key) {
                Ordering::Equal | Ordering::Less => match node.left {
                    Some(ref left) => node = left,
                    None => break,
                },
                Ordering::Greater => {
                    predecessor = Some((&node.key, &node.value));
                    match node.right {
                        Some(ref right) => node = right,
                        None => break,
                    }
                }
            }
        }

        predecessor
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.root_mut() {
            Some(ref mut root) => {
                splay(&key, root, &self.comparator);

                match (self.comparator)(&key, &root.key) {
                    Ordering::Equal => {
                        let old = mem::replace(&mut root.value, value);
                        return Some(old);
                    }
                    Ordering::Less => {
                        let left = root.pop_left();
                        let new = Node::new_boxed(key, value, left, None);
                        let prev = mem::replace(root, new);
                        root.right = Some(prev);
                    }
                    Ordering::Greater => {
                        let right = root.pop_right();
                        let new = Node::new_boxed(key, value, None, right);
                        let prev = mem::replace(root, new);
                        root.left = Some(prev);
                    }
                }
            }
            slot => {
                *slot = Some(Node::new_boxed(key, value, None, None));
            }
        }
        self.size += 1;
        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        match *self.root_mut() {
            None => {
                return None;
            }
            Some(ref mut root) => {
                splay(key, root, &self.comparator);
                if (self.comparator)(key, &root.key) != Ordering::Equal {
                    return None;
                }
            }
        }

        let (value, left, right) = match *self.root_mut().take().unwrap() {
            Node { left, right, value, .. } => (value, left, right),
        };

        *self.root_mut() = match left {
            None => right,
            Some(mut node) => {
                splay(key, &mut node, &self.comparator);
                node.right = right;
                Some(node)
            }
        };

        self.size -= 1;
        Some(value)
    }

    pub fn min(&self) -> Option<&K> {
        self.min_node().map(|node| &node.key)
    }

    pub fn max(&self) -> Option<&K> {
        self.max_node().map(|node| &node.key)
    }

    fn min_node(&self) -> Option<&Node<K, V>> {
        match self.root_ref() {
            Some(ref root) => {
                let mut node = root;

                while let Some(ref left) = node.left {
                    node = left
                }
                Some(node)
            }
            None => None,
        }
    }

    fn max_node(&self) -> Option<&Node<K, V>> {
        match self.root_ref() {
            Some(ref root) => {
                let mut node = root;

                while let Some(ref right) = node.right {
                    node = right
                }
                Some(node)
            }
            None => None,
        }
    }

    // Messy code ahead
    #[allow(clippy::mut_from_ref)]
    fn root_mut(&self) -> &mut Option<Box<Node<K, V>>> {
        unsafe { &mut *self.root.get() }
    }

    fn root_ref(&self) -> &Option<Box<Node<K, V>>> {
        unsafe { &*self.root.get() }
    }
}

impl<'a, K, V, C> Index<&'a K> for SplayTree<K, V, C>
where
    C: Fn(&K, &K) -> Ordering,
{
    type Output = V;

    fn index(&self, index: &'a K) -> &V {
        self.get(index).expect("key not present in SplayMap")
    }
}
impl<'a, K, V, C> IndexMut<&'a K> for SplayTree<K, V, C>
where
    C: Fn(&K, &K) -> Ordering,
{
    fn index_mut(&mut self, index: &K) -> &mut V {
        self.get_mut(index).expect("key not present in SplayMap")
    }
}

impl<K, V, C> Extend<(K, V)> for SplayTree<K, V, C>
where
    C: Fn(&K, &K) -> Ordering,
{
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, i: I) {
        for (k, v) in i {
            self.insert(k, v);
        }
    }
}

impl<K, V, C> IntoIterator for SplayTree<K, V, C>
where
    C: Fn(&K, &K) -> Ordering,
{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            cur: self.root_mut().take(),
            remaining: self.size,
        }
    }
}

impl<K, V, C> Drop for SplayTree<K, V, C>
where
    C: Fn(&K, &K) -> Ordering,
{
    fn drop(&mut self) {
        self.clear();
    }
}

impl<K, V, C> fmt::Debug for SplayTree<K, V, C>
where
    K: fmt::Debug,
    V: fmt::Debug,
    C: Fn(&K, &K) -> Ordering,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.root_ref())
    }
}

pub struct IntoIter<K, V> {
    cur: Option<Box<Node<K, V>>>,
    remaining: usize,
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);
    fn next(&mut self) -> Option<(K, V)> {
        let mut cur = match self.cur.take() {
            Some(cur) => cur,
            None => return None,
        };
        loop {
            match cur.pop_left() {
                Some(node) => {
                    let mut node = node;
                    cur.left = node.pop_right();
                    node.right = Some(cur);
                    cur = node;
                }

                None => {
                    self.cur = cur.pop_right();
                    // left and right fields are both None
                    let node = *cur;
                    let Node { key, value, .. } = node;
                    self.remaining -= 1;
                    return Some((key, value));
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<K, V> DoubleEndedIterator for IntoIter<K, V> {
    fn next_back(&mut self) -> Option<(K, V)> {
        let mut cur = match self.cur.take() {
            Some(cur) => cur,
            None => return None,
        };
        loop {
            match cur.pop_right() {
                Some(node) => {
                    let mut node = node;
                    cur.right = node.pop_left();
                    node.left = Some(cur);
                    cur = node;
                }

                None => {
                    self.cur = cur.pop_left();
                    // left and right fields are both None
                    let node = *cur;
                    let Node { key, value, .. } = node;
                    self.remaining -= 1;
                    return Some((key, value));
                }
            }
        }
    }
}

impl<K, V> ExactSizeIterator for IntoIter<K, V> {}

/// Performs a top-down splay operation on a tree rooted at `node`. This will
/// modify the pointer to contain the new root of the tree once the splay
/// operation is done. When finished, if `key` is in the tree, it will be at the
/// root. Otherwise the closest key to the specified key will be at the root.
#[allow(clippy::borrowed_box)]
fn splay<K, V, C>(key: &K, node: &mut Box<Node<K, V>>, comparator: &C)
where
    C: Fn(&K, &K) -> Ordering,
{
    let mut newleft = None;
    let mut newright = None;

    // Eplicitly grab a new scope so the loans on newleft/newright are
    // terminated before we move out of them.
    {
        // Yes, these are backwards, that's intentional.
        let mut l = &mut newright;
        let mut r = &mut newleft;

        loop {
            match comparator(key, &node.key) {
                // Found it, yay!
                Ordering::Equal => break,

                Ordering::Less => {
                    let mut left = match node.pop_left() {
                        Some(left) => left,
                        None => break,
                    };
                    // rotate this node right if necessary
                    if comparator(key, &left.key) == Ordering::Less {
                        // A bit odd, but avoids drop glue
                        mem::swap(&mut node.left, &mut left.right);
                        mem::swap(&mut left, node);
                        let none = mem::replace(&mut node.right, Some(left));
                        match mem::replace(&mut node.left, none) {
                            Some(l) => {
                                left = l;
                            }
                            None => break,
                        }
                    }

                    *r = Some(mem::replace(node, left));
                    let tmp = r;
                    r = &mut tmp.as_mut().unwrap().left;
                }

                // If you look closely, you may have seen some similar code
                // before
                Ordering::Greater => {
                    let mut right = match node.pop_right() {
                        Some(right) => right,
                        None => break,
                    };

                    if comparator(key, &right.key) == Ordering::Greater {
                        mem::swap(&mut node.right, &mut right.left);
                        mem::swap(&mut right, node);
                        let none = mem::replace(&mut node.left, Some(right));
                        match mem::replace(&mut node.right, none) {
                            Some(r) => {
                                right = r;
                            }
                            None => break,
                        }
                    }
                    *l = Some(mem::replace(node, right));
                    let tmp = l;
                    l = &mut tmp.as_mut().unwrap().right;
                }
            }
        }

        mem::swap(l, &mut node.left);
        mem::swap(r, &mut node.right);
    }

    node.left = newright;
    node.right = newleft;
}
