use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use subtale_mimir::prelude::*;

fn benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    let mut group = c.benchmark_group("weighted ruleset init");

    for &num_rules in &[10, 100, 1_000, 10_000] {
        group.bench_function(format!("{} rules", num_rules), |b| {
            b.iter(|| {
                let rules: Vec<Rule<_, _, _, _>> = (0..num_rules)
                    .map(|_| {
                        let mut rule = Rule::new(true);
                        for _ in 0..rng.gen_range(0..=20) {
                            rule.insert(1, FloatEvaluator::EqualTo(rng.gen_range(0..=100) as f64));
                        }
                        rule
                    })
                    .collect();

                let _ruleset = WeightedRuleset::new(rules);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
