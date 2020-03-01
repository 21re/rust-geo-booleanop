use geo_booleanop::boolean::BooleanOp;

use super::compact_geojson::write_compact_geojson;

use geo::{Coordinate, MultiPolygon, Polygon};
use geojson::{Feature, GeoJson, Geometry, Value};
use pretty_assertions::assert_eq;

use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;

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

    let op = feature
        .properties
        .as_ref()
        .expect("Feature needs 'properties'.")
        .get("operation")
        .expect("Feature 'properties' needs an 'operation' entry.")
        .as_str()
        .expect("'operation' entry must be a string.");

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
    }
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

pub fn update_feature(feature: &Feature, p: &MultiPolygon<f64>) -> Feature {
    let mut output_feature = feature.clone();
    output_feature.geometry = Some(Geometry::new(Value::from(p)));
    output_feature
}

pub fn run_generic_test_case(filename: &str, regenerate: bool) {
    println!("\n *** Running test case: {}", filename);

    let (features, p1, p2) = load_test_case(filename);

    let mut output_features: Vec<Feature> = vec![features[0].clone(), features[1].clone()];

    for feature in features.iter().skip(2) {
        let expected_result = extract_expected_result(&feature);
        println!("Testing operation: {:?}", expected_result.op);

        let result = match expected_result.op {
            TestOperation::Union => p1.union(&p2),
            TestOperation::Intersection => p1.intersection(&p2),
            TestOperation::Xor => p1.xor(&p2),
            TestOperation::DifferenceAB => p1.difference(&p2),
            TestOperation::DifferenceBA => p2.difference(&p1),
        };

        if !regenerate {
            assert_eq!(
                result, expected_result.result,
                "Deviation found in test case {} with operation {:?}",
                filename, expected_result.op,
            );
        }

        output_features.push(update_feature(&feature, &result));
    }

    if regenerate {
        write_compact_geojson(&output_features, filename);
    }
}

/*
pub fn run_generic_test_case_new(filename: &str, regenerate: bool) {
    println!("\n *** Running test case: {}", filename);

    let (features, p1, p2) = load_test_case(filename);

    let mut output_features: Vec<Feature> = vec![features[0].clone(), features[1].clone()];

    let failures = Vec::new();

    for feature in features.iter().skip(2) {
        let expected_result = extract_expected_result(&feature);
        println!("Testing operation: {:?}", expected_result.op);

        let result = std::panic::catch_unwind(|| match expected_result.op {
            TestOperation::Union => p1.union(&p2),
            TestOperation::Intersection => p1.intersection(&p2),
            TestOperation::Xor => p1.xor(&p2),
            TestOperation::DifferenceAB => p1.difference(&p2),
            TestOperation::DifferenceBA => p2.difference(&p1),
        });

        match result {
            Result::Err(err) => failures.push(err),
            Result::Ok(result) => {
                if !regenerate {
                    let result = std::panic::catch_unwind(||
                        assert_eq!(
                            result, expected_result.result,
                            "Deviation found in test case {} with operation {:?}",
                            filename, expected_result.op,
                    ));
                    if result.is_err() {
                        println!("{:?}", result);
                        panic!("A test case has failed");
                    }
                }
            }
        }


        let mut output_feature = feature.clone();
        output_feature.geometry = Some(Geometry::new(Value::from(&result)));
        output_features.push(output_feature);
    }

    if regenerate {
        write_compact_geojson(&output_features, filename);
    }
}
*/

