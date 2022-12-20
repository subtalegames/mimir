use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mimir::requirement::{RangeBound, Requirement};

fn benchmark(c: &mut Criterion) {
    let criterion = black_box(Requirement::InRange(
        RangeBound::Exclusive(5.),
        RangeBound::Inclusive(25.),
    ));

    c.bench_function("criterion evaluate", |b| {
        b.iter(|| {
            criterion.evaluate(black_box(15.));
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
