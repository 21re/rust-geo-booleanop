use super::helper::fixture_shapes;
use geo_booleanop::boolean::compare_segments::compare_segments;
use geo_booleanop::boolean::sweep_event::SweepEvent;
use geo_booleanop::splay::SplaySet;
use std::rc::{Rc, Weak};

#[test]
fn test_sweep_line() {
    let (s, c) = fixture_shapes("two_triangles.geojson");

    let ef_other = SweepEvent::new_rc(0, s.exterior().0[2], false, Weak::new(), true, true);
    let ef = SweepEvent::new_rc(0, s.exterior().0[0], true, Rc::downgrade(&ef_other), true, true);
    let eg_other = SweepEvent::new_rc(0, s.exterior().0[1], false, Weak::new(), false, true);
    let eg = SweepEvent::new_rc(0, s.exterior().0[0], true, Rc::downgrade(&eg_other), false, true);

    let mut tree = SplaySet::new(compare_segments);
    tree.insert(ef.clone());
    tree.insert(eg.clone());

    assert!(Rc::ptr_eq(tree.find(&ef).unwrap(), &ef));
    assert!(Rc::ptr_eq(tree.min().unwrap(), &ef));
    assert!(Rc::ptr_eq(tree.max().unwrap(), &eg));
    assert!(Rc::ptr_eq(tree.next(&ef).unwrap(), &eg));
    assert!(Rc::ptr_eq(tree.prev(&eg).unwrap(), &ef));

    let da_other = SweepEvent::new_rc(0, c.exterior().0[2], false, Weak::new(), true, true);
    let da = SweepEvent::new_rc(0, c.exterior().0[0], true, Rc::downgrade(&da_other), true, true);
    let dc_other = SweepEvent::new_rc(0, c.exterior().0[1], false, Weak::new(), false, true);
    let dc = SweepEvent::new_rc(0, c.exterior().0[0], true, Rc::downgrade(&dc_other), false, true);

    tree.insert(da.clone());
    tree.insert(dc.clone());

    assert!(Rc::ptr_eq(tree.min().unwrap(), &da));
    assert!(Rc::ptr_eq(tree.next(&da).unwrap(), &dc));
    assert!(Rc::ptr_eq(tree.next(&dc).unwrap(), &ef));
    assert!(Rc::ptr_eq(tree.next(&ef).unwrap(), &eg));
}
