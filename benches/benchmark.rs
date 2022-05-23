use criterion::{black_box, criterion_group, criterion_main, Criterion};
use msdsmchr::render;

fn criterion_benchmark(c: &mut Criterion) {
    let img = image::open("skin.png").unwrap();

    c.bench_function("Render head", |b| b.iter(|| render::to_3d_head(&img)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
