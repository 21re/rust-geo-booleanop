use super::helper::Float;
use geo_types::Coord;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::{Rc, Weak};

use super::helper::less_if;
use super::signed_area::signed_area;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EdgeType {
    Normal,
    NonContributing,
    SameTransition,
    DifferentTransition,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ResultTransition {
    None,
    InOut,
    OutIn,
}

#[derive(Clone, Debug)]
struct MutablePart<F>
where
    F: Float,
{
    left: bool,
    other_event: Weak<SweepEvent<F>>,
    prev_in_result: Weak<SweepEvent<F>>,
    edge_type: EdgeType,
    in_out: bool,
    other_in_out: bool,
    result_transition: ResultTransition,
    other_pos: i32,
    output_contour_id: i32,
}

#[derive(Clone, Debug)]
pub struct SweepEvent<F>
where
    F: Float,
{
    mutable: RefCell<MutablePart<F>>,
    pub contour_id: u32,
    pub point: Coord<F>,
    pub is_subject: bool,
    pub is_exterior_ring: bool,
}

impl<F> SweepEvent<F>
where
    F: Float,
{
    pub fn new_rc(
        contour_id: u32,
        point: Coord<F>,
        left: bool,
        other_event: Weak<SweepEvent<F>>,
        is_subject: bool,
        is_exterior_ring: bool,
    ) -> Rc<SweepEvent<F>> {
        Rc::new(SweepEvent {
            mutable: RefCell::new(MutablePart {
                left,
                other_event,
                prev_in_result: Weak::new(),
                edge_type: EdgeType::Normal,
                in_out: false,
                other_in_out: false,
                result_transition: ResultTransition::None,
                other_pos: 0,
                output_contour_id: -1,
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

    pub fn get_prev_in_result(&self) -> Option<Rc<SweepEvent<F>>> {
        self.mutable.borrow().prev_in_result.upgrade()
    }

    pub fn set_prev_in_result(&self, prev_in_result: &Rc<SweepEvent<F>>) {
        self.mutable.borrow_mut().prev_in_result = Rc::downgrade(prev_in_result);
    }

    pub fn unset_prev_in_result(&self) {
        self.mutable.borrow_mut().prev_in_result = Weak::new();
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
        self.mutable.borrow().result_transition != ResultTransition::None
    }

    pub fn set_result_transition(&self, result_transition: ResultTransition) {
        self.mutable.borrow_mut().result_transition = result_transition
    }

    pub fn get_result_transition(&self) -> ResultTransition {
        self.mutable.borrow().result_transition
    }

    pub fn set_in_out(&self, in_out: bool, other_in_out: bool) {
        let mut mutable = self.mutable.borrow_mut();

        mutable.in_out = in_out;
        mutable.other_in_out = other_in_out;
    }

    pub fn get_other_pos(&self) -> i32 {
        self.mutable.borrow().other_pos
    }

    pub fn set_other_pos(&self, other_pos: i32) {
        self.mutable.borrow_mut().other_pos = other_pos
    }

    pub fn get_output_contour_id(&self) -> i32 {
        self.mutable.borrow().output_contour_id
    }

    pub fn set_output_contour_id(&self, output_contour_id: i32) {
        self.mutable.borrow_mut().output_contour_id = output_contour_id
    }

    pub fn is_below(&self, p: Coord<F>) -> bool {
        if let Some(ref other_event) = self.get_other_event() {
            if self.is_left() {
                signed_area(self.point, other_event.point, p) > 0.
            } else {
                signed_area(other_event.point, self.point, p) > 0.
            }
        } else {
            false
        }
    }

    pub fn is_above(&self, p: Coord<F>) -> bool {
        !self.is_below(p)
    }

    pub fn is_vertical(&self) -> bool {
        match self.get_other_event() {
            Some(ref other_event) => self.point.x == other_event.point.x,
            None => false,
        }
    }

    /// Helper function to avoid confusion by inverted ordering
    pub fn is_before(&self, other: &SweepEvent<F>) -> bool {
        self > other
    }

    /// Helper function to avoid confusion by inverted ordering
    pub fn is_after(&self, other: &SweepEvent<F>) -> bool {
        self < other
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
    #[inline]
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
            if signed_area(p1, other1.point, other2.point) != 0. {
                return less_if(!self.is_below(other2.point));
            }
        }

        less_if(!self.is_subject && other.is_subject)
    }
}

#[cfg(feature = "debug-booleanop")]
pub trait JsonDebug {
    fn to_json_debug(&self) -> String;
    fn to_json_debug_short(&self) -> String;
}

#[cfg(feature = "debug-booleanop")]
impl<F> JsonDebug for Rc<SweepEvent<F>>
where
    F: Float,
{
    fn to_json_debug(&self) -> String {
        format!(
            "{{\"self\": {}, \"other\": {}}}",
            self.to_json_debug_short(),
            self.get_other_event().unwrap().to_json_debug_short(),
        )
    }

    fn to_json_debug_short(&self) -> String {
        format!(
            "{{\"addr\": \"{:p}\", \"point\": [{}, {}], \"type\": \"{}\", \"poly\": \"{}\"}}",
            *self,
            self.point.x,
            self.point.y,
            if self.is_left() { "L" } else { "R" },
            if self.is_subject { "A" } else { "B" },
        )
    }
}

#[cfg(test)]
mod test {
    use super::super::helper::test::xy;
    use super::*;

    pub fn se_pair(
        contour_id: u32,
        x: f64,
        y: f64,
        other_x: f64,
        other_y: f64,
        is_subject: bool,
    ) -> (Rc<SweepEvent<f64>>, Rc<SweepEvent<f64>>) {
        let other = SweepEvent::new_rc(
            contour_id,
            Coord { x: other_x, y: other_y },
            false,
            Weak::new(),
            is_subject,
            true,
        );
        let event = SweepEvent::new_rc(
            contour_id,
            Coord { x, y },
            true,
            Rc::downgrade(&other),
            is_subject,
            true,
        );
        // Make sure test cases fulfill the invariant of left/right relationship.
        assert!(event.is_before(&other));

        (event, other)
    }

    #[test]
    pub fn test_is_below() {
        let other_s1 = SweepEvent::new_rc(0, xy(1, 1), false, Weak::new(), false, true);
        let s1 = SweepEvent::new_rc(0, xy(0, 0), true, Rc::downgrade(&other_s1), false, true);
        let s2 = SweepEvent::new_rc(0, xy(0, 0), false, Rc::downgrade(&s1), false, true);

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
        let other_s1 = SweepEvent::new_rc(0, xy(1, 1), false, Weak::new(), false, true);
        let s1 = SweepEvent::new_rc(0, xy(0, 0), true, Rc::downgrade(&other_s1), false, true);
        let s2 = SweepEvent::new_rc(0, xy(0, 1), false, Rc::downgrade(&s1), false, true);

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
        let other_s1 = SweepEvent::new_rc(0, xy(0, 1), false, Weak::new(), false, true);
        let s1 = SweepEvent::new_rc(0, xy(0, 0), true, Rc::downgrade(&other_s1), false, true);
        let other_s2 = SweepEvent::new_rc(0, xy(0.0001, 1), false, Weak::new(), false, true);
        let s2 = SweepEvent::new_rc(0, xy(0, 0), true, Rc::downgrade(&other_s2), false, true);

        assert!(s1.is_vertical());
        assert!(!s2.is_vertical());
    }

    #[rustfmt::skip]
    #[test]
    fn test_order_star_pattern() {
        // This test verifies the assumption underlying the `precompute_iteration_order` logic:
        // Events with an identical points must be ordered:
        // - R events before L events
        // - R events in clockwise order
        // - L events in counter-clockwise order
        let id = 0;
        let z = 0.;

        // Group 'a' which have their right event at (0, 0), clockwise
        let (_av_l, av_r) = se_pair(id,  0., -1., z, z, true);   // vertical comes first
        let (_a1_l, a1_r) = se_pair(id, -2., -6., z, z, true);
        let (_a2_l, a2_r) = se_pair(id, -1., -2., z, z, true);
        let (_a3_l, a3_r) = se_pair(id, -1., -1., z, z, true);
        let (_a4_l, a4_r) = se_pair(id, -2., -1., z, z, true);
        let (_a5_l, a5_r) = se_pair(id, -2.,  1., z, z, true);
        let (_a6_l, a6_r) = se_pair(id, -1.,  1., z, z, true);
        let (_a7_l, a7_r) = se_pair(id, -1.,  2., z, z, true);
        let (_a8_l, a8_r) = se_pair(id, -2.,  6., z, z, true);

        // Group 'b' which have their left event at (0, 0), counter clockwise
        let (b1_l, _b1_r) = se_pair(id, z, z, 2., -6., true);
        let (b2_l, _b2_r) = se_pair(id, z, z, 1., -2., true);
        let (b3_l, _b3_r) = se_pair(id, z, z, 1., -1., true);
        let (b4_l, _b4_r) = se_pair(id, z, z, 2., -1., true);
        let (b5_l, _b5_r) = se_pair(id, z, z, 2.,  1., true);
        let (b6_l, _b6_r) = se_pair(id, z, z, 1.,  1., true);
        let (b7_l, _b7_r) = se_pair(id, z, z, 1.,  2., true);
        let (b8_l, _b8_r) = se_pair(id, z, z, 2.,  6., true);
        let (bv_l, _bv_r) = se_pair(id, z, z, 0.,  1., true);    // vertical comes last

        let events_expected_order = [
            av_r, a1_r, a2_r, a3_r, a4_r, a5_r, a6_r, a7_r, a8_r,
            b1_l, b2_l, b3_l, b4_l, b5_l, b6_l, b7_l, b8_l, bv_l,
        ];

        for i in 0 .. events_expected_order.len() - 1 {
            for j in i + 1 .. events_expected_order.len() {
                assert!(events_expected_order[i].is_before(&events_expected_order[j]));
            }
        }

    }
}
