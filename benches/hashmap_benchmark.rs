use ahash::AHashMap;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use fnv::FnvHashMap;
use rustc_hash::FxHashMap;
use std::collections::HashMap;

fn create_test_data(size: usize) -> Vec<(String, i32)> {
    (0..size).map(|i| (format!("key_{}", i), i as i32)).collect()
}

fn bench_insertion(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashmap_insertion");
    let sizes = [100, 1000, 10000];

    for size in sizes.iter() {
        let data = create_test_data(*size);

        group.bench_with_input(BenchmarkId::new("std_hashmap", size), &data, |b, data| {
            b.iter(|| {
                let mut map: HashMap<String, i32> = HashMap::new();
                for (k, v) in data {
                    map.insert(k.clone(), *v);
                }
                black_box(map)
            })
        });

        group.bench_with_input(BenchmarkId::new("fx_hashmap", size), &data, |b, data| {
            b.iter(|| {
                let mut map: FxHashMap<String, i32> = FxHashMap::default();
                for (k, v) in data {
                    map.insert(k.clone(), *v);
                }
                black_box(map)
            })
        });

        group.bench_with_input(BenchmarkId::new("fnv_hashmap", size), &data, |b, data| {
            b.iter(|| {
                let mut map: FnvHashMap<String, i32> = FnvHashMap::default();
                for (k, v) in data {
                    map.insert(k.clone(), *v);
                }
                black_box(map)
            })
        });

        group.bench_with_input(BenchmarkId::new("ahash_hashmap", size), &data, |b, data| {
            b.iter(|| {
                let mut map: AHashMap<String, i32> = AHashMap::new();
                for (k, v) in data {
                    map.insert(k.clone(), *v);
                }
                black_box(map)
            })
        });
    }
    group.finish();
}

fn bench_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashmap_lookup");
    let sizes = [100, 1000, 10000];

    for size in sizes.iter() {
        let data = create_test_data(*size);

        let std_map: HashMap<_, _> = data.iter().cloned().collect();
        let fx_map: FxHashMap<_, _> = data.iter().cloned().collect();
        let fnv_map: FnvHashMap<_, _> = data.iter().cloned().collect();
        let ahash_map: AHashMap<_, _> = data.iter().cloned().collect();

        let lookup_keys: Vec<String> = data.iter().step_by(10).map(|(k, _)| k.clone()).collect();

        group.bench_with_input(BenchmarkId::new("std_hashmap", size), &lookup_keys, |b, keys| {
            b.iter(|| {
                for key in keys {
                    black_box(std_map.get(key));
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("fx_hashmap", size), &lookup_keys, |b, keys| {
            b.iter(|| {
                for key in keys {
                    black_box(fx_map.get(key));
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("fnv_hashmap", size), &lookup_keys, |b, keys| {
            b.iter(|| {
                for key in keys {
                    black_box(fnv_map.get(key));
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("ahash_hashmap", size), &lookup_keys, |b, keys| {
            b.iter(|| {
                for key in keys {
                    black_box(ahash_map.get(key));
                }
            })
        });
    }
    group.finish();
}

fn bench_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashmap_iteration");
    let sizes = [100, 1000, 10000];

    for size in sizes.iter() {
        let data = create_test_data(*size);

        let std_map: HashMap<_, _> = data.iter().cloned().collect();
        let fx_map: FxHashMap<_, _> = data.iter().cloned().collect();
        let fnv_map: FnvHashMap<_, _> = data.iter().cloned().collect();
        let ahash_map: AHashMap<_, _> = data.iter().cloned().collect();

        group.bench_with_input(BenchmarkId::new("std_hashmap", size), &size, |b, _| {
            b.iter(|| {
                for item in &std_map {
                    black_box(item);
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("fx_hashmap", size), &size, |b, _| {
            b.iter(|| {
                for item in &fx_map {
                    black_box(item);
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("fnv_hashmap", size), &size, |b, _| {
            b.iter(|| {
                for item in &fnv_map {
                    black_box(item);
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("ahash_hashmap", size), &size, |b, _| {
            b.iter(|| {
                for item in &ahash_map {
                    black_box(item);
                }
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_insertion, bench_lookup, bench_iteration);
criterion_main!(benches);
