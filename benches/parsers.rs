use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lipsum::lipsum_words;
use mokareads_core::bench::markdown_sample;
use mokareads_core::resources::article::Article;

fn og_parser(words: usize) -> Article {
    Article::new(&markdown_sample(&lipsum_words(words)))
}

fn yaml_parser(words: usize) -> Article {
    Article::new_yaml(&markdown_sample(&lipsum_words(words)))
}

fn parser_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parser Comparison");
    group.bench_function("OG 5000", |b| b.iter(|| og_parser(black_box(5000))));
    group.bench_function("YAML 500", |b| b.iter(|| yaml_parser(black_box(5000))));
    group.finish();

    let mut group = c.benchmark_group("Parser Comparison");
    group.bench_function("OG 10000", |b| b.iter(|| og_parser(black_box(10000))));
    group.bench_function("YAML 10000", |b| b.iter(|| yaml_parser(black_box(10000))));
    group.finish();

    let mut group = c.benchmark_group("Parser Comparison");
    group.bench_function("OG 20000", |b| b.iter(|| og_parser(black_box(20000))));
    group.bench_function("YAML 20000", |b| b.iter(|| yaml_parser(black_box(20000))));
    group.finish();
}

criterion_group!(benches, parser_benchmark);
criterion_main!(benches);
