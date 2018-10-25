use super::sweep_event::SweepEvent;
use super::Operation;
use geo::{LineString, Polygon};
use num_traits::Float;
use std::collections::HashSet;
use std::rc::Rc;

fn order_events<F>(sorted_events: Vec<Rc<SweepEvent<F>>>) -> Vec<Rc<SweepEvent<F>>>
where
    F: Float,
{
    let mut result_events: Vec<Rc<SweepEvent<F>>> = Vec::new();

    for event in &sorted_events {
        if (event.is_left() && event.is_in_result())
            || (!event.is_left() && event.get_other_event().map(|o| o.is_in_result()).unwrap_or(false))
        {
            result_events.push(event.clone());
        }
    }

    let mut sorted = false;
    while !sorted {
        sorted = true;
        for i in 1..result_events.len() {
            if result_events[i - 1] < result_events[i] {
                result_events.swap(i - 1, i);
                sorted = false;
            }
        }
    }

    for (pos, event) in result_events.iter().enumerate() {
        event.set_pos(pos as i32)
    }

    for i in 0..result_events.len() {
        let event = result_events[i].clone();
        if !event.is_left() {
            if let Some(other) = event.get_other_event() {
                let tmp = event.get_pos();
                event.set_pos(other.get_pos());
                other.set_pos(tmp);
            }
        }
    }

    result_events
}

fn next_pos<F>(pos: i32, result_events: &Vec<Rc<SweepEvent<F>>>, processed: &mut HashSet<i32>, orig_index: i32) -> i32
where
    F: Float,
{
    let p = result_events[pos as usize].point;
    let mut new_pos = pos + 1;
    let length = result_events.len() as i32;
    let mut p1 = if new_pos < length {
        result_events[new_pos as usize].point
    } else {
        p
    };

    while new_pos < length && p == p1 {
        if !processed.contains(&new_pos) {
            return new_pos;
        } else {
            new_pos += 1;
        }
        if new_pos < length {
            p1 = result_events[new_pos as usize].point;
        }
    }

    new_pos = pos - 1;

    while processed.contains(&new_pos) && new_pos >= orig_index as i32 {
        new_pos -= 1;
    }
    new_pos
}

fn append_countour<F>(polygon: &mut Polygon<F>, contour: LineString<F>)
where
    F: Float,
{
    if polygon.exterior.0.len() > 0 {
        polygon.interiors.push(contour)
    } else {
        polygon.exterior = contour
    }
}

pub fn connect_edges<F>(sorted_events: Vec<Rc<SweepEvent<F>>>, operation: Operation) -> Vec<Polygon<F>>
where
    F: Float,
{
    let result_events = order_events(sorted_events);

    let mut result: Vec<Polygon<F>> = Vec::new();
    let mut processed: HashSet<i32> = HashSet::new();

    for i in 0..(result_events.len() as i32) {
        if processed.contains(&i) {
            continue;
        }
        let mut contour = LineString::<F>(Vec::new());
        let mut pos = i;
        let initial = result_events[i as usize].point;

        contour.0.push(initial);

        while pos >= i {
            processed.insert(pos);

            pos = result_events[pos as usize].get_pos();
            processed.insert(pos);
            contour.0.push(result_events[pos as usize].point);
            pos = next_pos(pos, &result_events, &mut processed, i);
        }

        if !result_events[i as usize].is_exterior_ring {
            if operation == Operation::Difference && !result_events[i as usize].is_subject && result.len() == 0 {
                result.push(Polygon::new(contour, Vec::new()));
            } else if result.len() == 0 {
                result.push(Polygon::new(contour, Vec::new()));
            } else {
                append_countour(
                    result.last_mut().expect("Result must not be empty at this point"),
                    contour,
                );
            }
        } else if operation == Operation::Difference && !result_events[i as usize].is_subject && result.len() > 1 {
            append_countour(
                result.last_mut().expect("Result must not be empty at this point"),
                contour,
            );
        } else {
            result.push(Polygon::new(contour, Vec::new()));
        }
    }

    result
}
