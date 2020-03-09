use geo_booleanop::boolean::BooleanOp;

use super::compact_geojson::write_compact_geojson;

use geo::{Coordinate, MultiPolygon, Polygon};
use geojson::{Feature, GeoJson, Geometry, Value};
use pretty_assertions::assert_eq;

use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::panic::catch_unwind;
use std::thread::Result;

pub fn load_fixture_from_path(path: &str) -> GeoJson {
    let mut file = File::open(path).expect("Cannot open/find fixture");
    let mut content = String::new();

    file.read_to_string(&mut content).expect("Unable to read fixture");

    content.parse::<GeoJson>().expect("Fixture is no geojson")
}

pub fn load_fixture(name: &str) -> GeoJson {
    load_fixture_from_path(&format!("./fixtures/{}", name))
}

pub fn fixture_polygon(name: &str) -> Polygon<f64> {
    let shape = match load_fixture(name) {
        GeoJson::Feature(feature) => feature.geometry.unwrap(),
        _ => panic!("Fixture is not a feature collection"),
    };
    shape.value.try_into().expect("Shape is not a polygon")
}

pub fn fixture_multi_polygon(name: &str) -> MultiPolygon<f64> {
    let shape = match load_fixture(name) {
        GeoJson::Feature(feature) => feature.geometry.unwrap(),
        _ => panic!("Fixture is not a feature collection"),
    };

    shape
        .value
        .clone()
        .try_into()
        .map(|p: Polygon<f64>| MultiPolygon(vec![p]))
        .or_else(|_| shape.value.try_into())
        .expect("Shape is not a multi polygon")
}

pub fn fixture_shapes(name: &str) -> (Polygon<f64>, Polygon<f64>) {
    let shapes = match load_fixture(name) {
        GeoJson::FeatureCollection(collection) => collection.features,
        _ => panic!("Fixture is not a feature collection"),
    };
    let s: Polygon<f64> = shapes[0]
        .geometry
        .as_ref()
        .unwrap()
        .value
        .clone()
        .try_into()
        .expect("Shape 1 not a polygon");
    let c: Polygon<f64> = shapes[1]
        .geometry
        .as_ref()
        .unwrap()
        .value
        .clone()
        .try_into()
        .expect("Shape 2 not a polygon");

    (s, c)
}

pub fn xy<X: Into<f64>, Y: Into<f64>>(x: X, y: Y) -> Coordinate<f64> {
    Coordinate {
        x: x.into(),
        y: y.into(),
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TestOperation {
    Intersection,
    Union,
    Xor,
    DifferenceAB,
    DifferenceBA,
}

#[derive(Debug)]
pub struct ExpectedResult {
    pub result: MultiPolygon<f64>,
    pub op: TestOperation,
    pub swap_ab_is_broken: bool,
}

pub fn extract_multi_polygon(feature: &Feature) -> MultiPolygon<f64> {
    let geometry_value = feature
        .geometry
        .as_ref()
        .expect("Feature must have 'geometry' property")
        .value
        .clone();
    let multi_polygon: MultiPolygon<f64> = match geometry_value {
        Value::Polygon(_) => MultiPolygon(vec![geometry_value.try_into().unwrap()]),
        Value::MultiPolygon(_) => geometry_value.try_into().unwrap(),
        _ => panic!("Feature must either be MultiPolygon or Polygon"),
    };
    multi_polygon
}

pub fn extract_expected_result(feature: &Feature) -> ExpectedResult {
    let multi_polygon = extract_multi_polygon(feature);

    let properties = feature.properties.as_ref().expect("Feature needs 'properties'.");

    let op = properties
        .get("operation")
        .expect("Feature 'properties' needs an 'operation' entry.")
        .as_str()
        .expect("'operation' entry must be a string.");

    let swap_ab_is_broken = properties
        .get("swap_ab_is_broken")
        .map(|x| x.as_bool().expect("swap_ab_is_broken must be a boolean"))
        .unwrap_or(false);

    let op = match op {
        "union" => TestOperation::Union,
        "intersection" => TestOperation::Intersection,
        "xor" => TestOperation::Xor,
        "diff" => TestOperation::DifferenceAB,
        "diff_ba" => TestOperation::DifferenceBA,
        _ => panic!(format!("Invalid operation: {}", op)),
    };

    ExpectedResult {
        result: multi_polygon,
        op,
        swap_ab_is_broken,
    }
}

pub fn update_feature(feature: &Feature, p: &MultiPolygon<f64>) -> Feature {
    let mut output_feature = feature.clone();
    output_feature.geometry = Some(Geometry::new(Value::from(p)));
    output_feature
}

pub fn load_test_case(filename: &str) -> (Vec<Feature>, MultiPolygon<f64>, MultiPolygon<f64>) {
    let original_geojson = load_fixture_from_path(filename);
    let features = match original_geojson {
        GeoJson::FeatureCollection(collection) => collection.features,
        _ => panic!("Fixture is not a feature collection"),
    };
    assert!(features.len() >= 2);
    let p1 = extract_multi_polygon(&features[0]);
    let p2 = extract_multi_polygon(&features[1]);
    (features, p1, p2)
}

pub fn apply_operation(p1: &MultiPolygon<f64>, p2: &MultiPolygon<f64>, op: TestOperation) -> MultiPolygon<f64> {
    match op {
        TestOperation::Union => p1.union(p2),
        TestOperation::Intersection => p1.intersection(p2),
        TestOperation::Xor => p1.xor(p2),
        TestOperation::DifferenceAB => p1.difference(p2),
        TestOperation::DifferenceBA => p2.difference(p1),
    }
}

#[derive(Debug)]
enum ResultTag {
    MainResult,
    SwapResult,
}

type WrappedResult = (ResultTag, Result<MultiPolygon<f64>>);

fn compute_all_results(
    p1: &MultiPolygon<f64>,
    p2: &MultiPolygon<f64>,
    op: TestOperation,
    skip_swap_ab: bool,
) -> Vec<WrappedResult> {
    let main_result = catch_unwind(|| {
        println!("Running operation {:?} / {:?}", op, ResultTag::MainResult);
        apply_operation(p1, p2, op)
    });

    let mut results = vec![(ResultTag::MainResult, main_result)];
    let swappable_op = match op {
        TestOperation::DifferenceAB => false,
        TestOperation::DifferenceBA => false,
        _ => true,
    };
    if swappable_op && !skip_swap_ab {
        let swap_result = catch_unwind(|| {
            println!("Running operation {:?} / {:?}", op, ResultTag::SwapResult);
            apply_operation(p2, p1, op)
        });
        results.push((ResultTag::SwapResult, swap_result));
    }
    results
}

pub fn run_generic_test_case(filename: &str, regenerate: bool) -> Vec<String> {
    println!("\n *** Running test case: {}", filename);

    let (features, p1, p2) = load_test_case(filename);

    let mut output_features = vec![features[0].clone(), features[1].clone()];
    let mut failures = Vec::new();

    for feature in features.iter().skip(2) {
        let expected_result = extract_expected_result(&feature);
        let op = expected_result.op;

        let all_results = compute_all_results(&p1, &p2, op, expected_result.swap_ab_is_broken);
        for result in &all_results {
            let (result_tag, result_poly) = result;
            match &result_poly {
                Result::Err(_) => failures.push(format!("{} / {:?} / {:?} has panicked", filename, op, result_tag)),
                Result::Ok(result) => {
                    let assertion_result = std::panic::catch_unwind(|| {
                        assert_eq!(
                            *result, expected_result.result,
                            "{} / {:?} / {:?} has result deviation",
                            filename, op, result_tag,
                        )
                    });
                    if assertion_result.is_err() {
                        failures.push(format!(
                            "{} / {:?} / {:?} has result deviation",
                            filename, op, result_tag
                        ));
                    }
                }
            }
        }

        if regenerate {
            let result = all_results
                .first()
                .expect("Need at least one result")
                .1
                .as_ref()
                .expect("Regeneration mode requires a valid result");
            output_features.push(update_feature(&feature, &result));
        }
    }

    if regenerate {
        write_compact_geojson(&output_features, filename);
    }

    failures
}
