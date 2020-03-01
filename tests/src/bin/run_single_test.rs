extern crate geo_booleanop_tests;
extern crate clap;

use clap::{Arg, App, AppSettings};
use geojson::{Feature, Geometry, Value};

use geo_booleanop_tests::helper::{apply_operation, load_test_case, extract_expected_result, update_feature};
use geo_booleanop_tests::compact_geojson::write_compact_geojson;

use std::fs;
use std::path::Path;
use std::process::Command;

pub fn run_generic_test_case_with_extra_options(filename: &str, swap_ab: bool) {
    println!("\n *** Running test case: {}", filename);

    let (features, p1, p2) = load_test_case(filename);

    let (p1, p2) = if !swap_ab { (p1, p2) } else { (p2, p1) };

    let mut output_features: Vec<Feature> = if !swap_ab {
        vec![features[0].clone(), features[1].clone()]
    } else {
        vec![features[1].clone(), features[0].clone()]
    };

    for feature in features.iter().skip(2) {
        let op = extract_expected_result(&feature).op;
        println!("Testing operation: {:?}", op);

        let result = apply_operation(&p1, &p2, op);

        output_features.push(update_feature(&feature, &result));
    }

    write_compact_geojson(&output_features, filename);
}

fn main() {
    let matches = App::new("Test case runner")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("file")
                 .required(true)
                 .help("Input file"))
        .arg(Arg::with_name("swap-ab")
                 .long("swap-ab")
                 .help("Swap A/B input polygons"))
        .get_matches();

    let swap_ab = matches.is_present("swap-ab");

    let filename_in = matches.value_of("file").unwrap().to_string();
    let filename_out = filename_in.clone() + ".generated";
    fs::copy(&filename_in, &filename_out).expect("Failed to copy file.");

    run_generic_test_case_with_extra_options(&filename_out, swap_ab);

    // Try to run Python plot
    let script_path = Path::new(file!()).to_path_buf()
        .canonicalize().unwrap()
        .parent().unwrap().to_path_buf() // -> bin
        .parent().unwrap().to_path_buf() // -> src
        .parent().unwrap().to_path_buf() // -> tests
        .join("scripts")
        .join("plot_test_cases.py");
    Command::new(script_path.as_os_str())
        .arg("-i")
        .arg(&filename_out)
        .spawn()
        .expect("Failed to run Python plot.");
}
