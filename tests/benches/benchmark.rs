use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use std::time::Duration;

use geo::MultiPolygon;

use geo_booleanop::boolean::BooleanOp;
use geo_booleanop_tests::data_generators::{
    generate_circles_vs_rects, generate_grid_polygons, generate_random_triangles_polygons,
};
use geo_booleanop_tests::helper::load_test_case;

fn load(filename: &str) -> (MultiPolygon<f64>, MultiPolygon<f64>) {
    let (_, p1, p2) = load_test_case(filename);
    (p1, p2)
}

#[rustfmt::skip]
fn benchmarks(c: &mut Criterion) {
    let mut g = c.benchmark_group("benches");

    // small cases
    g.bench_function("hole_hole/union", |b| b.iter_batched(
        || load("fixtures/benchmarks/hole_hole.geojson"),
        |(p1, p2)| p1.union(&p2),
        BatchSize::SmallInput,
    ));

    g.bench_function("many_rects/union", |b| b.iter_batched(
        || load("fixtures/generic_test_cases/many_rects.geojson"),
        |(p1, p2)| p1.union(&p2),
        BatchSize::SmallInput,
    ));

    // medium cases
    g.sample_size(30);

    g.bench_function("state_source/union", |b| b.iter_batched(
        || load("fixtures/benchmarks/states_source.geojson"),
        |(p1, p2)| p1.union(&p2),
        BatchSize::SmallInput,
    ));

    g.bench_function("issue96/intersection", |b| b.iter_batched(
        || load("fixtures/generic_test_cases/issue96.geojson"),
        |(p1, p2)| p1.intersection(&p2),
        BatchSize::SmallInput,
    ));

    g.bench_function("issue96/union", |b| b.iter_batched(
        || load("fixtures/generic_test_cases/issue96.geojson"),
        |(p1, p2)| p1.union(&p2),
        BatchSize::SmallInput,
    ));

    g.bench_function("random_triangles/xor", |b| b.iter_batched(
        generate_random_triangles_polygons,
        |(p1, p2)| p1.xor(&p2),
        BatchSize::LargeInput,
    ));

    g.bench_function("grid/xor", |b| b.iter_batched(
        generate_grid_polygons,
        |(p1, p2)| p1.xor(&p2),
        BatchSize::LargeInput,
    ));

    // large benchmarks
    g.sample_size(10);

    g.bench_function("asia/union", |b| b.iter_batched(
        || load("fixtures/benchmarks/asia.geojson"),
        |(p1, p2)| p1.union(&p2),
        BatchSize::SmallInput,
    ));

    g.bench_function("circles_vs_rects/xor", |b| b.iter_batched(
        generate_circles_vs_rects,
        |(p1, p2)| p1.xor(&p2),
        BatchSize::LargeInput,
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
