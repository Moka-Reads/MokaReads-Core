use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::SliceRandom;
use rand::Rng;
use rayon::prelude::*;

// Synchronous linear search
fn sync_linear_search(data: &[i32], target: i32) -> Option<usize> {
    for (i, &item) in data.iter().enumerate() {
        if item == target {
            return Some(i);
        }
    }
    None
}

// Parallel linear search
fn parallel_linear_search(data: &[i32], target: i32) -> Option<usize> {
    data.par_iter().position_any(|&item| item == target)
}

fn benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    // Define different dataset sizes
    let dataset_sizes = vec![10_000, 100_000, 1_000_000, 10_000_000];

    for size in dataset_sizes {
        let mut data: Vec<i32> = (0..size).collect();

        // Shuffle the data randomly
        data.shuffle(&mut rng);

        let target = rng.gen_range(0..size);

        c.bench_function(
            &format!("Sync Linear Search - Dataset Size: {}", size),
            |b| {
                b.iter(|| {
                    let result = sync_linear_search(&data, target);
                    black_box(result); // Ensure result is not optimized out
                })
            },
        );

        c.bench_function(
            &format!("Parallel Linear Search - Dataset Size: {}", size),
            |b| {
                b.iter(|| {
                    let result = parallel_linear_search(&data, target);
                    black_box(result); // Ensure result is not optimized out
                })
            },
        );

        c.bench_function(&format!("Sync vs Parallel - Dataset Size: {}", size), |b| {
            b.iter(|| {
                let _sync_result = sync_linear_search(&data, target);
                let _parallel_result = parallel_linear_search(&data, target);
                // Ensure both results are not optimized out
                black_box(&_sync_result);
                black_box(&_parallel_result);
            })
        });
    }
}
criterion_group!(benches, benchmark);
criterion_main!(benches);
