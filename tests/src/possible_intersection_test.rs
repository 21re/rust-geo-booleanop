use super::helper::fixture_shapes;
use geo::Coordinate;
use geo_booleanop::boolean::compare_segments::compare_segments;
use geo_booleanop::boolean::fill_queue::fill_queue;
use geo_booleanop::boolean::possible_intersection::possible_intersection;
use geo_booleanop::boolean::subdivide_segments::subdivide;
use geo_booleanop::boolean::sweep_event::SweepEvent;
use geo_booleanop::boolean::Operation;
use geo_booleanop::boolean::BoundingBox;
use geo_booleanop::splay::SplaySet;
use num_traits::Float;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::{Rc, Weak};

fn make_simple(a: Coordinate<f64>, b: Coordinate<f64>, is_subject: bool) -> (Rc<SweepEvent<f64>>, Rc<SweepEvent<f64>>) {
    let other = SweepEvent::new_rc(0, b, false, Weak::new(), is_subject, true);
    let event = SweepEvent::new_rc(0, a, true, Rc::downgrade(&other), is_subject, true);

    (event, other)
}

#[test]
fn test_possible_intersection() {
    let (s, c) = fixture_shapes("two_shapes.geojson");
    let mut q: BinaryHeap<Rc<SweepEvent<f64>>> = BinaryHeap::new();

    let (se1, _other1) = make_simple(s.exterior().0[3], s.exterior().0[2], true);
    let (se2, _other2) = make_simple(c.exterior().0[0], c.exterior().0[1], false);

    assert_eq!(possible_intersection(&se1, &se2, &mut q), 1);
    assert_eq!(q.len(), 4);

    let mut e = q.pop().unwrap();
    assert_eq!(
        e.point,
        Coordinate {
            x: 100.79403384562251,
            y: 233.41363754101192
        }
    );
    assert_eq!(e.get_other_event().unwrap().point, Coordinate { x: 56.0, y: 181.0 });

    e = q.pop().unwrap();
    assert_eq!(
        e.point,
        Coordinate {
            x: 100.79403384562251,
            y: 233.41363754101192
        }
    );
    assert_eq!(e.get_other_event().unwrap().point, Coordinate { x: 16.0, y: 282.0 });

    e = q.pop().unwrap();
    assert_eq!(
        e.point,
        Coordinate {
            x: 100.79403384562251,
            y: 233.41363754101192
        }
    );
    assert_eq!(e.get_other_event().unwrap().point, Coordinate { x: 153.0, y: 203.5 });

    e = q.pop().unwrap();
    assert_eq!(
        e.point,
        Coordinate {
            x: 100.79403384562251,
            y: 233.41363754101192
        }
    );
    assert_eq!(e.get_other_event().unwrap().point, Coordinate { x: 153.0, y: 294.5 });
}

#[test]
fn test_on_two_polygons() {
    let (s, c) = fixture_shapes("two_shapes.geojson");
    let mut sbbox = BoundingBox {
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

    let p0 = Coordinate { x: 16.0, y: 282.0 };
    let p1 = Coordinate { x: 298.0, y: 359.0 };
    let p2 = Coordinate { x: 156.0, y: 203.5 };

    let te = SweepEvent::new_rc(0, p0, true, Weak::new(), true, true);
    let te2 = SweepEvent::new_rc(0, p1, false, Rc::downgrade(&te), false, true);
    te.set_other_event(&te2);

    let te3 = SweepEvent::new_rc(0, p0, true, Weak::new(), true, true);
    let te4 = SweepEvent::new_rc(0, p2, true, Rc::downgrade(&te3), false, true);
    te3.set_other_event(&te4);

    let mut tr = SplaySet::new(compare_segments);

    tr.insert(te.clone());
    tr.insert(te3.clone());

    assert!(Rc::ptr_eq(&te, tr.find(&te).unwrap()));
    assert!(Rc::ptr_eq(&te3, tr.find(&te3).unwrap()));

    assert_eq!(compare_segments(&te, &te3), Ordering::Greater);
    assert_eq!(compare_segments(&te3, &te), Ordering::Less);

    let segments = subdivide(&mut q, &sbbox, &cbbox, Operation::Intersection);

    let left_segments = segments.iter().filter(|s| s.is_left()).cloned().collect::<Vec<_>>();

    assert_eq!(left_segments.len(), 11);

    let e = Coordinate::<f64> { x: 16.0, y: 282.0 };
    let i = Coordinate::<f64> {
        x: 100.79403384562252,
        y: 233.41363754101192,
    };
    let g = Coordinate::<f64> { x: 298.0, y: 359.0 };
    let c = Coordinate::<f64> { x: 153.0, y: 294.5 };
    let j = Coordinate::<f64> {
        x: 203.36313843035356,
        y: 257.5101243166895,
    };
    let f = Coordinate::<f64> { x: 153.0, y: 203.5 };
    let d = Coordinate::<f64> { x: 56.0, y: 181.0 };
    let a = Coordinate::<f64> { x: 108.5, y: 120.0 };
    let b = Coordinate::<f64> { x: 241.5, y: 229.5 };

    let intervals = &[
        ("EI", e, i, false, true, false),
        ("IF", i, f, false, false, true),
        ("FJ", f, j, false, false, true),
        ("JG", j, g, false, true, false),
        ("EG", e, g, true, true, false),
        ("DA", d, a, false, true, false),
        ("AB", a, b, false, true, false),
        ("JB", j, b, true, true, false),
        ("CJ", c, j, true, false, true),
        ("IC", i, c, true, false, true),
        ("DC", d, i, true, true, false),
    ];

    for (interval, a, b, in_out, other_in_out, in_result) in intervals {
        let mut found = false;

        for segment in &left_segments {
            if segment.point == *a
                && segment.get_other_event().unwrap().point == *b
                && segment.is_in_out() == *in_out
                && segment.is_other_in_out() == *other_in_out
                && segment.is_in_result() == *in_result
            {
                found = true;
                break;
            }
        }
        if !found {
            panic!("interval {} not found", interval);
        }
    }
}
