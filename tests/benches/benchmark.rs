use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};

use geo_booleanop::boolean::BooleanOp;
use geo_booleanop_tests::helper::load_test_case;


fn basics(c: &mut Criterion) {
    c.bench_function("multiply", |b| b.iter_batched(
        || load_test_case("fixtures/generic_test_cases/issue96.geojson"),
        |(_, p1, p2)| p1.union(&p2),
        BatchSize::LargeInput,
    ));
}


criterion_group!(
    benches,
    basics,
);
criterion_main!(benches);
