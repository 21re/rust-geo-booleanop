use super::helper::run_generic_test_case;
use glob::glob;

#[test]
fn test_generic_test_cases() {
    let regenerate = std::env::var("REGEN").is_ok();
    let test_cases: Vec<_> = glob("./fixtures/generic_test_cases/*.geojson").expect("Failed to read glob pattern").collect();
    assert!(test_cases.len() > 0, "Expected to find any test cases");

    let mut failures = Vec::new();
    for entry in &test_cases {
        let filename = entry.as_ref().expect("Valid glob entry").to_str().unwrap().to_string();
        failures.extend(run_generic_test_case(&filename, regenerate));
    }
    println!("\nFinished running {} test cases", test_cases.len());

    if failures.len() > 0 {
        println!("\nGeneric test case failures:");
        for failure in &failures {
            println!(" - {}", failure);
        }
        panic!("Aborting due to {} failures", failures.len());
    }

    if regenerate {
        panic!(
            "Regenerate is set to true. Won't let tests pass in this mode, because it may succeed accidentally."
        );
    }
}
