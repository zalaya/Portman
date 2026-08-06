use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use portman::session::{SortKey, filtered_items};

mod fixtures;
use fixtures::{config, fixture_items};

const SORT_KEYS: [(&str, SortKey); 5] = [
    ("port", SortKey::Port),
    ("bind", SortKey::Bind),
    ("process", SortKey::Process),
    ("pid", SortKey::Pid),
    ("risk", SortKey::Risk),
];

fn bench_sort_keys(c: &mut Criterion) {
    let mut group = c.benchmark_group("filtered_items/sort");

    for size in [64, 5_000] {
        let items = fixture_items(size);

        for (name, sort) in SORT_KEYS {
            group.bench_with_input(BenchmarkId::new(name, size), &items, |b, items| {
                b.iter(|| filtered_items(black_box(items), black_box(""), black_box(sort)));
            });
        }
    }

    group.finish();
}

fn bench_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("filtered_items/search");
    let items = fixture_items(5_000);

    group.bench_function("narrow_match", |b| {
        b.iter(|| {
            filtered_items(
                black_box(&items),
                black_box("process-1"),
                black_box(SortKey::Port),
            )
        });
    });

    group.bench_function("no_match", |b| {
        b.iter(|| {
            filtered_items(
                black_box(&items),
                black_box("nothing-matches-this"),
                black_box(SortKey::Port),
            )
        });
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = config();
    targets = bench_sort_keys, bench_search
}
criterion_main!(benches);
