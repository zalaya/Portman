use std::hint::black_box;

use criterion::{ BenchmarkId, Criterion, criterion_group, criterion_main };
use portman::command_line_interface::{ build_reports, evaluate };

mod fixtures;
use fixtures::{ config, fixture_items };

fn bench_build_reports(c: &mut Criterion) {
    let mut group = c.benchmark_group("report/build_reports");

    for size in [64, 5_000] {
        let items = fixture_items(size);

        group.bench_with_input(BenchmarkId::from_parameter(size), &items, |b, items| {
            b.iter(|| build_reports(black_box(items)));
        });
    }

    group.finish();
}

fn bench_evaluate(c: &mut Criterion) {
    let mut group = c.benchmark_group("report/evaluate");

    for size in [64, 5_000] {
        let items = fixture_items(size);

        group.bench_with_input(BenchmarkId::from_parameter(size), &items, |b, items| {
            b.iter(|| evaluate(black_box(items)));
        });
    }

    group.finish();
}

fn bench_json_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("report/to_json");

    for size in [64, 5_000] {
        let reports = build_reports(&fixture_items(size));

        group.bench_with_input(BenchmarkId::from_parameter(size), &reports, |b, reports| {
            b.iter(|| serde_json::to_string_pretty(black_box(reports)).unwrap());
        });
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = config();
    targets = bench_build_reports, bench_evaluate, bench_json_serialization
}
criterion_main!(benches);
