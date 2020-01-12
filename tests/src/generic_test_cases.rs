use super::helper::{load_generic_test_case};
use geo::Polygon;
use geo_booleanop::boolean::BooleanOp;
use glob::glob;


#[test]
fn test_generic_test_cases() {
    for entry in glob("./fixtures/generic_test_cases/*.geojson").expect("Failed to read glob pattern") {
        load_generic_test_case(entry.expect("Valid glob entry").to_str().unwrap());
    }
    //assert_eq!(1, 2);
}
