use criterion::{black_box, Criterion, criterion_group, criterion_main};
use rand::prelude::SliceRandom;

use mokareads_core::resources::article::{Article, Metadata};

fn sync_find(data: &[Article], target: &Article) -> Article {
    Article::find(data, &target.slug)
}

fn binary_search(data: &[Article], target: &Article) -> Article {
    Article::search(data, &target.slug)
}

fn special_article() -> Article {
    Article::new(
        Metadata::new("special", "special", "special", "special", "special"),
        "special".to_string(),
    )
}

fn fill_by_size(size: usize) -> Vec<Article> {
    let special = special_article();
    let mut vec = Vec::with_capacity(size);
    vec.push(special);

    for i in 1..size {
        vec.push(Article::default())
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

        let special = special_article();

        c.bench_function(
            &format!("Sync Find - Size {size}"),
            |b| {
                b.iter(|| {
                    let result = sync_find(&fill_by_size(size), &special);
                    black_box(result)
                })
            },
        );

        c.bench_function(
            &format!("Binary Search - Size {size}"),
            |b| {
                b.iter(|| {
                    let result = binary_search(&fill_by_size(size), &special);
                    black_box(result)
                })
            },
        );
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);