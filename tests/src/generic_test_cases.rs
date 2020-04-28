use glob::glob;
use std::panic::catch_unwind;
use std::thread::Result;

use geo::MultiPolygon;

use super::compact_geojson::write_compact_geojson;
use super::helper::{apply_operation, convert_to_feature, extract_expected_result, load_test_case, TestOperation};

#[derive(Debug)]
enum ResultTag {
    MainResult,
    SwapResult,
}

type WrappedResult = (ResultTag, Result<MultiPolygon<f64>>);

fn compute_all_results(
    p1: &MultiPolygon<f64>,
    p2: &MultiPolygon<f64>,
    op: TestOperation,
    skip_swap_ab: bool,
) -> Vec<WrappedResult> {
    let main_result = catch_unwind(|| {
        println!("Running operation {:?} / {:?}", op, ResultTag::MainResult);
        apply_operation(p1, p2, op)
    });

    let mut results = vec![(ResultTag::MainResult, main_result)];
    let swappable_op = match op {
        TestOperation::DifferenceAB => false,
        TestOperation::DifferenceBA => false,
        _ => true,
    };
    if swappable_op && !skip_swap_ab {
        let swap_result = catch_unwind(|| {
            println!("Running operation {:?} / {:?}", op, ResultTag::SwapResult);
            apply_operation(p2, p1, op)
        });
        results.push((ResultTag::SwapResult, swap_result));
    }
    results
}

fn run_generic_test_case(filename: &str, regenerate: bool) -> Vec<String> {
    println!("\n *** Running test case: {}", filename);

    let (features, p1, p2) = load_test_case(filename);

    let mut output_features = vec![features[0].clone(), features[1].clone()];
    let mut failures = Vec::new();

    for feature in features.iter().skip(2) {
        let expected_result = extract_expected_result(&feature);
        let op = expected_result.op;

        let all_results = compute_all_results(&p1, &p2, op, expected_result.swap_ab_is_broken);
        for result in &all_results {
            let (result_tag, result_poly) = result;
            match &result_poly {
                Result::Err(_) => failures.push(format!("{} / {:?} / {:?} has panicked", filename, op, result_tag)),
                Result::Ok(result) => {
                    let assertion_result = std::panic::catch_unwind(|| {
                        assert_eq!(
                            *result, expected_result.result,
                            "{} / {:?} / {:?} has result deviation",
                            filename, op, result_tag,
                        )
                    });
                    if assertion_result.is_err() {
                        failures.push(format!(
                            "{} / {:?} / {:?} has result deviation",
                            filename, op, result_tag
                        ));
                    }
                }
            }
        }

        if regenerate {
            if let Result::Ok(result) = &all_results.first().expect("Need at least one result").1 {
                let mut new_feature = convert_to_feature(&result, Some(op));
                new_feature.properties = feature.properties.clone(); // Copy existing properties to keep comments etc.
                output_features.push(new_feature);
            }
        }
    }

    if regenerate {
        write_compact_geojson(&output_features, filename);
    }

    failures
}

#[test]
fn test_generic_test_cases() {
    let regenerate = std::env::var("REGEN").is_ok();
    let test_cases: Vec<_> = glob("./fixtures/generic_test_cases/*.geojson")
        .expect("Failed to read glob pattern")
        .collect();
    assert!(test_cases.len() > 0, "Expected to find any test cases");

    let mut failures = Vec::new();
    for entry in &test_cases {
        let filename = entry.as_ref().expect("Valid glob entry").to_str().unwrap().to_string();
        failures.extend(run_generic_test_case(&filename, regenerate));
    }
    println!("\nFinished running {} test cases", test_cases.len());

    if failures.len() > 0 {
        println!("\nGeneric test case failures (see error details above):");
        for failure in &failures {
            println!(" - {}", failure);
        }
        panic!("Aborting due to {} failures", failures.len());
    }

    if regenerate {
        panic!("Regenerate is set to true. Won't let tests pass in this mode, because it may succeed accidentally.");
    }
}
