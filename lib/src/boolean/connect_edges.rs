use super::sweep_event::{SweepEvent, ResultTransition};
use geo_types::{Coordinate};
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

    // Populate `other_pos` by initializing with index and swapping with other event.
    for (pos, event) in result_events.iter().enumerate() {
        event.set_other_pos(pos as i32)
    }
    for event in &result_events {
        if event.is_left() {
            if let Some(other) = event.get_other_event() {
                let (a, b) = (event.get_other_pos(), other.get_other_pos());
                event.set_other_pos(b);
                other.set_other_pos(a);
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
            r.get_other_pos(),
            if r.is_left() { "L" } else { "R" },
            r.point,
            r.get_other_event().map(|o| o.point).unwrap(),
        );
    }

    result_events
}


fn next_pos<F>(pos: i32, result_events: &[Rc<SweepEvent<F>>], processed: &HashSet<i32>, orig_pos: i32) -> i32
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

    while processed.contains(&new_pos) && new_pos > orig_pos {
        new_pos -= 1;
    }
    new_pos
}


pub struct Contour<F>
where
    F: Float
{
    /// Raw coordinates of contour
    pub points: Vec<Coordinate<F>>,
    /// Contour IDs of holes if any.
    pub hole_ids: Vec<i32>,
    /// Contour ID of parent if this contour is a hole.
    pub hole_of: Option<i32>,
    /// Depth of the contour. Since the geo data structures don't store depth information,
    /// this field is not strictly necessary to compute. But it is very cheap to compute,
    /// so we can add it and see if it has relevance in the future.
    pub depth: i32,
}

impl<F> Contour<F>
where
    F: Float
{
    pub fn new(hole_of: Option<i32>, depth: i32) -> Contour<F> {
        Contour {
            points: Vec::new(),
            hole_ids: Vec::new(),
            hole_of: hole_of,
            depth: depth,
        }
    }

    /// The semantics of `is_exterior` are in the sense of an exterior ring of a polygon
    /// in GeoJSON. This is not to be confused with "external contour" as used in the
    /// Martinez paper (which refers to contours that are not included in any of the
    /// other polygon contours). `is_exterior` is true for all outer contours, not just
    /// the outermost.
    pub fn is_exterior(&self) -> bool {
        self.hole_of.is_none()
    }
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


fn mark_as_processed<F>(processed: &mut HashSet<i32>, result_events: &[Rc<SweepEvent<F>>], pos: i32, contour_id: i32)
where
    F: Float + std::fmt::Debug,
{
    processed.insert(pos);
    result_events[pos as usize].set_output_contour_id(contour_id);
}


pub fn connect_edges<F>(sorted_events: &[Rc<SweepEvent<F>>]) -> Vec<Contour<F>>
where
    F: Float + std::fmt::Debug,
{
    let result_events = order_events(sorted_events);
    debug_print_results(&result_events);

    let mut result: Vec<Contour<F>> = Vec::new();
    let mut processed: HashSet<i32> = HashSet::new();

    for i in 0..(result_events.len() as i32) {
        if processed.contains(&i) {
            continue;
        }

        let contour_id = result.len() as i32;

        // This logic implements the 4 cases of parent contours from Fig. 4 in the Martinez paper.
        let mut contour = if let Some(prev_in_result) = result_events[i as usize].get_prev_in_result() {
            // Note that it is valid to query the "previous in result" for its output contour id,
            // because we must have already processed it (i.e., assigned an output contour id)
            // in an earlier iteration, otherwise it wouldn't be possible that it is "previous in
            // result".
            let lower_contour_id = prev_in_result.get_output_contour_id();
            if prev_in_result.get_result_transition() == ResultTransition::OutIn {
                // We are inside. Now we have to check if the thing below us is another hole or
                // an exterior contour.
                let lower_contour = &result[lower_contour_id as usize];
                if let Some(parent_contour_id) = lower_contour.hole_of {
                    // The lower contour is a hole => Connect the new contour as a hole to its parent,
                    // and use same depth.
                    result[parent_contour_id as usize].hole_ids.push(contour_id);
                    let hole_of = Some(parent_contour_id);
                    let depth = result[lower_contour_id as usize].depth;
                    Contour::new(hole_of, depth)
                } else {
                    // The lower contour is an exterior contour => Connect the new contour as a hole,
                    // and increment depth.
                    result[lower_contour_id as usize].hole_ids.push(contour_id);
                    let hole_of = Some(lower_contour_id);
                    let depth = result[lower_contour_id as usize].depth + 1;
                    Contour::new(hole_of, depth)
                }
            } else {
                // We are outside => this contour is an exterior contour of same depth.
                let depth = result[lower_contour_id as usize].depth;
                Contour::new(None, depth)
            }
        } else {
            // There is no lower/previous contour => this contour is an exterior contour of depth 0.
            Contour::new(None, 0)
        };

        let orig_pos = i; // Alias just for clarity
        let mut pos = i;

        let initial = result_events[pos as usize].point;
        contour.points.push(initial);

        loop {
            // Loop clarifications:
            // - An iteration has two kinds of `pos` advancements:
            //   (A) following a segment via `other_pos`, and
            //   (B) searching for the next outgoing edge on same point.
            // - Therefore, the loop contains two "mark pos as processed" steps, using the
            //   convention that at beginning of the loop, `pos` isn't marked yet.
            // - The contour is extended after following a segment.
            // - Hitting pos == orig_pos after search (B) indicates no continuation and
            //   terminates the loop.
            mark_as_processed(&mut processed, &result_events, pos, contour_id);

            pos = result_events[pos as usize].get_other_pos();              // pos advancment (A)

            mark_as_processed(&mut processed, &result_events, pos, contour_id);
            contour.points.push(result_events[pos as usize].point);

            pos = next_pos(pos, &result_events, &processed, orig_pos);      // pos advancment (B)

            if pos == orig_pos {
                break;
            }
        }

        // This assert should be possible once the first stage of the algorithm is robust.
        // debug_assert_eq!(contour.points.first(), contour.points.last());

        result.push(contour);
    }

    result
}
