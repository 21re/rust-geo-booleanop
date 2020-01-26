extern crate geo_booleanop_tests;

use std::fs;
use std::process::Command;
use geo_booleanop_tests::helper::run_generic_test_case;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename_in = args[1].clone();
    let filename_out = filename_in.clone() + ".generated";

    fs::copy(&filename_in, &filename_out).expect("Failed to copy file.");

    run_generic_test_case(&filename_out, true);

    Command::new("../martinez/polygon_ops_debugging/plot_test_cases.py")
        .arg("-i")
        .arg(&filename_out)
        .spawn()
        .expect("Failed to run Python plot.");
}