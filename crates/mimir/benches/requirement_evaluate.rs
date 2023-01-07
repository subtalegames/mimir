use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[cfg(feature = "float")]
use mimir::evaluator::{Evaluator, FloatEvaluator, FloatRangeBound};

#[cfg(feature = "float")]
fn benchmark(c: &mut Criterion) {
    let evaluator = black_box(FloatEvaluator::InRange(
        FloatRangeBound::Exclusive(5.),
        FloatRangeBound::Inclusive(25.),
    ));

    c.bench_function("criterion evaluate", |b| {
        b.iter(|| {
            evaluator.evaluate(black_box(15.));
        })
    });
}

#[cfg(feature = "float")]
criterion_group!(benches, benchmark);
#[cfg(feature = "float")]
criterion_main!(benches);
