use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use obsidian_tidy_core::bench_utils::setup_registry;
use obsidian_tidy_core::rule::RulesSeed;
use serde::de::DeserializeSeed;
use serde_json::Deserializer;
use std::hint::black_box;

fn bench_deserialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("polymorphic_deserialization");

    for rule_count in [10, 100, 1000, 5000] {
        let json_data = obsidian_tidy_core::bench_utils::generate_benchmark_json(rule_count);
        let registry = setup_registry(rule_count);
        let seed = RulesSeed::new(&registry);

        group.throughput(Throughput::Bytes(json_data.len() as u64));

        group.bench_with_input(
            BenchmarkId::from_parameter(rule_count),
            &json_data,
            |b, data| {
                b.iter(|| {
                    let mut deserializer = Deserializer::from_str(black_box(data));
                    let result = seed.deserialize(&mut deserializer).unwrap();

                    black_box(result)
                })
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_deserialization);
criterion_main!(benches);
