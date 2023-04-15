use criterion::{criterion_group, criterion_main, Criterion};
use subtale_mimir::prelude::*;

#[cfg(feature = "float")]
fn benchmark(c: &mut Criterion) {
    let mut query = Query::new();
    query.insert("fact_1", 1.0);
    query.insert("fact_2", 5.0);
    query.insert("fact_3", 10.0);

    let mut rule_1 = Rule::new(());
    rule_1.insert("fact_1", FloatEvaluator::lte(2.0));
    rule_1.insert("fact_2", FloatEvaluator::gte(3.0));

    let mut rule_2 = Rule::new(());
    rule_2.insert("fact_1", FloatEvaluator::gt(0.5));
    rule_2.insert("fact_2", FloatEvaluator::lt(6.0));
    rule_2.insert("fact_3", FloatEvaluator::range(9.0, 12.0));

    let ruleset = Ruleset::new(vec![rule_1, rule_2]);

    c.bench_function("ruleset evaluate", |b| b.iter(|| ruleset.evaluate(&query)));
}

#[cfg(feature = "float")]
criterion_group!(benches, benchmark);
#[cfg(feature = "float")]
criterion_main!(benches);
