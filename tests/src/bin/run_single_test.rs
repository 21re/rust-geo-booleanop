extern crate geo_booleanop_tests;

use geo_booleanop_tests::helper::load_generic_test_case;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    println!("test2");

    load_generic_test_case(filename);
}