//!
//! This is a tiny binary allowing to run quickly run + plot the polygon cases
//! provided by the data generators.
//!

use clap::{App, AppSettings, Arg};

use geo::MultiPolygon;

use geo_booleanop_tests::compact_geojson::write_compact_geojson;
use geo_booleanop_tests::data_generators::{
    generate_grid, generate_nested_circles, generate_nested_rects, generate_random_triangles,
};
use geo_booleanop_tests::helper::{apply_operation, convert_to_feature, plot_generic_test_case, xy, TestOperation};

fn generate_grid_polygons() -> (MultiPolygon<f64>, MultiPolygon<f64>) {
    let a = generate_grid(-10.0, 10.0, 0.4, 21);
    let b = generate_grid(-10.4, 10.4, 0.4, 21);
    (a, b)
}

fn generate_circles_vs_rects() -> (MultiPolygon<f64>, MultiPolygon<f64>) {
    let a = generate_nested_circles(xy(0, 0), 1.0, 10.0, 20, 100);
    let b = generate_nested_rects(xy(1, 1), 2.0, 20.0, 20);
    (a, b)
}

fn generate_random_triangles_polygons() -> (MultiPolygon<f64>, MultiPolygon<f64>) {
    let a = generate_random_triangles(10, 1);
    let b = generate_random_triangles(10, 2);
    (a, b)
}

fn main() {
    #[rustfmt::skip]
    let matches = App::new("Test case runner")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("case")
                 .required(true)
                 .possible_values(&["grid", "circles_vs_rects", "random_triangles"])
                 .help("Input file"))
        .get_matches();

    let case = matches.value_of("case").unwrap().to_string();
    let (a, b) = match case.as_ref() {
        "grid" => generate_grid_polygons(),
        "circles_vs_rects" => generate_circles_vs_rects(),
        "random_triangles" => generate_random_triangles_polygons(),
        _ => panic!("Illegal benchmark mode"),
    };

    let op = TestOperation::Xor;
    let result = apply_operation(&a, &b, op);

    let tmp_file = std::env::temp_dir().as_path().join(format!("{}.geoson", case));
    let tmp_file = tmp_file.to_str().expect("File name should be valid");
    write_compact_geojson(
        &[
            convert_to_feature(&a, None),
            convert_to_feature(&b, None),
            convert_to_feature(&result, Some(op)),
        ],
        tmp_file,
    );

    plot_generic_test_case(tmp_file);
}
