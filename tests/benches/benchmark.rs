use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use std::time::Duration;

use geo::MultiPolygon;

use geo_booleanop::boolean::BooleanOp;
use geo_booleanop_tests::helper::load_test_case;

fn load(filename: &str) -> (MultiPolygon<f64>, MultiPolygon<f64>) {
    let (_, p1, p2) = load_test_case(filename);
    (p1, p2)
}

#[rustfmt::skip]
fn benchmarks(c: &mut Criterion) {

    c.bench_function("issue96 / intersection", |b| b.iter_batched(
        || load("fixtures/generic_test_cases/issue96.geojson"),
        |(p1, p2)| p1.intersection(&p2),
        BatchSize::SmallInput,
    ));
    c.bench_function("issue96 / union", |b| b.iter_batched(
        || load("fixtures/generic_test_cases/issue96.geojson"),
        |(p1, p2)| p1.union(&p2),
        BatchSize::SmallInput,
    ));

    c.bench_function("many_rects / union", |b| b.iter_batched(
        || load("fixtures/generic_test_cases/many_rects.geojson"),
        |(p1, p2)| p1.union(&p2),
        BatchSize::SmallInput,
    ));
}

fn config() -> Criterion {
    Criterion::default()
        .measurement_time(Duration::from_secs_f64(3.0))
        .warm_up_time(Duration::from_secs_f64(0.1))
}

criterion_group! {
    name = benches;
    config = config();
    targets = benchmarks
}
criterion_main!(benches);
