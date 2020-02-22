use super::helper::Float;
use super::sweep_event::SweepEvent;
use geo_types::Coordinate;
use std::collections::BinaryHeap;
use std::rc::Rc;

pub fn divide_segment<F>(se_l: &Rc<SweepEvent<F>>, inter: Coordinate<F>, queue: &mut BinaryHeap<Rc<SweepEvent<F>>>)
where
    F: Float,
{
    debug_assert!(se_l.is_left());

    let se_r = match se_l.get_other_event() {
        Some(se) => se,
        None => return,
    };

    // The idea is to divide the segment based on the given `inter` coordinate as follows:
    //
    //     (se_l)--------(r)(l)--------(se_r)
    //
    // Under normal circumstances the resulting events satisfy the conditions:
    //
    //     se_l is before r, and l is before se_r.
    //
    // Since the intersection point computation is bounded to the interval [se_l.x, se_r.x]
    // it is impossible for r/l to fall outside the interval. This leaves the corner cases:
    //
    //  1. r.x == se_l.x and r.y < se_l.y: This corresponds to the case where the first
    //     sub-segment becomes a perfectly vertical line. The problem is that vertical
    //     segments always have to be processed from bottom to top consistencly. The
    //     theoretically correct event order would be r first (bottom), se_l later (top).
    //     However, se_l is the event just being processed, so there is no (easy) way of
    //     processing r before se_l. The easiest solution to the problem is to avoid it,
    //     by incrementing inter.x by one ULP.
    //  2. l.x == se_r.x and l.y > se_r.y: This corresponds to the case where the second
    //     sub-segment becomes a perfectly vertical line, and because of the bottom-to-top
    //     convention for vertical segment, the order of l and se_r must be swapped.
    //     In this case swapping is not a problem, because both events are in the future.
    //
    // See also: https://github.com/21re/rust-geo-booleanop/pull/11

    // Prevent from corner case 1
    let mut inter = inter;
    if inter.x == se_l.point.x && inter.y < se_l.point.y {
        inter.x = inter.x.nextafter(true);
    }

    let r = SweepEvent::new_rc(
        se_l.contour_id,
        inter,
        false,
        Rc::downgrade(&se_l),
        se_l.is_subject,
        true,
    );
    let l = SweepEvent::new_rc(
        se_l.contour_id,
        inter,
        true,
        Rc::downgrade(&se_r),
        se_l.is_subject,
        true,
    );

    // Corner case 1 should be impossible
    debug_assert!(se_l.is_before(&r));
    // Corner case 2 can be accounted for by swapping l / se_r
    if !l.is_before(&se_r) {
        se_r.set_left(true);
        l.set_left(false);
    }

    se_l.set_other_event(&r);
    se_r.set_other_event(&l);

    queue.push(l);
    queue.push(r);
}

#[cfg(test)]
mod test {
    use super::super::segment_intersection::{intersection, LineIntersection};
    use super::super::sweep_event::SweepEvent;
    use super::*;
    use geo_types::Coordinate;
    use std::collections::BinaryHeap;
    use std::rc::{Rc, Weak};

    fn make_simple(
        x: f64,
        y: f64,
        other_x: f64,
        other_y: f64,
        is_subject: bool,
    ) -> (Rc<SweepEvent<f64>>, Rc<SweepEvent<f64>>) {
        let other = SweepEvent::new_rc(
            0,
            Coordinate { x: other_x, y: other_y },
            false,
            Weak::new(),
            is_subject,
            true,
        );
        let event = SweepEvent::new_rc(0, Coordinate { x, y }, true, Rc::downgrade(&other), is_subject, true);

        (event, other)
    }

    #[test]
    fn divide_segments() {
        let (se1, other1) = make_simple(0.0, 0.0, 5.0, 5.0, true);
        let (se2, other2) = make_simple(0.0, 5.0, 5.0, 0.0, false);
        let mut queue = BinaryHeap::new();

        queue.push(se1.clone());
        queue.push(se2.clone());

        let inter = match intersection(se1.point, other1.point, se2.point, other2.point) {
            LineIntersection::Point(p) => p,
            _ => panic!("Not a point intersection"),
        };

        divide_segment(&se1, inter, &mut queue);
        divide_segment(&se2, inter, &mut queue);

        assert_eq!(queue.len(), 6);
    }
}
