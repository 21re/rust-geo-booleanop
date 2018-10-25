mod node;
mod set;
mod tree;

pub use self::set::SplaySet;
pub use self::tree::SplayTree;

#[cfg(test)]
mod test {
    use super::*;
    use rand::random;
    use std::cmp::Ordering;
    use std::i32;

    fn int_comparator(a: &i32, b: &i32) -> Ordering {
        a.cmp(b)
    }

    #[test]
    fn insert_simple() {
        let mut t = SplayTree::new(int_comparator);
        assert_eq!(t.insert(1, 2), None);
        assert_eq!(t.insert(1, 3), Some(2));
        assert_eq!(t.insert(2, 3), None);

        assert_eq!(t.len(), 2);
        assert_eq!(t.min(), Some(&1));
        assert_eq!(t.max(), Some(&2));

        assert_eq!(t.next(&1), Some((&2, &3)));
        assert_eq!(t.next(&2), None);

        assert_eq!(t.prev(&2), Some((&1, &3)));
        assert_eq!(t.prev(&1), None);
        assert_eq!(t.prev(&100), Some((&2, &3)));
    }

    #[test]
    fn remove_simple() {
        let mut t = SplayTree::new(int_comparator);
        assert_eq!(t.insert(1, 2), None);
        assert_eq!(t.insert(2, 3), None);
        assert_eq!(t.insert(3, 4), None);
        assert_eq!(t.insert(0, 5), None);
        assert_eq!(t.remove(&1), Some(2));

        assert_eq!(t.len(), 3);
        assert_eq!(t.min(), Some(&0));
        assert_eq!(t.max(), Some(&3));

        assert_eq!(t.next(&0), Some((&2, &3)));
        assert_eq!(t.next(&1), Some((&2, &3)));
        assert_eq!(t.next(&2), Some((&3, &4)));
        assert_eq!(t.next(&3), None);

        assert_eq!(t.prev(&2), Some((&0, &5)));
        assert_eq!(t.prev(&0), None);
        assert_eq!(t.prev(&100), Some((&3, &4)));
    }

    #[test]
    fn pop_simple() {
        let mut t = SplayTree::new(int_comparator);
        assert_eq!(t.insert(1, 2), None);
        assert_eq!(t.insert(2, 3), None);
        assert_eq!(t.insert(3, 4), None);
        assert_eq!(t.insert(0, 5), None);

        assert_eq!(t.min(), Some(&0));
        assert_eq!(t.max(), Some(&3));

        assert_eq!(t.prev(&0), None);
        assert_eq!(t.prev(&1), Some((&0, &5)));
        assert_eq!(t.next(&0), Some((&1, &2)));
        assert_eq!(t.next(&1), Some((&2, &3)));
        assert_eq!(t.next(&2), Some((&3, &4)));
        assert_eq!(t.next(&3), None);

        assert_eq!(t.remove(&1), Some(2));
        assert_eq!(t.remove(&1), None);
        assert_eq!(t.remove(&0), Some(5));

        assert_eq!(t.min(), Some(&2));
        assert_eq!(t.max(), Some(&3));
    }

    #[test]
    fn test_len() {
        let mut m = SplayTree::new(int_comparator);
        assert_eq!(m.insert(3, 6), None);
        assert_eq!(m.len(), 1);
        assert_eq!(m.insert(0, 0), None);
        assert_eq!(m.len(), 2);
        assert_eq!(m.insert(4, 8), None);
        assert_eq!(m.len(), 3);
        assert_eq!(m.remove(&3), Some(6));
        assert_eq!(m.len(), 2);
        assert_eq!(m.remove(&5), None);
        assert_eq!(m.len(), 2);
        assert_eq!(m.insert(2, 4), None);
        assert_eq!(m.len(), 3);
        assert_eq!(m.insert(1, 2), None);
        assert_eq!(m.len(), 4);
    }

    #[test]
    fn test_clear() {
        let mut m = SplayTree::new(int_comparator);
        m.clear();
        assert_eq!(m.insert(5, 11), None);
        assert_eq!(m.insert(12, -3), None);
        assert_eq!(m.insert(19, 2), None);
        m.clear();
        assert_eq!(m.get(&5), None);
        assert_eq!(m.get(&12), None);
        assert_eq!(m.get(&19), None);
        assert!(m.is_empty());
    }

    #[test]
    fn insert_replace() {
        let mut m = SplayTree::new(int_comparator);
        assert_eq!(m.insert(5, 2), None);
        assert_eq!(m.insert(2, 9), None);
        assert_eq!(m.insert(2, 11), Some(9));
        assert_eq!(m[&2], 11);
    }

    #[test]
    fn find_empty() {
        let m = SplayTree::<i32, i32, _>::new(int_comparator);
        assert_eq!(m.get(&5), None);
    }

    #[test]
    fn find_not_found() {
        let mut m = SplayTree::new(int_comparator);
        assert_eq!(m.insert(1, 2), None);
        assert_eq!(m.insert(5, 3), None);
        assert_eq!(m.insert(9, 3), None);
        assert_eq!(m.get(&2), None);
    }

    #[test]
    fn get_works() {
        let mut m = SplayTree::new(int_comparator);
        m.insert(1, 1);
        assert_eq!(m[&1], 1);
    }

    #[test]
    fn into_iter() {
        let mut m = SplayTree::new(int_comparator);
        m.insert(1, 1);
        m.insert(2, 1);
        m.insert(0, 1);
        let mut cur = 0;
        for (k, v) in m {
            assert_eq!(k, cur);
            assert_eq!(v, 1);
            cur += 1;
        }
    }

    #[test]
    fn into_iter_backwards() {
        let mut m = SplayTree::new(int_comparator);
        m.insert(1, 1);
        m.insert(2, 1);
        m.insert(0, 1);
        let mut cur = 2;
        for (k, v) in m.into_iter().rev() {
            assert_eq!(k, cur);
            assert_eq!(v, 1);
            cur -= 1;
        }
    }

    #[test]
    fn large() {
        let mut m = SplayTree::new(int_comparator);
        let mut v = Vec::new();
        let mut min = i32::MAX;
        let mut max = i32::MIN;

        for _ in 0..400 {
            let i: i32 = random();
            m.insert(i, i);
            v.push(i);
            min = i32::min(min, i);
            max = i32::max(max, i);
        }

        for i in v.iter() {
            assert!(m.contains(i));
            assert_eq!(&m[i], i);
            match m.next(i) {
                Some((next, _)) => {
                    assert!(*next > *i);
                    assert_eq!(m.prev(&next), Some((i, i)));
                }
                None => assert_eq!(*i, max),
            }
            match m.prev(i) {
                Some((prev, _)) => {
                    assert!(*prev < *i);
                    assert_eq!(m.next(&prev), Some((i, i)));
                }
                None => assert_eq!(*i, min),
            }
        }
        assert_eq!(m.min(), Some(&min));
        assert_eq!(m.max(), Some(&max));
    }
}
