#[cfg(not(target_os = "linux"))]
compile_error!("This benchmark supports only Linux!");

use gungraun::{library_benchmark, library_benchmark_group, main};
use obsidian_tidy_core::bench_utils::{TestData, generate_data_for_test_rules};
use obsidian_tidy_core::prelude::RulesSeed;
use serde::de::DeserializeSeed;
use serde_json::Deserializer;
use std::hint::black_box;

#[library_benchmark]
#[bench::rules_10(generate_data_for_test_rules(10))]
#[bench::rules_100(generate_data_for_test_rules(100))]
#[bench::rules_1000(generate_data_for_test_rules(1000))]
#[bench::rules_5000(generate_data_for_test_rules(5000))]
fn bench_deserialization(data: TestData) {
    let TestData { json, registry } = data;

    let seed = RulesSeed::new(&registry);
    let mut deserializer = Deserializer::from_str(black_box(&json));
    let result = seed.deserialize(&mut deserializer).unwrap();

    black_box(result);
}

library_benchmark_group!(
    name = deserialization_group;
    benchmarks =
        bench_deserialization
);

main!(library_benchmark_groups = deserialization_group);
