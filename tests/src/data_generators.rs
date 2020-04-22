use geo::{Coordinate, LineString, MultiPolygon, Point, Polygon};
use geojson::{Feature, FeatureCollection, GeoJson, Geometry, Value};

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use serde_json::{json, Map};

use std::iter::FromIterator;

use super::compact_geojson::write_compact_geojson;
use super::helper::xy;

fn generate_rect_centered(center: Coordinate<f64>, w: f64, h: f64) -> Polygon<f64> {
    let w_half = w / 2.0;
    let h_half = h / 2.0;
    Polygon::new(
        LineString(vec![
            xy(center.x - w_half, center.y - h_half),
            xy(center.x + w_half, center.y - h_half),
            xy(center.x + w_half, center.y + h_half),
            xy(center.x - w_half, center.y + h_half),
            xy(center.x - w_half, center.y - h_half),
        ]),
        vec![],
    )
}

fn generate_circle_ring(center: Coordinate<f64>, num_points: usize, r: f64) -> LineString<f64> {
    let mut coords = Vec::with_capacity(num_points);

    for i in 0..num_points {
        let phi = (i as f64) / (num_points as f64) * 2.0 * std::f64::consts::PI;
        coords.push(xy(center.x + r * phi.sin(), center.y + r * phi.cos()));
    }

    LineString(coords)
}

pub fn generate_grid(min: f64, max: f64, rect_size: f64, num_rects: i32) -> MultiPolygon<f64> {
    assert!(num_rects >= 2);

    let positions: Vec<_> = (0..num_rects)
        .map(|i| min + (max - min) * i as f64 / ((num_rects - 1) as f64))
        .collect();

    let mut polygons = Vec::with_capacity((num_rects * num_rects) as usize);
    for x in &positions {
        for y in &positions {
            polygons.push(generate_rect_centered(
                Coordinate { x: *x, y: *y },
                rect_size,
                rect_size,
            ));
        }
    }

    MultiPolygon(polygons)
}

pub fn generate_nested_circles(
    center: Coordinate<f64>,
    r_min: f64,
    r_max: f64,
    num_polys: usize,
    num_points: usize,
) -> MultiPolygon<f64> {
    assert!(r_max > r_min);
    assert!(r_min > 0.0);
    assert!(num_polys >= 1);
    assert!(num_points >= 3);

    let num_radii = 2 * num_polys; // We need 2*n radii in total
    let radii: Vec<_> = (0..num_radii)
        .map(|i| r_min + (i as f64) * (r_max - r_min) / ((num_radii - 1) as f64))
        .collect();

    let mut polygons = Vec::new();
    for i in (0..num_radii).step_by(2) {
        let ring1 = generate_circle_ring(center, num_points, radii[i]);
        let ring2 = generate_circle_ring(center, num_points, radii[i + 1]);
        polygons.push(Polygon::new(ring1, vec![ring2]))
    }

    MultiPolygon(polygons)
}

pub fn generate_nested_rects(
    center: Coordinate<f64>,
    width_min: f64,
    width_max: f64,
    num_polys: usize,
) -> MultiPolygon<f64> {
    assert!(width_max > width_min);
    assert!(width_min > 0.0);
    assert!(num_polys >= 1);

    let num_widths = 2 * num_polys; // We need 2*n widths in total
    let widths: Vec<_> = (0..num_widths)
        .map(|i| width_min + (i as f64) * (width_max - width_min) / ((num_widths - 1) as f64))
        .collect();

    let mut polygons = Vec::new();
    for i in (0..num_widths).step_by(2) {
        let ring1 = generate_rect_centered(center, widths[i], widths[i]).exterior().clone();
        let ring2 = generate_rect_centered(center, widths[i + 1], widths[i + 1]).exterior().clone();
        polygons.push(Polygon::new(ring1, vec![ring2]))
    }

    MultiPolygon(polygons)
}

pub fn generate_random_triangles(num_polys: usize, seed: u64) -> MultiPolygon<f64> {
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

    let mut rand_coord = || {
        Coordinate{
            x: rng.gen_range(-1.0f64, 1.0f64),
            y: rng.gen_range(-1.0f64, 1.0f64)
        }
    };

    MultiPolygon((0..num_polys).map(|_| {
        let p1 = rand_coord();
        let p2 = rand_coord();
        let p3 = rand_coord();
        Polygon::new(LineString(vec![
            p1, p2, p3, p1,
        ]), vec![])
    }).collect())
}