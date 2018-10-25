use super::compare_segments::compare_segments;
use super::compute_fields::compute_fields;
use super::possible_intersection::possible_intersection;
use super::sweep_event::SweepEvent;
use super::Operation;
use geo::Rect;
use num_traits::Float;
use splay::SplaySet;
use std::collections::BinaryHeap;
use std::rc::Rc;

pub fn subdivide<F>(
    event_queue: &mut BinaryHeap<Rc<SweepEvent<F>>>,
    sbbox: &Rect<F>,
    cbbox: &Rect<F>,
    operation: Operation,
) -> Vec<Rc<SweepEvent<F>>>
where
    F: Float,
{
    let mut sweep_line = SplaySet::<Rc<SweepEvent<F>>, _>::new(compare_segments);
    let mut sorted_events: Vec<Rc<SweepEvent<F>>> = Vec::new();
    let rightbound = sbbox.max.x.min(cbbox.max.x);

    while let Some(event) = event_queue.pop() {
        sorted_events.push(event.clone());

        if operation == Operation::Intersection && event.point.x > rightbound
            || operation == Operation::Difference && event.point.x > sbbox.max.x
        {
            break;
        }

        if event.is_left() {
            sweep_line.insert(event.clone());

            let maybe_prev = sweep_line.prev(&event);
            let maybe_next = sweep_line.next(&event);

            compute_fields(&event, maybe_prev, operation);

            if let Some(next) = maybe_next {
                if possible_intersection(event.clone(), next.clone(), event_queue) == 2 {
                    compute_fields(&event, maybe_prev, operation);
                    compute_fields(&event, Some(next), operation);
                }
            }

            if let Some(prev) = maybe_prev {
                if possible_intersection(prev.clone(), event.clone(), event_queue) == 2 {
                    let maybe_prev_prev = sweep_line.prev(&prev);

                    compute_fields(&prev, maybe_prev_prev, operation);
                    compute_fields(&event, Some(prev), operation);
                }
            }
        } else if let Some(other_event) = event.get_other_event() {
            if sweep_line.contains(&other_event) {
                let maybe_prev = sweep_line.prev(&other_event).cloned();
                let maybe_next = sweep_line.next(&other_event).cloned();

                if let (Some(prev), Some(next)) = (maybe_prev, maybe_next) {
                    possible_intersection(prev.clone(), next.clone(), event_queue);
                }

                sweep_line.remove(&other_event);
            }
        }
    }

    sorted_events
}

#[cfg(test)]
mod test {
    use super::super::compare_segments::compare_segments;
    use super::super::helper::test::fixture_shapes;
    use super::super::sweep_event::SweepEvent;
    use splay::SplaySet;
    use std::rc::{Rc, Weak};

    #[test]
    fn test_sweep_line() {
        let (s, c) = fixture_shapes("two_triangles.geojson");

        let ef_other = SweepEvent::new(0, s.exterior.0[2], false, Weak::new(), true, true);
        let ef = SweepEvent::new(0, s.exterior.0[0], true, Rc::downgrade(&ef_other), true, true);
        let eg_other = SweepEvent::new(0, s.exterior.0[1], false, Weak::new(), false, true);
        let eg = SweepEvent::new(0, s.exterior.0[0], true, Rc::downgrade(&eg_other), false, true);

        let mut tree = SplaySet::new(compare_segments);
        tree.insert(ef.clone());
        tree.insert(eg.clone());

        assert!(Rc::ptr_eq(tree.find(&ef).unwrap(), &ef));
        assert!(Rc::ptr_eq(tree.min().unwrap(), &ef));
        assert!(Rc::ptr_eq(tree.max().unwrap(), &eg));
        assert!(Rc::ptr_eq(tree.next(&ef).unwrap(), &eg));
        assert!(Rc::ptr_eq(tree.prev(&eg).unwrap(), &ef));

        let da_other = SweepEvent::new(0, c.exterior.0[2], false, Weak::new(), true, true);
        let da = SweepEvent::new(0, c.exterior.0[0], true, Rc::downgrade(&da_other), true, true);
        let dc_other = SweepEvent::new(0, c.exterior.0[1], false, Weak::new(), false, true);
        let dc = SweepEvent::new(0, c.exterior.0[0], true, Rc::downgrade(&dc_other), false, true);

        tree.insert(da.clone());
        tree.insert(dc.clone());

        assert!(Rc::ptr_eq(tree.min().unwrap(), &da));
        assert!(Rc::ptr_eq(tree.next(&da).unwrap(), &dc));
        assert!(Rc::ptr_eq(tree.next(&dc).unwrap(), &ef));
        assert!(Rc::ptr_eq(tree.next(&ef).unwrap(), &eg));
    }
}
