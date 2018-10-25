use geo_types::Coordinate;
use num_traits::Float;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::{Rc, Weak};

use super::helper::less_if;
use super::signed_area::signed_area;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EdgeType {
    Normal,
    NonContributing,
    SameTransition,
    DifferentTransition,
}

#[derive(Clone)]
struct MutablePart<F>
where
    F: Float,
{
    left: bool,
    other_event: Weak<SweepEvent<F>>,
    edge_type: EdgeType,
    in_out: bool,
    other_in_out: bool,
    in_result: bool,
    pos: i32,
}

#[derive(Clone)]
pub struct SweepEvent<F>
where
    F: Float,
{
    mutable: RefCell<MutablePart<F>>,
    pub contour_id: u32,
    pub point: Coordinate<F>,
    pub is_subject: bool,
    pub is_exterior_ring: bool,
}

impl<F> SweepEvent<F>
where
    F: Float,
{
    pub fn new(
        contour_id: u32,
        point: Coordinate<F>,
        left: bool,
        other_event: Weak<SweepEvent<F>>,
        is_subject: bool,
        is_exterior_ring: bool,
    ) -> Rc<SweepEvent<F>> {
        Rc::new(SweepEvent {
            mutable: RefCell::new(MutablePart {
                left,
                other_event,
                edge_type: EdgeType::Normal,
                in_out: false,
                other_in_out: false,
                in_result: false,
                pos: 0,
            }),
            contour_id,
            point,
            is_subject,
            is_exterior_ring,
        })
    }

    pub fn is_left(&self) -> bool {
        self.mutable.borrow().left
    }

    pub fn set_left(&self, left: bool) {
        self.mutable.borrow_mut().left = left
    }

    pub fn get_other_event(&self) -> Option<Rc<SweepEvent<F>>> {
        self.mutable.borrow().other_event.upgrade()
    }

    pub fn set_other_event(&self, other_event: &Rc<SweepEvent<F>>) {
        self.mutable.borrow_mut().other_event = Rc::downgrade(other_event);
    }

    pub fn get_edge_type(&self) -> EdgeType {
        self.mutable.borrow().edge_type
    }

    pub fn set_edge_type(&self, edge_type: EdgeType) {
        self.mutable.borrow_mut().edge_type = edge_type
    }

    pub fn is_in_out(&self) -> bool {
        self.mutable.borrow().in_out
    }

    pub fn is_other_in_out(&self) -> bool {
        self.mutable.borrow().other_in_out
    }

    pub fn is_in_result(&self) -> bool {
        self.mutable.borrow().in_result
    }

    pub fn set_in_result(&self, in_result: bool) {
        self.mutable.borrow_mut().in_result = in_result
    }

    pub fn set_in_out(&self, in_out: bool, other_in_out: bool) {
        let mut mutable = self.mutable.borrow_mut();

        mutable.in_out = in_out;
        mutable.other_in_out = other_in_out;
    }

    pub fn get_pos(&self) -> i32 {
        self.mutable.borrow().pos
    }

    pub fn set_pos(&self, pos: i32) {
        self.mutable.borrow_mut().pos = pos
    }

    pub fn is_below(&self, p: Coordinate<F>) -> bool {
        if let Some(ref other_event) = self.get_other_event() {
            if self.is_left() {
                signed_area(self.point, other_event.point, p) > F::zero()
            } else {
                signed_area(other_event.point, self.point, p) > F::zero()
            }
        } else {
            false
        }
    }

    pub fn is_above(&self, p: Coordinate<F>) -> bool {
        !self.is_below(p)
    }

    pub fn is_vertical(&self) -> bool {
        match self.get_other_event() {
            Some(ref other_event) => self.point.x == other_event.point.x,
            None => false,
        }
    }
}

impl<F> PartialEq for SweepEvent<F>
where
    F: Float,
{
    fn eq(&self, other: &Self) -> bool {
        self.contour_id == other.contour_id
            && self.is_left() == other.is_left()
            && self.point == other.point
            && self.is_subject == other.is_subject
    }
}

impl<F> Eq for SweepEvent<F> where F: Float {}

impl<F> PartialOrd for SweepEvent<F>
where
    F: Float,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<F> Ord for SweepEvent<F>
where
    F: Float,
{
    fn cmp(&self, other: &Self) -> Ordering {
        // Ord is exactly the other way round as in the js implementation as BinaryHeap sorts decending
        let p1 = self.point;
        let p2 = other.point;

        if p1.x > p2.x {
            return Ordering::Less;
        }
        if p1.x < p2.x {
            return Ordering::Greater;
        }
        if p1.y > p2.y {
            return Ordering::Less;
        }
        if p1.y < p2.y {
            return Ordering::Greater;
        }

        if self.is_left() != other.is_left() {
            return less_if(self.is_left());
        }

        if let (Some(other1), Some(other2)) = (self.get_other_event(), other.get_other_event()) {
            if signed_area(p1, other1.point, other2.point) != F::zero() {
                return less_if(!self.is_below(other2.point));
            }
        }

        less_if(!self.is_subject && other.is_subject)
    }
}

#[cfg(test)]
mod test {
    use super::super::helper::test::xy;
    use super::*;

    #[test]
    pub fn test_is_below() {
        let other_s1 = SweepEvent::new(0, xy(1, 1), false, Weak::new(), false, true);
        let s1 = SweepEvent::new(0, xy(0, 0), true, Rc::downgrade(&other_s1), false, true);
        let s2 = SweepEvent::new(0, xy(0, 0), false, Rc::downgrade(&s1), false, true);

        assert!(s1.is_below(xy(0, 1)));
        assert!(s1.is_below(xy(1, 2)));
        assert!(!s1.is_below(xy(0, 0)));
        assert!(!s1.is_below(xy(5, -1)));

        assert!(!s2.is_below(xy(0, 1)));
        assert!(!s2.is_below(xy(1, 2)));
        assert!(!s2.is_below(xy(0, 0)));
        assert!(!s2.is_below(xy(5, -1)));
    }

    #[test]
    pub fn test_is_above() {
        let other_s1 = SweepEvent::new(0, xy(1, 1), false, Weak::new(), false, true);
        let s1 = SweepEvent::new(0, xy(0, 0), true, Rc::downgrade(&other_s1), false, true);
        let s2 = SweepEvent::new(0, xy(0, 1), false, Rc::downgrade(&s1), false, true);

        assert!(!s1.is_above(xy(0, 1)));
        assert!(!s1.is_above(xy(1, 2)));
        assert!(s1.is_above(xy(0, 0)));
        assert!(s1.is_above(xy(5, -1)));

        assert!(s2.is_above(xy(0, 1)));
        assert!(s2.is_above(xy(1, 2)));
        assert!(s2.is_above(xy(0, 0)));
        assert!(s2.is_above(xy(5, -1)));
    }

    #[test]
    pub fn test_is_vertical() {
        let other_s1 = SweepEvent::new(0, xy(0, 1), false, Weak::new(), false, true);
        let s1 = SweepEvent::new(0, xy(0, 0), true, Rc::downgrade(&other_s1), false, true);
        let other_s2 = SweepEvent::new(0, xy(0.0001, 1), false, Weak::new(), false, true);
        let s2 = SweepEvent::new(0, xy(0, 0), true, Rc::downgrade(&other_s2), false, true);

        assert!(s1.is_vertical());
        assert!(!s2.is_vertical());
    }
}
