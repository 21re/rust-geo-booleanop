use super::signed_area::signed_area;
use super::sweep_event::SweepEvent;
use num_traits::Float;
use std::cmp::Ordering;
use std::rc::Rc;

use super::helper::less_if;

pub fn compare_segments<F>(le1: &Rc<SweepEvent<F>>, le2: &Rc<SweepEvent<F>>) -> Ordering
where
    F: Float,
{
    if Rc::ptr_eq(&le1, &le2) {
        return Ordering::Equal;
    }

    if let (Some(other1), Some(other2)) = (le1.get_other_event(), le2.get_other_event()) {
        if signed_area(le1.point, other1.point, le2.point) != F::zero()
            || signed_area(le1.point, other1.point, other2.point) != F::zero()
        {
            if le1.point == le2.point {
                return less_if(le1.is_below(other2.point));
            }

            if le1.point.x == le2.point.x {
                return less_if(le1.point.y < le2.point.y);
            }

            if le1 < le2 {
                return less_if(le2.is_above(le1.point));
            }

            return less_if(le1.is_below(le2.point));
        }

        if le1.is_subject == le2.is_subject {
            if le1.point == le2.point {
                if other1.point == other2.point {
                    return Ordering::Equal;
                } else {
                    return less_if(le1.contour_id < le2.contour_id);
                }
            }
        } else {
            return less_if(le1.is_subject);
        }
    }

    less_if(le1 > le2)
}

#[cfg(test)]
mod test {
    use super::super::sweep_event::SweepEvent;
    use super::compare_segments;
    use geo_types::Coordinate;
    use splay::SplaySet;
    use std::cmp::Ordering;
    use std::rc::{Rc, Weak};

    fn make_simple(
        contour_id: u32,
        x: f64,
        y: f64,
        other_x: f64,
        other_y: f64,
        is_subject: bool,
    ) -> (Rc<SweepEvent<f64>>, Rc<SweepEvent<f64>>) {
        let other = SweepEvent::new(
            contour_id,
            Coordinate { x: other_x, y: other_y },
            false,
            Weak::new(),
            is_subject,
            true,
        );
        let event = SweepEvent::new(
            contour_id,
            Coordinate { x, y },
            true,
            Rc::downgrade(&other),
            is_subject,
            true,
        );

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

        assert_eq!(compare_segments(&se1, &se2), Ordering::Less);
        assert_eq!(compare_segments(&se2, &se1), Ordering::Greater);

        assert_eq!(se3.cmp(&se4), Ordering::Less);
        assert!(!se4.is_above(se3.point));
    }

    #[test]
    fn not_collinear_first_point_is_below() {
        let (se2, _other2) = make_simple(0, 1.0, 1.0, 5.0, 1.0, false);
        let (se1, _other1) = make_simple(0, -1.0, 0.0, 2.0, 3.0, false);

        assert!(!se1.is_below(se2.point));
        assert_eq!(compare_segments(&se1, &se2), Ordering::Greater);
    }

    #[test]
    fn collinear_segments() {
        let (se1, _other1) = make_simple(0, 1.0, 1.0, 5.0, 1.0, true);
        let (se2, _other2) = make_simple(0, 2.0, 01.0, 3.0, 1.0, false);

        assert_ne!(se1.is_subject, se2.is_subject);
        assert_eq!(compare_segments(&se1, &se2), Ordering::Less);
    }

    #[test]
    fn collinear_shared_left_point() {
        {
            let (se1, _other2) = make_simple(1, 0.0, 1.0, 5.0, 1.0, false);
            let (se2, _other1) = make_simple(2, 0.0, 1.0, 3.0, 1.0, false);

            assert_eq!(se1.is_subject, se2.is_subject);
            assert_eq!(se1.point, se2.point);

            assert_eq!(compare_segments(&se1, &se2), Ordering::Less);
        }
        {
            let (se1, _other2) = make_simple(2, 0.0, 1.0, 5.0, 1.0, false);
            let (se2, _other1) = make_simple(1, 0.0, 1.0, 3.0, 1.0, false);

            assert_eq!(compare_segments(&se1, &se2), Ordering::Greater);
        }
    }

    #[test]
    fn collinear_same_polygon_different_left() {
        let (se1, _other2) = make_simple(0, 1.0, 1.0, 5.0, 1.0, true);
        let (se2, _other1) = make_simple(0, 2.0, 1.0, 3.0, 1.0, true);

        assert_eq!(se1.is_subject, se2.is_subject);
        assert_ne!(se1.point, se2.point);
        assert_eq!(compare_segments(&se1, &se2), Ordering::Less);
        assert_eq!(compare_segments(&se2, &se1), Ordering::Greater);
    }
}
