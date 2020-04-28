//!
//! This binary allows running a test case directly by specifying its path
//! as argument.
//!
use clap::{App, AppSettings, Arg};
use geojson::Feature;

use geo_booleanop_tests::compact_geojson::write_compact_geojson;
use geo_booleanop_tests::helper::{
    apply_operation, convert_to_feature, extract_expected_result, load_test_case, plot_generic_test_case,
};

use std::fs;

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

        output_features.push(convert_to_feature(&result, Some(op)));
    }

    write_compact_geojson(&output_features, filename);
}

fn main() {
    #[rustfmt::skip]
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

    plot_generic_test_case(&filename_out);
}
