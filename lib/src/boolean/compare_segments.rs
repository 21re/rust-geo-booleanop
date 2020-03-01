use super::helper::Float;
use super::segment_intersection::{intersection, LineIntersection};
use super::signed_area::signed_area;
use super::sweep_event::SweepEvent;
use std::cmp::Ordering;
use std::rc::Rc;

use super::helper;

pub fn compare_segments<F>(se1_l: &Rc<SweepEvent<F>>, se2_l: &Rc<SweepEvent<F>>) -> Ordering
where
    F: Float,
{
    debug_assert!(
        se1_l.is_left(),
        "compare_segments requires left-events, got a right-event."
    );
    debug_assert!(
        se2_l.is_left(),
        "compare_segments requires left-events, got a right-event."
    );
    debug_assert!(
        se1_l.get_other_event().is_some(),
        "missing right-event in compare_segments"
    );
    debug_assert!(
        se2_l.get_other_event().is_some(),
        "missing right-event in compare_segments"
    );

    if Rc::ptr_eq(&se1_l, &se2_l) {
        return Ordering::Equal;
    }

    // The main logic of compare segments is to check the orientation of the later/older
    // SweepEvent w.r.t. the segment of the earlier/newer one. The logic is easier to
    // express by swapping them here according to their temporal order. In case we have
    // to swap, the result function must be inverted accordingly.
    let (se_old_l, se_new_l, less_if) = if se1_l.is_before(&se2_l) {
        (se1_l, se2_l, helper::less_if as fn(bool) -> Ordering)
    } else {
        (se2_l, se1_l, helper::less_if_inversed as fn(bool) -> Ordering)
    };

    if let (Some(se_old_r), Some(se_new_r)) = (se_old_l.get_other_event(), se_new_l.get_other_event()) {
        let sa_l = signed_area(se_old_l.point, se_old_r.point, se_new_l.point);
        let sa_r = signed_area(se_old_l.point, se_old_r.point, se_new_r.point);
        if sa_l != 0. || sa_r != 0. {
            // Segments are not collinear

            // Left endpoints exactly identical? Use the right endpoint to sort
            if se_old_l.point == se_new_l.point {
                return less_if(se_old_l.is_below(se_new_r.point));
            }

            // Left endpoints identical in x, but different in y? Sort by y
            if se_old_l.point.x == se_new_l.point.x {
                return less_if(se_old_l.point.y < se_new_l.point.y);
            }

            // If `l` and `r` lie on the same side of the reference segment,
            // no intersection check is necessary.
            if (sa_l > 0.) == (sa_r > 0.) {
                return less_if(sa_l > 0.);
            }

            // If `l` lies on the reference segment, compare based on `r`.
            if sa_l == 0. {
                return less_if(sa_r > 0.);
            }

            // According to the signed-area values the segments cross. Verify if
            // we can get an intersection point whic is truely different from `l`.
            let inter = intersection(se_old_l.point, se_old_r.point, se_new_l.point, se_new_r.point);
            match inter {
                LineIntersection::None => return less_if(sa_l > 0.),
                LineIntersection::Point(p) => {
                    if p == se_new_l.point {
                        return less_if(sa_r > 0.);
                    } else {
                        return less_if(sa_l > 0.);
                    }
                }
                _ => {} // go into collinear logic below
            }
        }

        // Segments are collinear
        if se_old_l.is_subject == se_new_l.is_subject {
            if se_old_l.point == se_new_l.point {
                // Previously this was returning Ordering::Equal if the segments had identical
                // left and right endpoints. I think in order to properly support self-overlapping
                // segments we must return Ordering::Equal if and only if segments are the same
                // by identity (the Rc::ptr_eq above).
                less_if(se_old_l.contour_id < se_new_l.contour_id)
            } else {
                // Fallback to purely temporal-based comparison. Since `less_if` already
                // encodes "earlier-is-less" semantics, no comparison is needed.
                less_if(true)
            }
        } else {
            less_if(se_old_l.is_subject)
        }
    } else {
        debug_assert!(false, "Other events should always be defined in compare_segment.");
        less_if(true)
    }
}

#[cfg(test)]
mod test {
    use super::super::sweep_event::SweepEvent;
    use super::compare_segments;
    use crate::splay::SplaySet;
    use geo_types::Coordinate;
    use std::cmp::Ordering;
    use std::rc::{Rc, Weak};

    macro_rules! assert_ordering {
        ($se1:expr, $se2:expr, $ordering:expr) => {
            let inverse_ordering = match $ordering {
                Ordering::Less => Ordering::Greater,
                Ordering::Greater => Ordering::Less,
                _ => Ordering::Equal,
            };
            assert_eq!(
                compare_segments(&$se1, &$se2),
                $ordering,
                "Comparing se1/se2 with expected value {:?}",
                $ordering
            );
            assert_eq!(
                compare_segments(&$se2, &$se1),
                inverse_ordering,
                "Comparing se2/se1 with expected value {:?}",
                inverse_ordering
            );
        };
    }

    fn make_simple(
        contour_id: u32,
        x: f64,
        y: f64,
        other_x: f64,
        other_y: f64,
        is_subject: bool,
    ) -> (Rc<SweepEvent<f64>>, Rc<SweepEvent<f64>>) {
        let other = SweepEvent::new_rc(
            contour_id,
            Coordinate { x: other_x, y: other_y },
            false,
            Weak::new(),
            is_subject,
            true,
        );
        let event = SweepEvent::new_rc(
            contour_id,
            Coordinate { x, y },
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
    fn not_collinear_shared_left_right_first() {
        let (se1, _other1) = make_simple(0, 0.0, 0.0, 1.0, 1.0, false);
        let (se2, _other2) = make_simple(0, 0.0, 0.0, 2.0, 3.0, false);

        let mut tree = SplaySet::new(compare_segments);

        tree.insert(se1);
        tree.insert(se2);

        let min_other = tree.min().unwrap().get_other_event().unwrap();
        let max_other = tree.max().unwrap().get_other_event().unwrap();

        assert_eq!(max_other.point, Coordinate { x: 2.0, y: 3.0 });
        assert_eq!(min_other.point, Coordinate { x: 1.0, y: 1.0 });
    }

    #[test]
    fn not_collinear_different_left_point_right_sort_y() {
        let (se1, _other1) = make_simple(0, 0.0, 1.0, 1.0, 1.0, false);
        let (se2, _other2) = make_simple(0, 0.0, 2.0, 2.0, 3.0, false);

        let mut tree = SplaySet::new(compare_segments);

        tree.insert(se1);
        tree.insert(se2);

        let min_other = tree.min().unwrap().get_other_event().unwrap();
        let max_other = tree.max().unwrap().get_other_event().unwrap();

        assert_eq!(min_other.point, Coordinate { x: 1.0, y: 1.0 });
        assert_eq!(max_other.point, Coordinate { x: 2.0, y: 3.0 });
    }

    #[test]
    fn not_collinear_order_in_sweep_line() {
        let (se1, _other1) = make_simple(0, 0.0, 1.0, 2.0, 1.0, false);
        let (se2, _other2) = make_simple(0, -1.0, 0.0, 2.0, 3.0, false);
        let (se3, _other3) = make_simple(0, 0.0, 1.0, 3.0, 4.0, false);
        let (se4, _other4) = make_simple(0, -1.0, 0.0, 3.0, 1.0, false);

        assert_eq!(se1.cmp(&se2), Ordering::Less);
        assert!(!se2.is_below(se1.point));
        assert!(se2.is_above(se1.point));

        assert_ordering!(se1, se2, Ordering::Less);

        assert_eq!(se3.cmp(&se4), Ordering::Less);
        assert!(!se4.is_above(se3.point));
    }

    #[test]
    fn not_collinear_first_point_is_below() {
        let (se2, _other2) = make_simple(0, 1.0, 1.0, 5.0, 1.0, false);
        let (se1, _other1) = make_simple(0, -1.0, 0.0, 2.0, 3.0, false);

        assert!(!se1.is_below(se2.point));
        assert_ordering!(se1, se2, Ordering::Greater);
    }

    #[test]
    fn collinear_segments() {
        let (se1, _other1) = make_simple(0, 1.0, 1.0, 5.0, 1.0, true);
        let (se2, _other2) = make_simple(0, 2.0, 01.0, 3.0, 1.0, false);

        assert_ne!(se1.is_subject, se2.is_subject);
        assert_ordering!(se1, se2, Ordering::Less);
    }

    #[test]
    fn collinear_shared_left_point() {
        {
            let (se1, _other2) = make_simple(1, 0.0, 1.0, 5.0, 1.0, false);
            let (se2, _other1) = make_simple(2, 0.0, 1.0, 3.0, 1.0, false);

            assert_eq!(se1.is_subject, se2.is_subject);
            assert_eq!(se1.point, se2.point);

            assert_ordering!(se1, se2, Ordering::Less);
        }
        {
            let (se1, _other2) = make_simple(2, 0.0, 1.0, 5.0, 1.0, false);
            let (se2, _other1) = make_simple(1, 0.0, 1.0, 3.0, 1.0, false);

            assert_ordering!(se1, se2, Ordering::Greater);
        }
    }

    #[test]
    fn collinear_same_polygon_different_left() {
        let (se1, _other2) = make_simple(0, 1.0, 1.0, 5.0, 1.0, true);
        let (se2, _other1) = make_simple(0, 2.0, 1.0, 3.0, 1.0, true);

        assert_eq!(se1.is_subject, se2.is_subject);
        assert_ne!(se1.point, se2.point);
        assert_ordering!(se1, se2, Ordering::Less);
    }

    #[test]
    fn t_shaped_cases() {
        // shape:  /
        //        /\
        let (se1, _other1) = make_simple(0, 0.0, 0.0, 1.0, 1.0, true);
        let (se2, _other2) = make_simple(0, 0.5, 0.5, 1.0, 0.0, true);
        assert_ordering!(se1, se2, Ordering::Greater);

        // shape: \/
        //         \
        let (se1, _other1) = make_simple(0, 0.0, 1.0, 1.0, 0.0, true);
        let (se2, _other2) = make_simple(0, 0.5, 0.5, 1.0, 1.0, true);
        assert_ordering!(se1, se2, Ordering::Less);

        // shape: T
        let (se1, _other1) = make_simple(0, 0.0, 1.0, 1.0, 1.0, true);
        let (se2, _other2) = make_simple(0, 0.5, 0.0, 0.5, 1.0, true);
        assert_ordering!(se1, se2, Ordering::Greater);

        // shape: T upside down
        let (se1, _other1) = make_simple(0, 0.0, 0.0, 1.0, 0.0, true);
        let (se2, _other2) = make_simple(0, 0.5, 0.0, 0.5, 1.0, true);
        assert_ordering!(se1, se2, Ordering::Less);
    }

    #[test]
    fn vertical_segment() {
        // vertical reference segment at x = 0, expanding from y = -1 to +1.
        let (se1, _other1) = make_simple(0, 0.0, -1.0, 0.0, 1.0, true);

        // "above" cases
        let (se2, _other2) = make_simple(0, -1.0, 1.0, 0.0, 1.0, true);
        assert_ordering!(se1, se2, Ordering::Less);
        let (se2, _other2) = make_simple(0, 0.0, 1.0, 1.0, 1.0, true);
        assert_ordering!(se1, se2, Ordering::Less);
        let (se2, _other2) = make_simple(0, -1.0, 2.0, 0.0, 2.0, true);
        assert_ordering!(se1, se2, Ordering::Less);
        let (se2, _other2) = make_simple(0, 0.0, 2.0, 1.0, 2.0, true);
        assert_ordering!(se1, se2, Ordering::Less);
        let (se2, _other2) = make_simple(0, 0.0, 1.0, 0.0, 2.0, true);
        assert_ordering!(se1, se2, Ordering::Less);

        // "below" cases
        let (se2, _other2) = make_simple(0, -1.0, -1.0, 0.0, -1.0, true);
        assert_ordering!(se1, se2, Ordering::Greater);
        let (se2, _other2) = make_simple(0, 0.0, -1.0, 1.0, -1.0, true);
        assert_ordering!(se1, se2, Ordering::Greater);
        let (se2, _other2) = make_simple(0, -1.0, -2.0, 0.0, -2.0, true);
        assert_ordering!(se1, se2, Ordering::Greater);
        let (se2, _other2) = make_simple(0, 0.0, -2.0, 1.0, -2.0, true);
        assert_ordering!(se1, se2, Ordering::Greater);
        let (se2, _other2) = make_simple(0, 0.0, -2.0, 0.0, -1.0, true);
        assert_ordering!(se1, se2, Ordering::Greater);

        // overlaps
        let (se2, _other2) = make_simple(0, 0.0, -0.5, 0.0, 0.5, true);
        assert_ordering!(se1, se2, Ordering::Less);
        // When left endpoints are identical, the ordering is no longer anti-symmetric.
        // TODO: Decide if this is a problem.
        // let (se2, _other2) = make_simple(0, 0.0, -1.0, 0.0, 0.0, true);
        // assert_ordering!(se1, se2, Ordering::Less); // fails because of its not anti-symmetric.
    }
}
