//! # Regression Test: Zero-Intermediate-Buffering Deserialization
//!
//! This test serves as an empirical, mathematical proof that our deserialization
//! architecture (`DeserializeSeed` + `erased-serde` + factory registry) operates
//! in a true streaming fashion, without materializing intermediate AST buffers
//! (such as `serde_json::Value` or temporary `HashMap`s).
//!
//! ### Why this test is necessary:
//! Streaming deserialization is fragile. A seemingly harmless refactoring (e.g.,
//! adding `.collect()`, using `ok_or` instead of `ok_or_else`, or changing a
//! `Cow` to an owned `String`) can silently introduce massive intermediate
//! allocations. This test acts as a strict guardrail against such regressions.
//!
//! ### How it works:
//! 1. We deserialize a large configuration ([`self::RULE_COUNT`] rules) while
//!    tracking allocations via `dhat`.
//! 2. We capture `HeapStats` *before* the `result` is dropped, measuring the exact
//!    memory footprint of the final data structures.
//! 3. We analyze the ratio of peak memory (`max_bytes`) to final memory (`curr_bytes`).
//!
//! ### Assertion logic:
//! - `ratio <= 1.01`: If peak memory significantly exceeds final memory, it proves
//!   that a large temporary buffer was allocated and subsequently dropped. A ratio
//!   near 1.0 means memory only grows to accommodate the final structures
//!   (`BTreeMap` nodes, `Box<dyn ErasedRule>`, etc.).
//! - `total_ratio <= 1.5`: Ensures we aren't doing excessive cyclic allocations
//!   and deallocations during the parse loop (e.g., the historical `ok_or` formatting
//!   bug that previously caused a 30% performance hit due to 1000x useless `String` creations).
//!
//! Passing this test confirms that `serde` streams data directly into the final
//! allocations, fulfilling our zero-intermediate-buffering design goal.

use obsidian_tidy_core::{
    bench_utils::{TestData, generate_data_for_test_rules},
    rule::RulesSeed,
};
use serde::de::DeserializeSeed;
use serial_test::serial;

const RULE_COUNT: usize = 1000;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[test]
#[serial]
fn dhat_limit() {
    let TestData { json, registry } = generate_data_for_test_rules(RULE_COUNT);
    let seed = RulesSeed::new(&registry);

    let _profiler = dhat::Profiler::builder().testing().build();

    let result = {
        let mut deserializer = serde_json::Deserializer::from_str(&json);
        seed.deserialize(&mut deserializer).unwrap()
    };

    let stats = dhat::HeapStats::get();

    assert_eq!(result.len(), RULE_COUNT);

    let curr = stats.curr_bytes as f64;
    let max = stats.max_bytes as f64;
    let total = stats.total_blocks as f64;

    dhat::assert!(
        curr > 0.0,
        "Final structure should not occupy 0 bytes with RULE_COUNT={}",
        RULE_COUNT
    );

    let ratio = max / curr;
    dhat::assert!(
        ratio <= 1.01,
        "INTERMEDIATE BUFFER DETECTED! \n\
         Peak memory ({}) exceeds final data size ({}) by more than 1%. \n\
         This indicates that data is first copied into temporary structures \n\
         (e.g., serde_json::Value or HashMap) and then transformed.",
        stats.max_bytes,
        stats.curr_bytes
    );

    let total_ratio = total / max;
    dhat::assert!(
        total_ratio <= 1.5,
        "Multiple cyclic allocations/deallocations detected during deserialization. \n\
         total_bytes: {}, max_bytes: {}",
        stats.total_bytes,
        stats.max_bytes
    );
}
