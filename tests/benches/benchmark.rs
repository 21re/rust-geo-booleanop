use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

use geo_booleanop::boolean::BooleanOp;
use geo_booleanop_tests::helper::load_test_case;

#[rustfmt::skip]
fn benchmarks(c: &mut Criterion) {

    let (_, p1, p2) = load_test_case("fixtures/generic_test_cases/issue96.geojson");
    c.bench_function("issue96 / intersection", |b| b.iter(
        || p1.intersection(&p2),
    ));
    c.bench_function("issue96 / union", |b| b.iter(
        || p1.union(&p2),
    ));

    let (_, p1, p2) = load_test_case("fixtures/generic_test_cases/many_rects.geojson");
    c.bench_function("many_rects / union", |b| b.iter(
        || p1.union(&p2),
    ));
}

fn config() -> Criterion {
    Criterion::default().measurement_time(Duration::from_millis(1))
}

criterion_group! {
    name = benches;
    config = config();
    targets = benchmarks
}
criterion_main!(benches);
