extern crate clap;
extern crate geo_booleanop_tests;

use clap::{App, AppSettings, Arg};
use geojson::Feature;

use geo_booleanop::boolean::BooleanOp;

use geo_booleanop_tests::compact_geojson::write_compact_geojson;
use geo_booleanop_tests::helper::{apply_operation, extract_expected_result, load_test_case, update_feature};
use geo_booleanop_tests::data_generators::{generate_grid, convert_to_feature};

use std::fs;
use std::path::Path;
use std::process::Command;


fn main() {
    /*
    #[rustfmt::skip]
    let matches = App::new("Test case runner")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("case")
                 .required(true)
                 .help("Input file"))
        .get_matches();
    */

    let grid1 = generate_grid(-10.0, 10.0, 0.4, 21);
    let grid2 = generate_grid(-10.4, 10.4, 0.4, 21);

    let xor = grid1.xor(&grid2);

    write_compact_geojson(&[
        convert_to_feature(&grid1, None),
        convert_to_feature(&grid2, None),
        convert_to_feature(&xor, Some("xor".to_string())),
    ], "newtest.geoson");

    //write_testcase(&[grid1, grid2, xor], "newtest.geoson");
}
