use super::compare_segments::compare_segments;
use super::compute_fields::compute_fields;
use super::helper::Float;
use super::possible_intersection::possible_intersection;
use super::sweep_event::SweepEvent;
use super::Operation;
use crate::splay::SplaySet;
use geo_types::Rect;
use std::collections::BinaryHeap;
use std::rc::Rc;

#[cfg(feature="debug-booleanop")]
use super::sweep_event::JsonDebug;

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
        #[cfg(feature="debug-booleanop")] {
            println!("{{\"processEvent\": {}}}", event.to_json_debug());
        }
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
                #[cfg(feature="debug-booleanop")] {
                    println!("{{\"seNextEvent\": {}}}", next.to_json_debug());
                }
                if possible_intersection(&event, &next, event_queue) == 2 {
                    println!("Intersection with next");
                    compute_fields(&event, maybe_prev, operation);
                    //compute_fields(&event, Some(next), operation);
                    compute_fields(&next, Some(&event), operation);
                }
            }

            if let Some(prev) = maybe_prev {
                #[cfg(feature="debug-booleanop")] {
                    println!("{{\"sePrevEvent\": {}}}", prev.to_json_debug());
                }
                if possible_intersection(&prev, &event, event_queue) == 2 {
                    let maybe_prev_prev = sweep_line.prev(&prev);
                    //println!("Intersection with prev");
                    compute_fields(&prev, maybe_prev_prev, operation);
                    compute_fields(&event, Some(prev), operation);
                }
            }
        } else if let Some(other_event) = event.get_other_event() {
            //println!("sweep_line.contains(&other_event) {}", sweep_line.contains(&other_event));
            if sweep_line.contains(&other_event) {
                let maybe_prev = sweep_line.prev(&other_event).cloned();
                let maybe_next = sweep_line.next(&other_event).cloned();

                if let (Some(prev), Some(next)) = (maybe_prev, maybe_next) {
                    #[cfg(feature="debug-booleanop")] {
                        println!("Possible post intersection");
                        println!("{{\"sePostNextEvent\": {}}}", next.to_json_debug());
                        println!("{{\"sePostPrevEvent\": {}}}", prev.to_json_debug());
                    }
                    possible_intersection(&prev, &next, event_queue);
                }

                #[cfg(feature="debug-booleanop")] {
                    println!("{{\"removing\": {}}}", other_event.to_json_debug());
                }
                sweep_line.remove(&other_event);
            }
        }

        //let s: Vec<String> = &sweep_line.into_iter().map(|e| e.to_json_debug()).collect();
        //let s = s.join(", ");
        //println!("{{\"sweepLineState\": {{{}}}}}", s);
    }

    sorted_events
}
