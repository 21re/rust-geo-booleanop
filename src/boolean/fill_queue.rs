use geo::{LineString, Polygon, Rect};
use num_traits::Float;
use std::collections::BinaryHeap;
use std::rc::{Rc, Weak};

use super::sweep_event::SweepEvent;
use super::Operation;

pub fn fill_queue<F>(
    subject: &[Polygon<F>],
    clipping: &[Polygon<F>],
    sbbox: &mut Rect<F>,
    cbbox: &mut Rect<F>,
    operation: Operation,
) -> BinaryHeap<Rc<SweepEvent<F>>>
where
    F: Float,
{
    let mut event_queue: BinaryHeap<Rc<SweepEvent<F>>> = BinaryHeap::new();
    let mut contour_id = 0u32;

    for polygon in subject {
        contour_id += 1;
        process_polygon(&polygon.exterior, true, contour_id, &mut event_queue, sbbox, true);
        for interior in &polygon.interiors {
            process_polygon(interior, true, contour_id, &mut event_queue, sbbox, false);
        }
    }

    for polygon in clipping {
        let exterior = operation != Operation::Difference;
        if exterior {
            contour_id += 1;
        }
        process_polygon(&polygon.exterior, false, contour_id, &mut event_queue, cbbox, exterior);
        for interior in &polygon.interiors {
            process_polygon(interior, false, contour_id, &mut event_queue, cbbox, false);
        }
    }

    event_queue
}

fn process_polygon<F>(
    contour_or_hole: &LineString<F>,
    is_subject: bool,
    contour_id: u32,
    event_queue: &mut BinaryHeap<Rc<SweepEvent<F>>>,
    bbox: &mut Rect<F>,
    is_exterior_ring: bool,
) where
    F: Float,
{
    for line in contour_or_hole.lines() {
        if line.start == line.end {
            continue; // skip collapsed edges
        }

        let mut e1 = SweepEvent::new(contour_id, line.start, false, Weak::new(), is_subject, is_exterior_ring);
        let mut e2 = SweepEvent::new(
            contour_id,
            line.end,
            false,
            Rc::downgrade(&e1),
            is_subject,
            is_exterior_ring,
        );
        e1.set_other_event(&e2);

        if e1 < e2 {
            e2.set_left(true)
        } else {
            e1.set_left(true)
        }

        bbox.min.x = bbox.min.x.min(line.start.x);
        bbox.min.y = bbox.min.y.min(line.start.y);
        bbox.max.x = bbox.max.x.max(line.start.x);
        bbox.max.y = bbox.max.y.max(line.start.y);

        event_queue.push(e1);
        event_queue.push(e2);
    }
}

#[cfg(test)]
mod test {
    use super::super::helper::test::fixture_shapes;
    use super::super::sweep_event::SweepEvent;
    use super::*;
    use geo::{Coordinate, Rect};
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;
    use std::rc::{Rc, Weak};

    fn make_simple(x: f64, y: f64, is_subject: bool) -> Rc<SweepEvent<f64>> {
        SweepEvent::new(0, Coordinate { x, y }, false, Weak::new(), is_subject, true)
    }

    fn check_order_in_queue(first: Rc<SweepEvent<f64>>, second: Rc<SweepEvent<f64>>) {
        let mut queue: BinaryHeap<Rc<SweepEvent<f64>>> = BinaryHeap::new();

        assert_eq!(first.cmp(&second), Ordering::Greater);
        assert_eq!(second.cmp(&first), Ordering::Less);
        {
            queue.push(first.clone());
            queue.push(second.clone());

            let p1 = queue.pop().unwrap();
            let p2 = queue.pop().unwrap();

            assert!(Rc::ptr_eq(&first, &p1));
            assert!(Rc::ptr_eq(&second, &p2));
        }
        {
            queue.push(second.clone());
            queue.push(first.clone());

            let p1 = queue.pop().unwrap();
            let p2 = queue.pop().unwrap();

            assert!(Rc::ptr_eq(&first, &p1));
            assert!(Rc::ptr_eq(&second, &p2));
        }
    }

    #[test]
    fn test_least_by_x() {
        check_order_in_queue(make_simple(0.0, 0.0, false), make_simple(0.5, 0.5, false))
    }

    #[test]
    fn test_least_by_y() {
        check_order_in_queue(make_simple(0.0, 0.0, false), make_simple(0.0, 0.5, false))
    }

    #[test]
    fn test_least_left() {
        let e1 = make_simple(0.0, 0.0, false);
        e1.set_left(true);
        let e2 = make_simple(0.0, 0.0, false);
        e2.set_left(false);

        check_order_in_queue(e2, e1)
    }

    #[test]
    fn test_shared_edge_not_colinear() {
        let other_e1 = make_simple(1.0, 1.0, false);
        let e1 = make_simple(0.0, 0.0, false);
        e1.set_other_event(&other_e1);
        e1.set_left(true);
        let other_e2 = make_simple(2.0, 3.0, false);
        let e2 = make_simple(0.0, 0.0, false);
        e2.set_other_event(&other_e2);
        e2.set_left(true);

        check_order_in_queue(e1, e2)
    }

    #[test]
    fn test_collinear_edges() {
        let other_e1 = make_simple(1.0, 1.0, true);
        let e1 = make_simple(0.0, 0.0, true);
        e1.set_other_event(&other_e1);
        e1.set_left(true);
        let other_e2 = make_simple(2.0, 2.0, false);
        let e2 = make_simple(0.0, 0.0, false);
        e2.set_other_event(&other_e2);
        e2.set_left(true);

        check_order_in_queue(e1, e2)
    }

    #[test]
    fn test_two_polygons() {
        let (s, c) = fixture_shapes("two_shapes.geojson");
        let mut sbbox = Rect {
            min: Coordinate {
                x: f64::infinity(),
                y: f64::infinity(),
            },
            max: Coordinate {
                x: f64::neg_infinity(),
                y: f64::neg_infinity(),
            },
        };
        let mut cbbox = sbbox;
        let mut q = fill_queue(&[s], &[c], &mut sbbox, &mut cbbox, Operation::Intersection);

        let mut sorted = Vec::new();
        let mut keep_ref = Vec::new();
        while let Some(e) = q.pop() {
            keep_ref.push(e.clone());
            sorted.push((
                e.is_left(),
                e.point.x,
                e.point.y,
                e.get_other_event().unwrap().point.x,
                e.get_other_event().unwrap().point.y,
            ));
        }

        assert_eq!(
            sorted,
            vec![
                (true, 16.0, 282.0, 153.0, 203.5),
                (true, 16.0, 282.0, 298.0, 359.0),
                (true, 56.0, 181.0, 108.5, 120.0),
                (true, 56.0, 181.0, 153.0, 294.5),
                (false, 108.5, 120.0, 56.0, 181.0),
                (true, 108.5, 120.0, 241.5, 229.5),
                (false, 153.0, 203.5, 16.0, 282.0),
                (true, 153.0, 203.5, 298.0, 359.0),
                (false, 153.0, 294.5, 56.0, 181.0),
                (true, 153.0, 294.5, 241.5, 229.5),
                (false, 241.5, 229.5, 108.5, 120.0),
                (false, 241.5, 229.5, 153.0, 294.5),
                (false, 298.0, 359.0, 153.0, 203.5),
                (false, 298.0, 359.0, 16.0, 282.0),
            ]
        );
    }

    #[test]
    fn test_fill_event_queue() {
        fn xy<T: Into<f64>>(x: T, y: T) -> Coordinate<f64> {
            Coordinate {
                x: x.into(),
                y: y.into(),
            }
        }

        let (s, c) = fixture_shapes("two_triangles.geojson");
        let mut sbbox = Rect {
            min: xy(f64::infinity(), f64::infinity()),
            max: xy(f64::neg_infinity(), f64::neg_infinity()),
        };
        let mut cbbox = sbbox;
        let mut q = fill_queue(&[s], &[c], &mut sbbox, &mut cbbox, Operation::Intersection);

        assert_eq!(
            sbbox,
            Rect {
                min: xy(20.0, -113.5),
                max: xy(226.5, 74.0)
            },
        );
        assert_eq!(
            cbbox,
            Rect {
                min: xy(54.5, -198.0),
                max: xy(239.5, 33.5)
            },
        );

        let mut sorted = Vec::new();
        let mut keep_ref = Vec::new();
        while let Some(e) = q.pop() {
            keep_ref.push(e.clone());
            sorted.push((
                e.point.x,
                e.point.y,
                e.is_left(),
                e.get_other_event().unwrap().point.x,
                e.get_other_event().unwrap().point.y,
                e.get_other_event().unwrap().is_left(),
            ));
        }
        assert_eq!(
            sorted,
            vec![
                (20.0, -23.5, true, 226.5, -113.5, false),
                (20.0, -23.5, true, 170.0, 74.0, false),
                (54.5, -170.5, true, 239.5, -198.0, false),
                (54.5, -170.5, true, 140.5, 33.5, false),
                (140.5, 33.5, false, 54.5, -170.5, true),
                (140.5, 33.5, true, 239.5, -198.0, false),
                (170.0, 74.0, false, 20.0, -23.5, true),
                (170.0, 74.0, true, 226.5, -113.5, false),
                (226.5, -113.5, false, 20.0, -23.5, true),
                (226.5, -113.5, false, 170.0, 74.0, true),
                (239.5, -198.0, false, 54.5, -170.5, true),
                (239.5, -198.0, false, 140.5, 33.5, true)
            ]
        );
    }
}
