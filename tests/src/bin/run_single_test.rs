extern crate geo_booleanop_tests;

use geo_booleanop_tests::helper::run_generic_test_case;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename_in = args[1].clone();
    let filename_out = filename_in.clone() + ".generated";

    fs::copy(&filename_in, &filename_out).expect("Failed to copy file.");

    run_generic_test_case(&filename_out, true);

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
