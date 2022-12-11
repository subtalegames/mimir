use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    let criterion = black_box(mimir::criterion::Criterion {
        gt: 5.,
        gt_inclusive: false,
        lt: 25.,
        lt_inclusive: true,
    });

    c.bench_function("criterion evaluate", |b| {
        b.iter(|| {
            criterion.evaluate(black_box(15.));
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
