use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::SliceRandom;
use rand::Rng;
use rayon::prelude::*;
use mokareads_core::resources::cheatsheet::{Cheatsheet, Metadata};

// Synchronous linear search
fn sync_linear_search(data: &[Cheatsheet], target: &Cheatsheet) -> Option<Cheatsheet> {
    data.iter().find(|x| x.slug == target.slug && x.lang() == target.lang()).cloned()
}

// Parallel linear search
fn parallel_linear_search(data: &[Cheatsheet], target: &Cheatsheet) -> Option<Cheatsheet> {
    data.par_iter().find_first(|x|x.slug == target.slug && x.lang() == target.lang()).cloned()
}

fn fill_by_size(size: usize) -> Vec<Cheatsheet>{
    let one_to_find = Cheatsheet::new(Metadata::new("special", "special", 1, "special", "special"), "special".to_string());
    let mut vec = Vec::with_capacity(size);
    vec.push(one_to_find);

    for i in 1..size{
        vec.push(Cheatsheet::default())
    }

    vec
}

fn benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    // Define different dataset sizes
    let dataset_sizes = vec![10_000, 100_000, 1_000_000, 10_000_000];

    for size in dataset_sizes {
        let mut data = fill_by_size(size);

        // Shuffle the data randomly
        data.shuffle(&mut rng);

        let one_to_find = Cheatsheet::new(Metadata::new("special", "special", 1, "special", "special"), "special".to_string());

        c.bench_function(
            &format!("Sync Linear Search - Dataset Size: {}", size),
            |b| {
                b.iter(|| {
                    let result = sync_linear_search(&data, &one_to_find);
                    black_box(result); // Ensure result is not optimized out
                })
            },
        );

        c.bench_function(
            &format!("Parallel Linear Search - Dataset Size: {}", size),
            |b| {
                b.iter(|| {
                    let result = parallel_linear_search(&data, &one_to_find);
                    black_box(result); // Ensure result is not optimized out
                })
            },
        );

        c.bench_function(&format!("Sync vs Parallel - Dataset Size: {}", size), |b| {
            b.iter(|| {
                let _sync_result = sync_linear_search(&data, &one_to_find);
                let _parallel_result = parallel_linear_search(&data, &one_to_find);
                // Ensure both results are not optimized out
                black_box(&_sync_result);
                black_box(&_parallel_result);
            })
        });
    }
}
criterion_group!(benches, benchmark);
criterion_main!(benches);
