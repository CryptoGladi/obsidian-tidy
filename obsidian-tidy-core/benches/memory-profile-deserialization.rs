//! DHAT benchmarks for deserializer

use obsidian_tidy_core::{
    bench_utils::{TestData, generate_data_for_test_rules},
    rule::RulesSeed,
};
use serde::de::DeserializeSeed;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

const FILE_NAME: &str = "dhat-heap.json";
const RULE_COUNT: usize = 100;

fn main() {
    let TestData { json, registry } = generate_data_for_test_rules(RULE_COUNT);
    let seed = RulesSeed::new(&registry);

    let _profiler = dhat::Profiler::builder().file_name(FILE_NAME).build();

    for _ in 0..10 {
        let mut deserializer = serde_json::Deserializer::from_str(&json);
        let result = seed.deserialize(&mut deserializer).unwrap();
        std::hint::black_box(result);
    }

    drop(_profiler);

    println!("DHAT report written to `{FILE_NAME}`");
    println!("Open it at: https://nnethercote.github.io/dh_view/dh_view.html");
}
