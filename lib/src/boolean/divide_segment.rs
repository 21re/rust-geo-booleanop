use super::sweep_event::SweepEvent;
use geo_types::Coordinate;
use num_traits::Float;
use std::collections::BinaryHeap;
use std::rc::Rc;

pub fn divide_segment<F>(se: &Rc<SweepEvent<F>>, inter: Coordinate<F>, queue: &mut BinaryHeap<Rc<SweepEvent<F>>>)
where
    F: Float,
{
    let other_event = match se.get_other_event() {
        Some(other_event) => other_event,
        None => return,
    };

    let r = SweepEvent::new(se.contour_id, inter, false, Rc::downgrade(&se), se.is_subject, true);
    let l = SweepEvent::new(
        se.contour_id,
        inter,
        true,
        Rc::downgrade(&other_event),
        se.is_subject,
        true,
    );

    if l < other_event {
        se.set_left(true);
        l.set_left(false);
    }

    other_event.set_other_event(&l);
    se.set_other_event(&r);

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
        let other = SweepEvent::new(
            0,
            Coordinate { x: other_x, y: other_y },
            false,
            Weak::new(),
            is_subject,
            true,
        );
        let event = SweepEvent::new(0, Coordinate { x, y }, true, Rc::downgrade(&other), is_subject, true);

        (event, other)
    }

    #[test]
    fn devide_segments() {
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
