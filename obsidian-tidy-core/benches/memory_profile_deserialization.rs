//! DHAT benchmarks for deserializer

use obsidian_tidy_core::{bench_utils::setup_registry, rule::RulesSeed};
use serde::de::DeserializeSeed;
use serde_json::Deserializer;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

const FILE_NAME: &str = "dhat-heap.json";
const RULE_CONST: usize = 100;

fn main() {
    let json_data = obsidian_tidy_core::bench_utils::generate_benchmark_json(RULE_CONST);
    let registry = setup_registry(RULE_CONST);
    let seed = RulesSeed::new(&registry);

    let _profiler = dhat::Profiler::builder().file_name(FILE_NAME).build();

    for _ in 0..10 {
        let mut deserializer = Deserializer::from_str(&json_data);
        let result = seed.deserialize(&mut deserializer).unwrap();
        std::hint::black_box(result);
    }

    drop(_profiler);

    println!("DHAT report written to `{FILE_NAME}`");
    println!("Open it at: https://nnethercote.github.io/dh_view/dh_view.html");
}
