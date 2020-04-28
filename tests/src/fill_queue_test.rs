use super::helper::fixture_shapes;
use geo::{Coordinate, Rect};
use geo_booleanop::boolean::fill_queue::fill_queue;
use geo_booleanop::boolean::Operation;
use num_traits::Float;

use super::helper::xy;

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
