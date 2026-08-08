use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use obsidian_tidy_core::bench_utils::{TestData, generate_data_for_test_rules};
use obsidian_tidy_core::rule::RulesSeed;
use serde::de::DeserializeSeed;
use serde_json::Deserializer;
use std::hint::black_box;

fn bench_deserialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("polymorphic_deserialization");

    for rule_count in [10, 100, 1000, 5000] {
        let TestData { json, registry } = generate_data_for_test_rules(rule_count);
        let seed = RulesSeed::new(&registry);

        group.throughput(Throughput::Bytes(json.len() as u64));

        group.bench_with_input(BenchmarkId::from_parameter(rule_count), &json, |b, json| {
            b.iter(|| {
                let mut deserializer = Deserializer::from_str(black_box(json));
                let result = seed.deserialize(&mut deserializer).unwrap();

                black_box(result)
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_deserialization);
criterion_main!(benches);
