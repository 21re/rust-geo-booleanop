use super::helper::Float;
use super::sweep_event::{ResultTransition, SweepEvent};
use geo_types::Coordinate;
use std::collections::HashSet;
use std::rc::Rc;

fn order_events<F>(sorted_events: &[Rc<SweepEvent<F>>]) -> Vec<Rc<SweepEvent<F>>>
where
    F: Float,
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

    result_events
}

/// Helper function that identifies groups of sweep event that belong to one
/// vertex, and precomputes in which order the events within one group should
/// be iterated. The result is a vector with the semantics:
///
/// map[i] = index of next event belonging to vertex
///
/// Iteration order is in positive index direction for right events, but in
/// reverse index direction for left events in order to ensure strict clockwise
/// iteration around the vertex.
#[allow(clippy::needless_range_loop)] // Check has false positive here
fn precompute_iteration_order<T, I, L>(data: &[T], is_identical: I, is_left: L) -> Vec<usize>
where
    I: Fn(&T, &T) -> bool,
    L: Fn(&T) -> bool,
{
    let mut map = vec![0; data.len()];

    let mut i = 0;
    while i < data.len() {
        let x_ref = &data[i];

        // Find index range of R events
        let r_from = i;
        while i < data.len() && is_identical(x_ref, &data[i]) && !is_left(&data[i]) {
            i += 1;
        }
        let r_upto_exclusive = i;

        // Find index range of L event
        let l_from = i;
        while i < data.len() && is_identical(x_ref, &data[i]) {
            debug_assert!(is_left(&data[i]));
            i += 1;
        }
        let l_upto_exclusive = i;

        let has_r_events = r_upto_exclusive > r_from;
        let has_l_events = l_upto_exclusive > l_from;

        if has_r_events {
            let r_upto = r_upto_exclusive - 1;
            // Connect elements in [r_from, r_upto) to larger index
            for j in r_from..r_upto {
                map[j] = j + 1;
            }
            // Special handling of *last* element: Connect either the last L event
            // or loop back to start of R events (if no L events).
            if has_l_events {
                map[r_upto] = l_upto_exclusive - 1;
            } else {
                map[r_upto] = r_from;
            }
        }
        if has_l_events {
            let l_upto = l_upto_exclusive - 1;
            // Connect elements in (l_from, l_upto] to lower index
            for j in l_from + 1..=l_upto {
                map[j] = j - 1;
            }
            // Special handling of *first* element: Connect either to the first R event
            // or loop back to end of L events (if no R events).
            if has_r_events {
                map[l_from] = r_from;
            } else {
                map[l_from] = l_upto;
            }
        }
    }

    map
}

fn get_next_pos(pos: i32, processed: &HashSet<i32>, iteration_map: &[usize]) -> Option<i32> {
    let mut pos = pos;
    let start_pos = pos;

    loop {
        pos = iteration_map[pos as usize] as i32;
        if pos == start_pos {
            // Entire group is already processed?
            return None;
        } else if !processed.contains(&pos) {
            return Some(pos);
        }
    }
}

pub struct Contour<F>
where
    F: Float,
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
    F: Float,
{
    pub fn new(hole_of: Option<i32>, depth: i32) -> Contour<F> {
        Contour {
            points: Vec::new(),
            hole_ids: Vec::new(),
            hole_of,
            depth,
        }
    }

    /// This logic implements the 4 cases of parent contours from Fig. 4 in the Martinez paper.
    pub fn initialize_from_context(
        event: &Rc<SweepEvent<F>>,
        contours: &mut [Contour<F>],
        contour_id: i32,
    ) -> Contour<F> {
        if let Some(prev_in_result) = event.get_prev_in_result() {
            // Note that it is valid to query the "previous in result" for its output contour id,
            // because we must have already processed it (i.e., assigned an output contour id)
            // in an earlier iteration, otherwise it wouldn't be possible that it is "previous in
            // result".
            let lower_contour_id = prev_in_result.get_output_contour_id();
            if prev_in_result.get_result_transition() == ResultTransition::OutIn {
                // We are inside. Now we have to check if the thing below us is another hole or
                // an exterior contour.
                let lower_contour = &contours[lower_contour_id as usize];
                if let Some(parent_contour_id) = lower_contour.hole_of {
                    // The lower contour is a hole => Connect the new contour as a hole to its parent,
                    // and use same depth.
                    contours[parent_contour_id as usize].hole_ids.push(contour_id);
                    let hole_of = Some(parent_contour_id);
                    let depth = contours[lower_contour_id as usize].depth;
                    Contour::new(hole_of, depth)
                } else {
                    // The lower contour is an exterior contour => Connect the new contour as a hole,
                    // and increment depth.
                    contours[lower_contour_id as usize].hole_ids.push(contour_id);
                    let hole_of = Some(lower_contour_id);
                    let depth = contours[lower_contour_id as usize].depth + 1;
                    Contour::new(hole_of, depth)
                }
            } else {
                // We are outside => this contour is an exterior contour of same depth.
                let depth = if lower_contour_id < 0 || lower_contour_id as usize >= contours.len() {
                    debug_assert!(false, "Invalid lower_contour_id should be impossible.");
                    0
                } else {
                    contours[lower_contour_id as usize].depth
                };
                Contour::new(None, depth)
            }
        } else {
            // There is no lower/previous contour => this contour is an exterior contour of depth 0.
            Contour::new(None, 0)
        }
    }

    /// Whether a contour is an exterior contour or a hole.
    /// Note: The semantics of `is_exterior` are in the sense of an exterior ring of a
    /// polygon in GeoJSON, not to be confused with "external contour" as used in the
    /// Martinez paper (which refers to contours that are not included in any of the
    /// other polygon contours; `is_exterior` is true for all outer contours, not just
    /// the outermost).
    pub fn is_exterior(&self) -> bool {
        self.hole_of.is_none()
    }
}

fn mark_as_processed<F>(processed: &mut HashSet<i32>, result_events: &[Rc<SweepEvent<F>>], pos: i32, contour_id: i32)
where
    F: Float,
{
    processed.insert(pos);
    result_events[pos as usize].set_output_contour_id(contour_id);
}

pub fn connect_edges<F>(sorted_events: &[Rc<SweepEvent<F>>]) -> Vec<Contour<F>>
where
    F: Float,
{
    let result_events = order_events(sorted_events);

    let iteration_map = precompute_iteration_order(&result_events, |a, b| a.point == b.point, |e| e.is_left());

    #[cfg(feature = "debug-booleanop")]
    write_debug_csv(&result_events);

    let mut contours: Vec<Contour<F>> = Vec::new();
    let mut processed: HashSet<i32> = HashSet::new();

    for i in 0..(result_events.len() as i32) {
        if processed.contains(&i) {
            continue;
        }

        let contour_id = contours.len() as i32;
        let mut contour = Contour::initialize_from_context(&result_events[i as usize], &mut contours, contour_id);

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

            // pos advancement (A)
            pos = result_events[pos as usize].get_other_pos();

            mark_as_processed(&mut processed, &result_events, pos, contour_id);
            contour.points.push(result_events[pos as usize].point);

            // pos advancement (B)
            let next_pos_opt = get_next_pos(pos, &processed, &iteration_map);
            match next_pos_opt {
                Some(npos) => {
                    pos = npos;
                }
                None => {
                    break;
                }
            }

            // Optional: Terminate contours early (to avoid overly long contours that
            // may mix clockwise and counter-clockwise winding rules, which can be more
            // difficult to handle in some use cases).
            if result_events[pos as usize].point == initial {
                break;
            }
        }

        // This assert should be possible once the first stage of the algorithm is robust.
        // debug_assert_eq!(contour.points.first(), contour.points.last());

        contours.push(contour);
    }

    contours
}

// Debug csv output generator
#[cfg(feature = "debug-booleanop")]
use std::fs::File;
#[cfg(feature = "debug-booleanop")]
use std::io::Write;

#[cfg(feature = "debug-booleanop")]
fn write_debug_csv<F>(events: &[Rc<SweepEvent<F>>])
where
    F: Float,
{
    let mut writer = File::create("debug.csv").unwrap();
    writeln!(
        &mut writer,
        "index;x;y;other_x;other_y;lr;result_transition;in_out;other_in_out;is_subject;is_exterior_ring;prev_in_result"
    )
    .expect("Failed to write to file");
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

#[cfg(test)]
mod test_precompute_iteration_order {
    use super::*;
    use LeftRight::*;

    #[derive(Debug, PartialEq)]
    enum LeftRight {
        L,
        R,
    }

    macro_rules! check {
        ($data_values:expr, $data_lr:expr, $expected:expr) => {
            let data_values: &[i32] = &$data_values;
            let data_lr: &[LeftRight] = &$data_lr;
            let data: Vec<(&i32, &LeftRight)> = data_values.iter().zip(data_lr.iter()).collect();
            let expected: &[usize] = &$expected;
            println!("\nData: {:?}\nExpected: {:?}", data, expected);
            assert_eq!(
                precompute_iteration_order(&data, |a, b| a.0 == b.0, |(_, lr)| **lr == L),
                expected,
            );
        };
    }

    #[test]
    fn test_precompute_iteration_order_right_events() {
        check!([], [], []);
        check!([1], [R], [0]);
        check!([1, 2, 3], [R, R, R], [0, 1, 2]);

        check!([1, 1], [R, R], [1, 0]);
        check!([1, 1, 2, 2], [R, R, R, R], [1, 0, 3, 2]);
        check!([1, 2, 2, 3], [R, R, R, R], [0, 2, 1, 3]);
        check!([1, 1, 1, 1], [R, R, R, R], [1, 2, 3, 0]);
    }

    #[test]
    fn test_precompute_iteration_order_left_events() {
        check!([], [], []);
        check!([1], [L], [0]);
        check!([1, 2, 3], [L, L, L], [0, 1, 2]);

        check!([1, 1], [L, L], [1, 0]);
        check!([1, 1, 2, 2], [L, L, L, L], [1, 0, 3, 2]);
        check!([1, 2, 2, 3], [L, L, L, L], [0, 2, 1, 3]);
        check!([1, 1, 1, 1], [L, L, L, L], [3, 0, 1, 2]);
    }

    #[test]
    fn test_precompute_iteration_order_mixed_events() {
        check!([1, 2], [R, L], [0, 1]);
        check!([1, 2], [L, R], [0, 1]);

        check!([1, 1], [R, L], [1, 0]);

        check!([1, 1, 1, 1], [R, R, L, L], [1, 3, 0, 2]);

        check!([1, 1, 1, 2, 2, 2], [R, R, R, L, L, L], [1, 2, 0, 5, 3, 4]);
        check!([1, 1, 1, 2, 2, 2], [L, L, L, R, R, R], [2, 0, 1, 4, 5, 3]);

        check!([1, 1, 1, 1, 1, 1], [R, R, R, L, L, L], [1, 2, 5, 0, 3, 4]);

        check!(
            [1, 1, 1, 1, 2, 2, 3, 3, 3, 3],
            [R, R, L, L, R, L, R, R, L, L],
            [1, 3, 0, 2, 5, 4, 7, 9, 6, 8]
        );
    }
}
