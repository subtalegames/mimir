use criterion::{black_box, criterion_group, criterion_main, Criterion};
use subtale_mimir::prelude::*;

#[cfg(feature = "float")]
fn benchmark(c: &mut Criterion) {
    let evaluator = black_box(FloatEvaluator::InRange(
        FloatRangeBound::Exclusive(5.),
        FloatRangeBound::Inclusive(25.),
    ));

    c.bench_function("float_evaluator evaluate", |b| {
        b.iter(|| {
            evaluator.evaluate(black_box(15.));
        })
    });
}

#[cfg(feature = "float")]
criterion_group!(benches, benchmark);
#[cfg(feature = "float")]
criterion_main!(benches);
