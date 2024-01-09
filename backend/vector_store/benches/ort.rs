use criterion::{criterion_group, criterion_main, Criterion};

use fastembed::{EmbeddingBase, EmbeddingModel, FlagEmbedding, InitOptions};

fn criterion_benchmark(c: &mut Criterion) {
    let model: FlagEmbedding = FlagEmbedding::try_new(InitOptions {
        model_name: EmbeddingModel::AllMiniLML6V2,
        show_download_message: false,
        ..Default::default()
    })
    .unwrap();
    c.bench_function("ortv2", |b| {
        b.iter(|| model.query_embed("Hello, World!").unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
