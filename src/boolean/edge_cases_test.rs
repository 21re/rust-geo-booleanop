use super::helper::test::{fixture_shapes, xy};
use super::BooleanOp;

#[test]
fn touching_hourglass_intersection() {
    let (s, c) = fixture_shapes("hourglasses.geojson");

    let result = s.intersection(&c);

    assert_eq!(result.0.len(), 2);
    assert_eq!(
        result.0[0].exterior.0,
        vec![xy(0, 0.5), xy(0.25, 0.75), xy(0, 1), xy(0, 0.5)]
    );
    assert_eq!(result.0[0].interiors.len(), 0);
    assert_eq!(
        result.0[1].exterior.0,
        vec![xy(0.75, 0.75), xy(1, 0.5), xy(1, 1), xy(0.75, 0.75)]
    );
    assert_eq!(result.0[1].interiors.len(), 0);
}

#[test]
fn touching_hourglass_union() {
    let (s, c) = fixture_shapes("hourglasses.geojson");

    let result = s.union(&c);

    assert_eq!(result.0.len(), 2);
    assert_eq!(
        result.0[0].exterior.0,
        vec![
            xy(0, 0),
            xy(0.5, 0.5),
            xy(0.25, 0.75),
            xy(0.5, 1),
            xy(0, 1.5),
            xy(0, 1),
            xy(0, 0.5),
            xy(0, 0)
        ]
    );
    assert_eq!(result.0[0].interiors.len(), 0);
    assert_eq!(
        result.0[1].exterior.0,
        vec![
            xy(0.5, 0.5),
            xy(1, 0),
            xy(1, 0.5),
            xy(1, 1),
            xy(1, 1.5),
            xy(0.5, 1),
            xy(0.75, 0.75),
            xy(0.5, 0.5)
        ]
    );
    assert_eq!(result.0[1].interiors.len(), 0);
}

#[test]
fn touching_hourglass_difference() {
    let (s, c) = fixture_shapes("hourglasses.geojson");

    let result = s.difference(&c);

    assert_eq!(result.0.len(), 2);
    assert_eq!(
        result.0[0].exterior.0,
        vec![xy(0, 0), xy(0.5, 0.5), xy(0.25, 0.75), xy(0, 0.5), xy(0, 0)]
    );
    assert_eq!(result.0[0].interiors.len(), 0);
    assert_eq!(
        result.0[1].exterior.0,
        vec![xy(0.5, 0.5), xy(1, 0), xy(1, 0.5), xy(0.75, 0.75), xy(0.5, 0.5)]
    );
    assert_eq!(result.0[1].interiors.len(), 0);
}

#[test]
fn touching_hourglass_differenc2() {
    let (s, c) = fixture_shapes("hourglasses.geojson");

    let result = c.difference(&s);

    assert_eq!(result.0.len(), 2);
    assert_eq!(
        result.0[0].exterior.0,
        vec![xy(0, 1), xy(0.25, 0.75), xy(0.5, 1), xy(0, 1.5), xy(0, 1)]
    );
    assert_eq!(result.0[0].interiors.len(), 0);
    assert_eq!(
        result.0[1].exterior.0,
        vec![xy(0.5, 1), xy(0.75, 0.75), xy(1, 1), xy(1, 1.5), xy(0.5, 1)]
    );
    assert_eq!(result.0[1].interiors.len(), 0);
}

#[test]
fn polygon_trapezoid_overlap_intersection() {
    let (s, c) = fixture_shapes("polygon_trapezoid_edge_overlap.geojson");

    let result = s.intersection(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior.0,
        vec![xy(3.5, 3.5), xy(7, 0), xy(14, 0), xy(17.5, 3.5), xy(3.5, 3.5)]
    );
    assert_eq!(result.0[0].interiors.len(), 0);
}

#[test]
fn polygon_trapezoid_overlap_union() {
    let (s, c) = fixture_shapes("polygon_trapezoid_edge_overlap.geojson");

    let result = s.union(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior.0,
        vec![
            xy(0, 0),
            xy(7, 0),
            xy(14, 0),
            xy(21, 0),
            xy(21, 3.5),
            xy(17.5, 3.5),
            xy(21, 7),
            xy(0, 7),
            xy(3.5, 3.5),
            xy(0, 3.5),
            xy(0, 0)
        ]
    );
    assert_eq!(result.0[0].interiors.len(), 0);
}

#[test]
fn polygon_trapezoid_overlap_difference() {
    let (s, c) = fixture_shapes("polygon_trapezoid_edge_overlap.geojson");

    let result = s.difference(&c);

    assert_eq!(result.0.len(), 2);
    assert_eq!(
        result.0[0].exterior.0,
        vec![xy(0, 0), xy(7, 0), xy(3.5, 3.5), xy(0, 3.5), xy(0, 0)]
    );
    assert_eq!(result.0[0].interiors.len(), 0);
    assert_eq!(
        result.0[1].exterior.0,
        vec![xy(14, 0), xy(21, 0), xy(21, 3.5), xy(17.5, 3.5), xy(14, 0)]
    );
    assert_eq!(result.0[1].interiors.len(), 0);
}

#[test]
fn overlap_loop_intersection() {
    let (s, c) = fixture_shapes("overlap_loop.geojson");

    let result = s.intersection(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior.0,
        vec![
            xy(57.8, -49.1),
            xy(177.8, -49.1),
            xy(177.8, -37.1),
            xy(57.8, -37.1),
            xy(57.8, -49.1)
        ]
    );
    assert_eq!(result.0[0].interiors.len(), 0);
}

#[test]
fn overlap_loop_union() {
    let (s, c) = fixture_shapes("overlap_loop.geojson");

    let result = s.union(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior.0,
        vec![
            xy(57.8, -97.1),
            xy(196.4, -97.1),
            xy(196.4, -11.5),
            xy(57.8, -11.5),
            xy(57.8, -37.1),
            xy(57.8, -49.1),
            xy(57.8, -97.1)
        ]
    );
    assert_eq!(result.0[0].interiors.len(), 0);
}

#[test]
fn overlap_loop_difference() {
    let (s, c) = fixture_shapes("overlap_loop.geojson");

    let result = s.difference(&c);

    assert_eq!(result.0.len(), 0);
}

#[test]
fn overlap_y_shift_intersection() {
    let (s, c) = fixture_shapes("overlap_y.geojson");

    let result = s.intersection(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior.0,
        vec![
            xy(-1883, -8.5),
            xy(-1783, -8.5),
            xy(-1783, -3),
            xy(-1883, -3),
            xy(-1883, -8.5)
        ]
    );
    assert_eq!(result.0[0].interiors.len(), 0);
}

#[test]
fn overlap_y_shift_union() {
    let (s, c) = fixture_shapes("overlap_y.geojson");

    let result = s.union(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior.0,
        vec![
            xy(-1883, -25),
            xy(-1783, -25),
            xy(-1783, -8.5),
            xy(-1783, -3),
            xy(-1783, 75),
            xy(-1883, 75),
            xy(-1883, -3),
            xy(-1883, -8.5),
            xy(-1883, -25)
        ]
    );
    assert_eq!(result.0[0].interiors.len(), 0);
}

#[test]
fn overlap_y_shift_difference() {
    let (s, c) = fixture_shapes("overlap_y.geojson");

    let result = s.difference(&c);

    assert_eq!(result.0.len(), 0);
}

#[test]
fn touching_boxes_intersection() {
    let (s, c) = fixture_shapes("touching_boxes.geojson");

    let result = s.intersection(&c);

    assert_eq!(result.0.len(), 0);
}

#[test]
fn touching_boxes_union() {
    let (s, c) = fixture_shapes("touching_boxes.geojson");

    let result = s.union(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior.0,
        vec![
            xy(0, 0),
            xy(3, 0),
            xy(3, 1),
            xy(4, 1),
            xy(4, 2),
            xy(3, 2),
            xy(3, 3),
            xy(0, 3),
            xy(0, 0)
        ]
    );
    assert_eq!(result.0[0].interiors.len(), 0);
}

#[test]
fn touching_boxes_difference() {
    let (s, c) = fixture_shapes("touching_boxes.geojson");

    let result = s.difference(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior.0,
        vec![xy(0, 0), xy(3, 0), xy(3, 1), xy(3, 2), xy(3, 3), xy(0, 3), xy(0, 0)]
    );
    assert_eq!(result.0[0].interiors.len(), 0);
}

#[test]
fn fatal1_intersection() {
    let (s, c) = fixture_shapes("fatal1.geojson");

    let result = s.intersection(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior.0,
        vec![
            xy(117.6317159208374, 3.2710533372738473),
            xy(117.63180470386553, 3.2708954059271287),
            xy(117.6320843, 3.2708497),
            xy(117.6321104, 3.2709415),
            xy(117.6317159208374, 3.2710533372738473)
        ]
    );
    assert_eq!(result.0[0].interiors.len(), 0);
}

#[test]
fn fatal2_union() {
    let (s, c) = fixture_shapes("fatal1.geojson");

    let result = s.union(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior.0,
        vec![
            xy(117.62484785200004, 3.283270575000117),
            xy(117.6317159208374, 3.2710533372738473),
            xy(117.6315993, 3.2710864),
            xy(117.631605, 3.2711063),
            xy(117.6315403, 3.2711246),
            xy(117.6314897, 3.2709469),
            xy(117.63180470386553, 3.2708954059271287),
            xy(117.63331139400016, 3.268215236000103),
            xy(117.659922722, 3.255275783000087),
            xy(117.62484785200004, 3.283270575000117)
        ]
    );
    assert_eq!(result.0[0].interiors.len(), 0);
}

#[test]
fn fatal1_difference() {
    let (s, c) = fixture_shapes("fatal1.geojson");

    let result = s.difference(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior.0,
        vec![
            xy(117.62484785200004, 3.283270575000117),
            xy(117.6317159208374, 3.2710533372738473),
            xy(117.6321104, 3.2709415),
            xy(117.6320843, 3.2708497),
            xy(117.63180470386553, 3.2708954059271287),
            xy(117.63331139400016, 3.268215236000103),
            xy(117.659922722, 3.255275783000087),
            xy(117.62484785200004, 3.283270575000117)
        ]
    );
    assert_eq!(result.0[0].interiors.len(), 0);
}

#[test]
fn fatal2_intersection() {
    let (s, c) = fixture_shapes("fatal2.geojson");

    let result = s.intersection(&c);

    assert_eq!(result.0.len(), 4);
    assert_eq!(
        result.0[0].exterior.0,
        vec![
            xy(-79.887688, 40.444658),
            xy(-79.88768799972165, 40.44465799897759),
            xy(-79.88768795318525, 40.44465798378203),
            xy(-79.887688, 40.444658)
        ]
    );
    assert_eq!(result.0[0].interiors.len(), 0);
    assert_eq!(
        result.0[1].exterior.0,
        vec![
            xy(-79.88768796122203, 40.444657857562895),
            xy(-79.8872430162168, 40.4430235100967),
            xy(-79.887574, 40.44424199906834),
            xy(-79.88768796122203, 40.444657857562895)
        ]
    );
    assert_eq!(result.0[1].interiors.len(), 0);
    assert_eq!(
        result.0[2].exterior.0,
        vec![
            xy(-79.88761078560455, 40.444631250727284),
            xy(-79.88757599999991, 40.44461799906841),
            xy(-79.887472, 40.44457999906844),
            xy(-79.887351, 40.444534999068445),
            xy(-79.88724, 40.44449899906847),
            xy(-79.887128, 40.44446399906846),
            xy(-79.8871280003921, 40.44446400013584),
            xy(-79.88761078560455, 40.444631250727284)
        ]
    );
    assert_eq!(result.0[2].interiors.len(), 0);
    assert_eq!(
        result.0[3].exterior.0,
        vec![
            xy(-79.88711873229528, 40.44256717591859),
            xy(-79.88685922414403, 40.4416281542633),
            xy(-79.88690199999989, 40.44178499906848),
            xy(-79.887067, 40.4423799990685),
            xy(-79.88711873229528, 40.44256717591859)
        ]
    );
    assert_eq!(result.0[3].interiors.len(), 0);
}
