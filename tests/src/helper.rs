use geo_booleanop::boolean::BooleanOp;

use super::compact_geojson::write_compact_geojson;

use geo::{Coordinate, MultiPolygon, Polygon};
use geojson::{GeoJson, Feature, Value, Geometry};
use pretty_assertions::assert_eq;

use std::fs::File;
use std::convert::TryInto;
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
struct ExpectedResult {
    result: MultiPolygon<f64>,
    op: TestOperation,
}

fn extract_multi_polygon(feature: &Feature) -> MultiPolygon<f64> {
    let geometry_value = feature.geometry.as_ref().expect("Feature must have 'geometry' property").value.clone();
    let multi_polygon: MultiPolygon<f64> = match geometry_value {
        Value::Polygon(_) => MultiPolygon(vec![geometry_value.try_into().unwrap()]),
        Value::MultiPolygon(_) => geometry_value.try_into().unwrap(),
        _ => panic!("Feature must either be MultiPolygon or Polygon"),
    };
    multi_polygon
}

fn extract_expected_result(feature: &Feature) -> ExpectedResult {
    let multi_polygon = extract_multi_polygon(feature);

    let op = feature.properties
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

    ExpectedResult{
        result: multi_polygon,
        op: op,
    }
}

pub fn run_generic_test_case(filename: &str, regenerate: bool) {
    println!("Running test case: {}", filename);

    let original_geojson = load_fixture_from_path(filename);
    let features = match original_geojson {
        GeoJson::FeatureCollection(collection) => collection.features,
        _ => panic!("Fixture is not a feature collection"),
    };
    assert!(features.len() >= 2);
    let p1 = extract_multi_polygon(&features[0]);
    let p2 = extract_multi_polygon(&features[1]);

    let mut output_features: Vec<Feature> = vec![features[0].clone(), features[1].clone()];

    for i in 2 .. features.len() {
        let expected_result = extract_expected_result(&features[i]);

        let result = match expected_result.op {
            TestOperation::Union => p1.union(&p2),
            TestOperation::Intersection => p1.intersection(&p2),
            TestOperation::Xor => p1.xor(&p2),
            TestOperation::DifferenceAB => p1.difference(&p2),
            TestOperation::DifferenceBA => p2.difference(&p1),
        };

        if !regenerate {
            assert_eq!(result, expected_result.result);
        }

        let mut output_feature = features[i].clone();
        output_feature.geometry = Some(Geometry::new(Value::from(&result)));
        output_features.push(output_feature);
    }

    if regenerate {
        write_compact_geojson(&output_features, filename);
    }
}
