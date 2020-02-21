use criterion::{criterion_group, criterion_main, Criterion};

use geo_booleanop::boolean::BooleanOp;
use geo_booleanop_tests::helper::load_test_case;

#[rustfmt::skip]
fn benchmarks(c: &mut Criterion) {
    let (_, p1, p2) = load_test_case("fixtures/generic_test_cases/issue96.geojson");
    c.bench_function("issue96 - intersection", |b| b.iter(
        || p1.intersection(&p2),
    ));
    c.bench_function("issue96 - union", |b| b.iter(
        || p1.union(&p2),
    ));

    let (_, p1, p2) = load_test_case("fixtures/generic_test_cases/checkerboard1.geojson");
    c.bench_function("checkerboard1 - union", |b| b.iter(
        || p1.union(&p2),
    ));
}

criterion_group!(benches, benchmarks,);
criterion_main!(benches);
