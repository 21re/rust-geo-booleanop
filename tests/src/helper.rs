use geo::{Coordinate, MultiPolygon, Polygon};
use geojson::conversion::TryInto;
use geojson::GeoJson;
use std::fs::File;
use std::io::prelude::*;

pub fn load_fixture(name: &str) -> GeoJson {
    let mut file = File::open(format!("./fixtures/{}", name)).expect("Cannot open/find fixture");
    let mut content = String::new();

    file.read_to_string(&mut content).expect("Unable to read fixture");

    content.parse::<GeoJson>().expect("Fixture is no geojson")
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
