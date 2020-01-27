use super::sweep_event::{SweepEvent, ResultTransition};
use super::Operation;
use geo_types::{LineString, Polygon, Coordinate};
use num_traits::Float;
use std::collections::HashSet;
use std::collections::HashMap;
use std::rc::Rc;

fn order_events<F>(sorted_events: &[Rc<SweepEvent<F>>]) -> Vec<Rc<SweepEvent<F>>>
where
    F: Float + std::fmt::Debug,
{
    let mut result_events: Vec<Rc<SweepEvent<F>>> = Vec::new();

    for event in sorted_events {
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

    for event in &result_events {
        if !event.is_left() {
            if let Some(other) = event.get_other_event() {
                let tmp = event.get_pos();
                event.set_pos(other.get_pos());
                other.set_pos(tmp);
            }
        }
    }

    for r in &result_events {
        println!("{:?}", r);
        debug_assert!(r.get_other_event().is_some());
    }

    for (i, r) in result_events.iter().enumerate() {
        println!("pos {:3} linked to {:3}    {}    {:?} => {:?}",
            i,
            r.get_pos(),
            if r.is_left() { "L" } else { "R" },
            r.point,
            r.get_other_event().map(|o| o.point).unwrap(),
        );
    }

    result_events
}


fn next_pos<F>(pos: i32, result_events: &[Rc<SweepEvent<F>>], processed: &HashSet<i32>, orig_index: i32) -> i32
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

    while processed.contains(&new_pos) {
        new_pos -= 1;
    }
    new_pos
}


pub struct Contour<F>
where
    F: Float
{
    pub points: Vec<Coordinate<F>>,
    pub hole_ids: Vec<i32>,
    pub is_external: bool,
}


use std::fs::File;
use std::io::Write;
fn debug_print_results<F>(events: &[Rc<SweepEvent<F>>])
where
    F: Float + std::fmt::Debug,
{
    let mut writer = File::create("debug.csv").unwrap();
    writeln!(&mut writer,
        "index;x;y;other_x;other_y;lr;result_transition;in_out;other_in_out;is_subject;is_exterior_ring;prev_in_result"
    ).expect("Failed to write to file");
    for (i, evt) in events.iter().enumerate() {
        writeln!(&mut writer, "{i};{x:?};{y:?};{other_x:?};{other_y:?};{lr};{transition:?};{in_out};{other_in_out};{subject};{exterior_ring};{prev_in_result:?}",
            i=i,
            x=evt.point.x,
            y=evt.point.y,
            other_x=evt.get_other_event().unwrap().point.x,
            other_y=evt.get_other_event().unwrap().point.y,
            lr=if evt.is_left() { "L" } else { "R" },
            transition=evt.get_result_transition(),
            in_out=evt.is_in_out(),
            other_in_out=evt.is_other_in_out(),
            subject=evt.is_subject,
            exterior_ring=evt.is_exterior_ring,
            prev_in_result=evt.get_prev_in_result().map(|o| format!("{:?}", o.point)),
        ).expect("Failed to write to file");
    }
}


pub fn connect_edges<F>(sorted_events: &[Rc<SweepEvent<F>>]) -> Vec<Contour<F>>
where
    F: Float + std::fmt::Debug,
{
    let result_events = order_events(sorted_events);
    debug_print_results(&result_events);

    //let mut result: Vec<Polygon<F>> = Vec::new();
    let mut result: Vec<Contour<F>> = Vec::new();
    let mut processed: HashSet<i32> = HashSet::new();

    let mut depth: HashMap<i32, i32> = HashMap::new();
    let mut hole_of: HashMap<i32, i32> = HashMap::new();

    for i in 0..(result_events.len() as i32) {
        if processed.contains(&i) {
            continue;
        }
        let mut contour = Contour{
            points: Vec::new(),
            hole_ids: Vec::new(),
            is_external: true,
        };

        let contour_id = result.len() as i32;
        println!("\n *** Adding contour id {}", contour_id);
        depth.insert(contour_id, 0);
        hole_of.insert(contour_id, -1);

        if let Some(prev_in_result) = result_events[i as usize].get_prev_in_result() {
            let lower_contour_id = prev_in_result.get_output_contour_id();
            println!("Inferring information from lower_contour_id = {} with result transition = {:?}", lower_contour_id, prev_in_result.get_result_transition());
            println!("{:?}", prev_in_result.point);
            println!("{:?}", prev_in_result.get_other_event().unwrap().point);
            if prev_in_result.get_result_transition() == ResultTransition::OutIn {
                // We are inside, let's check if the thing below us is an exterior contour or just
                // another hole.
                if result[lower_contour_id as usize].is_external {
                    result[lower_contour_id as usize].hole_ids.push(contour_id);
                    hole_of.insert(contour_id, lower_contour_id);
                    depth.insert(contour_id, depth[&lower_contour_id] + 1);
                    contour.is_external = false;
                    println!("Marking contour as hole of {} with depth {}", lower_contour_id, depth[&contour_id]);
                } else {
                    let parent_contour_id = hole_of[&lower_contour_id];
                    result[parent_contour_id as usize].hole_ids.push(contour_id);
                    hole_of.insert(contour_id, parent_contour_id);
                    depth.insert(contour_id, depth[&lower_contour_id]);
                    contour.is_external = false;
                    println!("Transitively marking contour as hole of {} via {} with depth {}", parent_contour_id, lower_contour_id, depth[&contour_id]);
                }
            } else {
                contour.is_external = true;
                println!("Keeping contour as external");
            }
        }

        let mut pos = i;
        let initial = result_events[i as usize].point;

        contour.points.push(initial);

        while pos >= 0 && result_events[pos as usize].get_other_event().unwrap().point != initial {
            println!("pos = {}   {}   {:?} => {:?}",
                pos,
                if result_events[pos as usize].is_left() { "L" } else { "R" },
                result_events[pos as usize].point,
                result_events[pos as usize].get_other_event().unwrap().point,
            );
            processed.insert(pos);
            result_events[pos as usize].set_output_contour_id(contour_id);

            pos = result_events[pos as usize].get_pos();
            println!("Jumped to: {}", pos);

            processed.insert(pos);
            result_events[pos as usize].set_output_contour_id(contour_id);

            contour.points.push(result_events[pos as usize].point);
            pos = next_pos(pos, &result_events, &processed, i);
            println!("Next pos: {}", pos);
        }

        if pos != -1 {
            processed.insert(pos);
            result_events[pos as usize].set_output_contour_id(contour_id);
            pos = result_events[pos as usize].get_pos();
            processed.insert(pos);
            result_events[pos as usize].set_output_contour_id(contour_id);
        }

        result.push(contour);
    }

    result
}
