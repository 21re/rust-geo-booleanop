use super::tree;
use super::SplayTree;
use std::cmp::Ordering;

pub struct SplaySet<T, C>
where
    C: Fn(&T, &T) -> Ordering,
{
    tree: SplayTree<T, (), C>,
}

impl<T, C> SplaySet<T, C>
where
    C: Fn(&T, &T) -> Ordering,
{
    pub fn new(comparator: C) -> SplaySet<T, C> {
        SplaySet {
            tree: SplayTree::new(comparator),
        }
    }

    pub fn len(&self) -> usize {
        self.tree.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn clear(&mut self) {
        self.tree.clear()
    }

    pub fn contains(&self, t: &T) -> bool {
        self.tree.contains(t)
    }

    pub fn find(&self, t: &T) -> Option<&T> {
        self.tree.find_key(t)
    }

    pub fn next(&self, t: &T) -> Option<&T> {
        self.tree.next(t).map(|kv| kv.0)
    }

    pub fn prev(&self, t: &T) -> Option<&T> {
        self.tree.prev(t).map(|kv| kv.0)
    }

    pub fn insert(&mut self, t: T) -> bool {
        self.tree.insert(t, ()).is_none()
    }

    pub fn remove(&mut self, t: &T) -> bool {
        self.tree.remove(t).is_some()
    }

    pub fn min(&self) -> Option<&T> {
        self.tree.min()
    }

    pub fn max(&self) -> Option<&T> {
        self.tree.max()
    }
}

impl<T, C> IntoIterator for SplaySet<T, C>
where
    C: Fn(&T, &T) -> Ordering,
{
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.tree.into_iter(),
        }
    }
}

pub struct IntoIter<T> {
    inner: tree::IntoIter<T, ()>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.inner.next().map(|p| p.0)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.inner.next_back().map(|(k, _)| k)
    }
}

impl<T, C> Extend<T> for SplaySet<T, C>
where
    C: Fn(&T, &T) -> Ordering,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, i: I) {
        for t in i {
            self.insert(t);
        }
    }
}
