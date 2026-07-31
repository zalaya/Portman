use std::net::IpAddr;

use criterion::{ BenchmarkId, Criterion, black_box, criterion_group, criterion_main };
use portman::app::{ SortKey, filtered_items };
use portman::data::network::Protocol;
use portman::data::port::PortUsage;

fn fixture_items(count: usize) -> Vec<PortUsage> {
    (0..count)
        .map(|i| PortUsage {
            port: (1024 + (i % 60_000)) as u16,
            protocol: if i % 2 == 0 { Protocol::Tcp } else { Protocol::Udp },
            pid: i as u32,
            process_name: Some(format!("process-{i}")),
            local_addr: if i % 3 == 0 { IpAddr::from([0, 0, 0, 0]) } else { IpAddr::from([127, 0, 0, 1]) },
        })
        .collect()
}

const SORT_KEYS: [(&str, SortKey); 5] =
    [("port", SortKey::Port), ("bind", SortKey::Bind), ("process", SortKey::Process), ("pid", SortKey::Pid), ("risk", SortKey::Risk)];

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
        b.iter(|| filtered_items(black_box(&items), black_box("process-1"), black_box(SortKey::Port)));
    });

    group.bench_function("no_match", |b| {
        b.iter(|| filtered_items(black_box(&items), black_box("nothing-matches-this"), black_box(SortKey::Port)));
    });

    group.finish();
}

criterion_group!(benches, bench_sort_keys, bench_search);
criterion_main!(benches);
