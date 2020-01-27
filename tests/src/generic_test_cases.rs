use super::helper::{run_generic_test_case};
use glob::glob;


#[test]
fn test_generic_test_cases() {
    let regenerate = std::env::var("REGEN").is_ok();

    for entry in glob("./fixtures/generic_test_cases/*.geojson").expect("Failed to read glob pattern") {
        let filename = entry.expect("Valid glob entry").to_str().unwrap().to_string();
        run_generic_test_case(&filename, regenerate);
    }

    if regenerate {
        assert!(false,
            "Regenerate is set to true. Won't let tests pass in this mode, because assertions are disabled.");
    }
}
