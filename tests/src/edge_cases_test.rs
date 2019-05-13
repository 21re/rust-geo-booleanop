use super::helper::{fixture_shapes, xy};
use geo_booleanop::boolean::BooleanOp;

#[test]
fn touching_hourglass_intersection() {
    let (s, c) = fixture_shapes("hourglasses.geojson");

    let result = s.intersection(&c);

    assert_eq!(result.0.len(), 2);
    assert_eq!(
        result.0[0].exterior().0,
        vec![xy(0, 0.5), xy(0.25, 0.75), xy(0, 1), xy(0, 0.5)]
    );
    assert_eq!(result.0[0].interiors().len(), 0);
    assert_eq!(
        result.0[1].exterior().0,
        vec![xy(0.75, 0.75), xy(1, 0.5), xy(1, 1), xy(0.75, 0.75)]
    );
    assert_eq!(result.0[1].interiors().len(), 0);
}

#[test]
fn touching_hourglass_union() {
    let (s, c) = fixture_shapes("hourglasses.geojson");

    let result = s.union(&c);

    assert_eq!(result.0.len(), 2);
    assert_eq!(
        result.0[0].exterior().0,
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
    assert_eq!(result.0[0].interiors().len(), 0);
    assert_eq!(
        result.0[1].exterior().0,
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
    assert_eq!(result.0[1].interiors().len(), 0);
}

#[test]
fn touching_hourglass_difference() {
    let (s, c) = fixture_shapes("hourglasses.geojson");

    let result = s.difference(&c);

    assert_eq!(result.0.len(), 2);
    assert_eq!(
        result.0[0].exterior().0,
        vec![xy(0, 0), xy(0.5, 0.5), xy(0.25, 0.75), xy(0, 0.5), xy(0, 0)]
    );
    assert_eq!(result.0[0].interiors().len(), 0);
    assert_eq!(
        result.0[1].exterior().0,
        vec![xy(0.5, 0.5), xy(1, 0), xy(1, 0.5), xy(0.75, 0.75), xy(0.5, 0.5)]
    );
    assert_eq!(result.0[1].interiors().len(), 0);
}

#[test]
fn touching_hourglass_differenc2() {
    let (s, c) = fixture_shapes("hourglasses.geojson");

    let result = c.difference(&s);

    assert_eq!(result.0.len(), 2);
    assert_eq!(
        result.0[0].exterior().0,
        vec![xy(0, 1), xy(0.25, 0.75), xy(0.5, 1), xy(0, 1.5), xy(0, 1)]
    );
    assert_eq!(result.0[0].interiors().len(), 0);
    assert_eq!(
        result.0[1].exterior().0,
        vec![xy(0.5, 1), xy(0.75, 0.75), xy(1, 1), xy(1, 1.5), xy(0.5, 1)]
    );
    assert_eq!(result.0[1].interiors().len(), 0);
}

#[test]
fn polygon_trapezoid_overlap_intersection() {
    let (s, c) = fixture_shapes("polygon_trapezoid_edge_overlap.geojson");

    let result = s.intersection(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior().0,
        vec![xy(3.5, 3.5), xy(7, 0), xy(14, 0), xy(17.5, 3.5), xy(3.5, 3.5)]
    );
    assert_eq!(result.0[0].interiors().len(), 0);
}

#[test]
fn polygon_trapezoid_overlap_union() {
    let (s, c) = fixture_shapes("polygon_trapezoid_edge_overlap.geojson");

    let result = s.union(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior().0,
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
    assert_eq!(result.0[0].interiors().len(), 0);
}

#[test]
fn polygon_trapezoid_overlap_difference() {
    let (s, c) = fixture_shapes("polygon_trapezoid_edge_overlap.geojson");

    let result = s.difference(&c);

    assert_eq!(result.0.len(), 2);
    assert_eq!(
        result.0[0].exterior().0,
        vec![xy(0, 0), xy(7, 0), xy(3.5, 3.5), xy(0, 3.5), xy(0, 0)]
    );
    assert_eq!(result.0[0].interiors().len(), 0);
    assert_eq!(
        result.0[1].exterior().0,
        vec![xy(14, 0), xy(21, 0), xy(21, 3.5), xy(17.5, 3.5), xy(14, 0)]
    );
    assert_eq!(result.0[1].interiors().len(), 0);
}

#[test]
fn overlap_loop_intersection() {
    let (s, c) = fixture_shapes("overlap_loop.geojson");

    let result = s.intersection(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior().0,
        vec![
            xy(57.8, -49.1),
            xy(177.8, -49.1),
            xy(177.8, -37.1),
            xy(57.8, -37.1),
            xy(57.8, -49.1)
        ]
    );
    assert_eq!(result.0[0].interiors().len(), 0);
}

#[test]
fn overlap_loop_union() {
    let (s, c) = fixture_shapes("overlap_loop.geojson");

    let result = s.union(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior().0,
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
    assert_eq!(result.0[0].interiors().len(), 0);
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
        result.0[0].exterior().0,
        vec![
            xy(-1883, -8.5),
            xy(-1783, -8.5),
            xy(-1783, -3),
            xy(-1883, -3),
            xy(-1883, -8.5)
        ]
    );
    assert_eq!(result.0[0].interiors().len(), 0);
}

#[test]
fn overlap_y_shift_union() {
    let (s, c) = fixture_shapes("overlap_y.geojson");

    let result = s.union(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior().0,
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
    assert_eq!(result.0[0].interiors().len(), 0);
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
        result.0[0].exterior().0,
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
    assert_eq!(result.0[0].interiors().len(), 0);
}

#[test]
fn touching_boxes_difference() {
    let (s, c) = fixture_shapes("touching_boxes.geojson");

    let result = s.difference(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior().0,
        vec![xy(0, 0), xy(3, 0), xy(3, 1), xy(3, 2), xy(3, 3), xy(0, 3), xy(0, 0)]
    );
    assert_eq!(result.0[0].interiors().len(), 0);
}

#[test]
fn fatal1_intersection() {
    let (s, c) = fixture_shapes("fatal1.geojson");

    let result = s.intersection(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior().0,
        vec![
            xy(117.6317159208374, 3.2710533372738473),
            xy(117.63180470386553, 3.2708954059271287),
            xy(117.6320843, 3.2708497),
            xy(117.6321104, 3.2709415),
            xy(117.6317159208374, 3.2710533372738473)
        ]
    );
    assert_eq!(result.0[0].interiors().len(), 0);
}

#[test]
fn fatal1_union() {
    let (s, c) = fixture_shapes("fatal1.geojson");

    let result = s.union(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior().0,
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
    assert_eq!(result.0[0].interiors().len(), 0);
}

#[test]
fn fatal1_difference() {
    let (s, c) = fixture_shapes("fatal1.geojson");

    let result = s.difference(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior().0,
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
    assert_eq!(result.0[0].interiors().len(), 0);
}

#[test]
fn fatal2_intersection() {
    let (s, c) = fixture_shapes("fatal2.geojson");

    let result = s.intersection(&c);

    assert_eq!(result.0.len(), 4);
    assert_eq!(
        result.0[0].exterior().0,
        vec![
            xy(-79.887688, 40.444658),
            xy(-79.88768799972165, 40.44465799897759),
            xy(-79.88768795318525, 40.44465798378203),
            xy(-79.887688, 40.444658)
        ]
    );
    assert_eq!(result.0[0].interiors().len(), 0);
    assert_eq!(
        result.0[1].exterior().0,
        vec![
            xy(-79.88768796122203, 40.444657857562895),
            xy(-79.8872430162168, 40.4430235100967),
            xy(-79.887574, 40.44424199906834),
            xy(-79.88768796122203, 40.444657857562895)
        ]
    );
    assert_eq!(result.0[1].interiors().len(), 0);
    assert_eq!(
        result.0[2].exterior().0,
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
    assert_eq!(result.0[2].interiors().len(), 0);
    assert_eq!(
        result.0[3].exterior().0,
        vec![
            xy(-79.88711873229528, 40.44256717591859),
            xy(-79.88685922414403, 40.4416281542633),
            xy(-79.88690199999989, 40.44178499906848),
            xy(-79.887067, 40.4423799990685),
            xy(-79.88711873229528, 40.44256717591859)
        ]
    );
    assert_eq!(result.0[3].interiors().len(), 0);
}

#[test]
fn fatal2_union() {
    let (s, c) = fixture_shapes("fatal2.geojson");

    let result = s.union(&c);

    assert_eq!(result.0.len(), 4);
    assert_eq!(
        result.0[0].exterior().0,
        vec![
            xy(-79.894363, 40.44117499906849),
            xy(-79.894272, 40.44123699906842),
            xy(-79.894179, 40.44123699906842),
            xy(-79.89357, 40.44123799906848),
            xy(-79.893368, 40.441238999068446),
            xy(-79.893169, 40.441239999068465),
            xy(-79.8925749999999, 40.44124399906848),
            xy(-79.892543, 40.44124499906835),
            xy(-79.892377, 40.44125199906844),
            xy(-79.892264, 40.44125599906838),
            xy(-79.892213, 40.44125799906857),
            xy(-79.891928, 40.441241999068474),
            xy(-79.891816, 40.44123699906842),
            xy(-79.891702, 40.441229999068575),
            xy(-79.891555, 40.44122299906853),
            xy(-79.891502, 40.44121599906848),
            xy(-79.891437, 40.4412009990685),
            xy(-79.891368, 40.44117599906851),
            xy(-79.891262, 40.441137999068566),
            xy(-79.890921, 40.44101499906854),
            xy(-79.89092, 40.44101499906854),
            xy(-79.889892, 40.44067299906851),
            xy(-79.88955, 40.440559999068554),
            xy(-79.889376, 40.44050199906853),
            xy(-79.888857, 40.44032999906851),
            xy(-79.888684, 40.440272999068426),
            xy(-79.888579, 40.440237999068586),
            xy(-79.888264, 40.44013299906854),
            xy(-79.888159, 40.44009899906841),
            xy(-79.88793, 40.440022999068574),
            xy(-79.8872429999999, 40.43979699906849),
            xy(-79.887015, 40.43972199906852),
            xy(-79.886931, 40.43969399906849),
            xy(-79.886921, 40.43969299906843),
            xy(-79.886882, 40.43968999906852),
            xy(-79.886626, 40.439687999068525),
            xy(-79.886528, 40.439687999068525),
            xy(-79.886501, 40.43968699906842),
            xy(-79.88642, 40.43968699906842),
            xy(-79.886393, 40.43968699906842),
            xy(-79.886406, 40.43974199906857),
            xy(-79.886445, 40.43990699906852),
            xy(-79.886458, 40.43996299906853),
            xy(-79.886476, 40.440069999068506),
            xy(-79.88648, 40.44009399906854),
            xy(-79.88652999999987, 40.44039399906853),
            xy(-79.886548, 40.44050199906853),
            xy(-79.886608, 40.44071799906848),
            xy(-79.88661599999989, 40.44074599906848),
            xy(-79.886788, 40.4413689990685),
            xy(-79.886848, 40.441586999068434),
            xy(-79.88685922414403, 40.4416281542633),
            xy(-79.886548, 40.440502),
            xy(-79.886393, 40.439687),
            xy(-79.885782, 40.436843),
            xy(-79.882656, 40.436087),
            xy(-79.881163, 40.438717),
            xy(-79.880716, 40.439506),
            xy(-79.879353, 40.441889),
            xy(-79.880724, 40.442343),
            xy(-79.887128, 40.444464),
            xy(-79.8871280003921, 40.44446400013584),
            xy(-79.887182, 40.44461099906854),
            xy(-79.887176, 40.44464599906839),
            xy(-79.887158, 40.4447569990685),
            xy(-79.887134, 40.444810999068544),
            xy(-79.88695099999988, 40.445137999068415),
            xy(-79.886934, 40.44516499906851),
            xy(-79.886837, 40.44533099906844),
            xy(-79.886745, 40.44548899906847),
            xy(-79.886472, 40.44596399906852),
            xy(-79.886381, 40.446122999068365),
            xy(-79.886335, 40.44620099906844),
            xy(-79.886199, 40.4464369990684),
            xy(-79.886155, 40.44651599906838),
            xy(-79.88611, 40.44659399906846),
            xy(-79.885974, 40.446827999068454),
            xy(-79.88593, 40.44690699906842),
            xy(-79.886143, 40.446980999068494),
            xy(-79.88638, 40.447058999068375),
            xy(-79.887733, 40.44750899906846),
            xy(-79.888184, 40.44765899906843),
            xy(-79.888318, 40.44771499906848),
            xy(-79.888366, 40.44773499906852),
            xy(-79.888406, 40.44777299906845),
            xy(-79.888547, 40.447843999068475),
            xy(-79.888729, 40.44806099906853),
            xy(-79.889005, 40.44832999906835),
            xy(-79.88924199999991, 40.448540999068435),
            xy(-79.889816, 40.44905099906851),
            xy(-79.890444, 40.44945899906849),
            xy(-79.89107299999988, 40.4497789990684),
            xy(-79.891154, 40.44982099906843),
            xy(-79.891768, 40.45009799906849),
            xy(-79.892038, 40.45017699906838),
            xy(-79.892372, 40.45025199906842),
            xy(-79.892423, 40.45026599906851),
            xy(-79.892423, 40.45017299906844),
            xy(-79.892423, 40.45010299906852),
            xy(-79.892424, 40.45006999906842),
            xy(-79.892429, 40.44997199906848),
            xy(-79.892431, 40.44993999906836),
            xy(-79.892436, 40.449918999068366),
            xy(-79.892452, 40.44985699906838),
            xy(-79.892458, 40.44983699906852),
            xy(-79.892529, 40.44956799906845),
            xy(-79.892742, 40.44876399906836),
            xy(-79.892759, 40.44870099906847),
            xy(-79.892788, 40.44858799906836),
            xy(-79.892802, 40.44851799906848),
            xy(-79.892996, 40.4475889990685),
            xy(-79.893015, 40.447503999068445),
            xy(-79.893021, 40.447475999068395),
            xy(-79.893163, 40.44684999906851),
            xy(-79.893225, 40.44649399906848),
            xy(-79.893242, 40.44640399906853),
            xy(-79.893338, 40.44599099906845),
            xy(-79.893428, 40.44549099906846),
            xy(-79.893578, 40.444658999068416),
            xy(-79.893584, 40.44458899906848),
            xy(-79.893751, 40.443836999068466),
            xy(-79.893782, 40.443722999068385),
            xy(-79.893841, 40.44344499906848),
            xy(-79.893923, 40.443065999068416),
            xy(-79.893999, 40.44271799906857),
            xy(-79.894005, 40.44268899906845),
            xy(-79.894085, 40.44231599906854),
            xy(-79.894169, 40.441928999068494),
            xy(-79.894172, 40.44191599906846),
            xy(-79.894231, 40.44163799906855),
            xy(-79.894242, 40.44160999906842),
            xy(-79.894257, 40.44156199906851),
            xy(-79.894279, 40.44148399906843),
            xy(-79.894336, 40.44128999906837),
            xy(-79.894344, 40.4412529990685),
            xy(-79.894363, 40.44117499906849)
        ]
    );
    assert_eq!(result.0[0].interiors().len(), 0);
    assert_eq!(
        result.0[1].exterior().0,
        vec![
            xy(-79.887688, 40.444657999068475),
            xy(-79.88768796122203, 40.444657857562895),
            xy(-79.88768799972165, 40.44465799897759),
            xy(-79.887688, 40.444657999068475)
        ]
    );
    assert_eq!(result.0[1].interiors().len(), 0);
    assert_eq!(
        result.0[2].exterior().0,
        vec![
            xy(-79.88768795318525, 40.44465798378203),
            xy(-79.88761078560455, 40.444631250727284),
            xy(-79.887639, 40.44464199906838),
            xy(-79.88768795318525, 40.44465798378203)
        ]
    );
    assert_eq!(result.0[2].interiors().len(), 0);
    assert_eq!(
        result.0[3].exterior().0,
        vec![
            xy(-79.8872430162168, 40.4430235100967),
            xy(-79.887235, 40.44299399906848),
            xy(-79.887122, 40.44257899906844),
            xy(-79.88711873229528, 40.44256717591859),
            xy(-79.887122, 40.442579),
            xy(-79.8872430162168, 40.4430235100967)
        ]
    );
    assert_eq!(result.0[3].interiors().len(), 0);
}

#[test]
fn fatal2_difference() {
    let (s, c) = fixture_shapes("fatal2.geojson");

    let result = s.difference(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior().0,
        vec![
            xy(-79.88768799972165, 40.44465799897759),
            xy(-79.88768796122203, 40.444657857562895),
            xy(-79.887574, 40.44424199906834),
            xy(-79.8872430162168, 40.4430235100967),
            xy(-79.887122, 40.442579),
            xy(-79.88711873229528, 40.44256717591859),
            xy(-79.887067, 40.4423799990685),
            xy(-79.88690199999989, 40.44178499906848),
            xy(-79.88685922414403, 40.4416281542633),
            xy(-79.886548, 40.440502),
            xy(-79.886393, 40.439687),
            xy(-79.885782, 40.436843),
            xy(-79.882656, 40.436087),
            xy(-79.881163, 40.438717),
            xy(-79.880716, 40.439506),
            xy(-79.879353, 40.441889),
            xy(-79.880724, 40.442343),
            xy(-79.887128, 40.444464),
            xy(-79.8871280003921, 40.44446400013584),
            xy(-79.887128, 40.44446399906846),
            xy(-79.88724, 40.44449899906847),
            xy(-79.887351, 40.444534999068445),
            xy(-79.887472, 40.44457999906844),
            xy(-79.88757599999991, 40.44461799906841),
            xy(-79.88761078560455, 40.444631250727284),
            xy(-79.88768795318525, 40.44465798378203),
            xy(-79.88768799972165, 40.44465799897759)
        ]
    );
    assert_eq!(result.0[0].interiors().len(), 0);
}

#[test]
fn rectangles_intersection() {
    let (s, c) = fixture_shapes("rectangles.geojson");

    let result = s.intersection(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior().0,
        vec![
            xy(-19.3046867422006, -126.63400219275148),
            xy(-19.3046867422006, -107.63400219275148),
            xy(10.695313257799395, -107.63400219275148),
            xy(10.695313257799395, -126.63400219275148),
            xy(-19.3046867422006, -126.63400219275148)
        ]
    );
    assert_eq!(result.0[0].interiors().len(), 0);
}

#[test]
fn rectangles_union() {
    let (s, c) = fixture_shapes("rectangles.geojson");

    let result = s.union(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior().0,
        vec![
            xy(-96.6603326972832, -126.63400219275148),
            xy(-19.3046867422006, -126.63400219275148),
            xy(-19.304686742200587, -357.48241878255635),
            xy(10.695313257799413, -357.48241878255635),
            xy(10.695313257799395, -126.63400219275148),
            xy(13.370917302716792, -126.63400219275148),
            xy(13.370917302716792, -107.63400219275148),
            xy(10.695313257799395, -107.63400219275148),
            xy(10.695313257799384, 126.92383121744363),
            xy(-19.304686742200616, 126.92383121744363),
            xy(-19.3046867422006, -107.63400219275148),
        ]
    );
    assert_eq!(result.0[0].interiors().len(), 0);
}

#[test]
fn rectangles_difference() {
    let (s, c) = fixture_shapes("rectangles.geojson");

    let result = s.difference(&c);

    assert_eq!(result.0.len(), 1);
    assert_eq!(
        result.0[0].exterior().0,
        vec![
            xy(-19.304686742200616, 126.92383121744363),
            xy(-19.3046867422006, -107.63400219275148),
            xy(10.695313257799395, -107.63400219275148),
            xy(10.695313257799384, 126.92383121744363),
            xy(10.695313257799413, -357.48241878255635),
            xy(10.695313257799395, -126.63400219275148),
            xy(-19.3046867422006, -126.63400219275148),
            xy(-19.304686742200587, -357.48241878255633),
        ]
    );
    assert_eq!(result.0[0].interiors().len(), 0);
}
