//!
//! This is a tiny binary allowing to run quickly run + plot the polygon cases
//! provided by the data generators.
//!

use clap::{App, AppSettings, Arg};

use geo_booleanop_tests::compact_geojson::write_compact_geojson;
use geo_booleanop_tests::data_generators::{
    generate_circles_vs_rects, generate_grid_polygons, generate_random_triangles_polygons,
};
use geo_booleanop_tests::helper::{apply_operation, convert_to_feature, plot_generic_test_case, TestOperation};

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
