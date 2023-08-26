use super::divide_segment::divide_segment;
use super::helper::Float;
use super::segment_intersection::{intersection, LineIntersection};
use super::sweep_event::{EdgeType, SweepEvent};
use std::collections::BinaryHeap;
use std::rc::Rc;

pub fn possible_intersection<F>(
    se1: &Rc<SweepEvent<F>>,
    se2: &Rc<SweepEvent<F>>,
    queue: &mut BinaryHeap<Rc<SweepEvent<F>>>,
) -> u8
where
    F: Float,
{
    let (other1, other2) = match (se1.get_other_event(), se2.get_other_event()) {
        (Some(other1), Some(other2)) => (other1, other2),
        _ => return 0,
    };

    let inter = intersection(se1.point, other1.point, se2.point, other2.point);

    #[cfg(feature = "debug-booleanop")]
    match inter {
        LineIntersection::Point(inter) => {
            println!("{{\"intersection\": [{}, {}]}}", inter.x, inter.y);
        }
        LineIntersection::Overlap(p1, p2) => {
            println!(
                "{{\"overlap1\": [{}, {}], \"overlap2\": [{}, {}]}}",
                p1.x, p1.y, p2.x, p2.y
            );
        }
        _ => {}
    }

    match inter {
        LineIntersection::None => 0, // No intersection
        LineIntersection::Point(_) if se1.point == se2.point || other1.point == other2.point => {
            // The line segments intersect at either the left or right endpoint.
            // In this case we ignore the result of intersection computation for numerical
            // stability (the computed intersection can slightly deviate from the endpoints).
            // It may be tempting to make this check earlier to short-circuit the intersection
            // computation, but it may be tricky, because we still need to differentiate from
            // overlapping cases.
            0
        }
        LineIntersection::Point(inter) => {
            if se1.point != inter && other1.point != inter {
                divide_segment(se1, inter, queue);
            }
            if se2.point != inter && other2.point != inter {
                divide_segment(se2, inter, queue);
            }
            1
        }
        LineIntersection::Overlap(_, _) if se1.is_subject == se2.is_subject => 0, // The line segments associated to se1 and se2 overlap
        LineIntersection::Overlap(_, _) => {
            let mut events = Vec::new();
            let mut left_coincide = false;
            let mut right_coincide = false;

            if se1.point == se2.point {
                left_coincide = true
            } else if se1 < se2 {
                events.push((se2.clone(), other2.clone()));
                events.push((se1.clone(), other1.clone()));
            } else {
                events.push((se1.clone(), other1.clone()));
                events.push((se2.clone(), other2.clone()));
            }

            if other1.point == other2.point {
                right_coincide = true
            } else if other1 < other2 {
                events.push((other2, se2.clone()));
                events.push((other1, se1.clone()));
            } else {
                events.push((other1, se1.clone()));
                events.push((other2, se2.clone()));
            }

            if left_coincide {
                // both line segments are equal or share the left endpoint
                se2.set_edge_type(EdgeType::NonContributing);
                if se1.is_in_out() == se2.is_in_out() {
                    se1.set_edge_type(EdgeType::SameTransition)
                } else {
                    se1.set_edge_type(EdgeType::DifferentTransition)
                }

                if left_coincide && !right_coincide {
                    divide_segment(&events[1].1, events[0].0.point, queue)
                }
                return 2;
            }

            if right_coincide {
                // the line segments share the right endpoint
                divide_segment(&events[0].0, events[1].0.point, queue);
                return 3;
            }

            if !Rc::ptr_eq(&events[0].0, &events[3].1) {
                // no line segment includes totally the other one
                divide_segment(&events[0].0, events[1].0.point, queue);
                divide_segment(&events[1].0, events[2].0.point, queue);
                return 3;
            }

            // one line segment includes the other one
            // TODO: write this in a non-panicking way. Note that we must not access the "other event"
            // via events[3].1 because that is only a static reference, and the first divide segment
            // internally modifies the other event point (we must access the updated other event).
            // Probably the best solution is to introduce explicit return types for divide_segment.
            divide_segment(&events[0].0, events[1].0.point, queue);
            divide_segment(&events[3].0.get_other_event().unwrap(), events[2].0.point, queue);

            3
        }
    }
}
