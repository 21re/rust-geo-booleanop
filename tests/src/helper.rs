use geo_booleanop::boolean::BooleanOp;

use geo::{Coordinate, MultiPolygon, Polygon};

use geojson::{Feature, GeoJson, Geometry, Value};
use serde_json::{json, Map};

use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;

// ----------------------------------------------------------------------------
// General geo / booleanop helpers
// ----------------------------------------------------------------------------

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

pub fn apply_operation(p1: &MultiPolygon<f64>, p2: &MultiPolygon<f64>, op: TestOperation) -> MultiPolygon<f64> {
    match op {
        TestOperation::Union => p1.union(p2),
        TestOperation::Intersection => p1.intersection(p2),
        TestOperation::Xor => p1.xor(p2),
        TestOperation::DifferenceAB => p1.difference(p2),
        TestOperation::DifferenceBA => p2.difference(p1),
    }
}

// ----------------------------------------------------------------------------
// Fixture loading
// ----------------------------------------------------------------------------

fn load_fixture_from_path(path: &str) -> GeoJson {
    let mut file = File::open(path).expect("Cannot open/find fixture");
    let mut content = String::new();

    file.read_to_string(&mut content).expect("Unable to read fixture");

    content.parse::<GeoJson>().expect("Fixture is no geojson")
}

pub fn fixture_shapes(name: &str) -> (Polygon<f64>, Polygon<f64>) {
    let path = format!("./fixtures/{}", name);
    let shapes = match load_fixture_from_path(&path) {
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

// ----------------------------------------------------------------------------
// JSON <=> geo type conversion helpers
// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct ExpectedResult {
    pub result: MultiPolygon<f64>,
    pub op: TestOperation,
    pub swap_ab_is_broken: bool,
}

/// Conversion of Feature to MultiPolygon
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

/// Extended conversion of Feature to MultiPolygon, extracting additional result annotations.
pub fn extract_expected_result(feature: &Feature) -> ExpectedResult {
    let properties = feature.properties.as_ref().expect("Feature needs 'properties'.");

    let op = properties
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

    let swap_ab_is_broken = properties
        .get("swap_ab_is_broken")
        .map(|x| x.as_bool().expect("swap_ab_is_broken must be a boolean"))
        .unwrap_or(false);

    ExpectedResult {
        result: extract_multi_polygon(feature),
        op,
        swap_ab_is_broken,
    }
}

/// Conversion of MultiPolygon => Feature
pub fn convert_to_feature(p: &MultiPolygon<f64>, operation: Option<TestOperation>) -> Feature {
    Feature {
        geometry: Some(Geometry::new(Value::from(p))),
        bbox: None,
        id: None,
        properties: operation.map(
            |operation| Map::from_iter(
                std::iter::once(("operation".to_string(), json!(
                    match operation {
                        TestOperation::Union => "union",
                        TestOperation::Intersection => "intersection",
                        TestOperation::Xor => "xor",
                        TestOperation::DifferenceAB => "diff",
                        TestOperation::DifferenceBA => "diff_ba",
                    }
                )))
            )
        ),
        foreign_members: None,
    }
}
